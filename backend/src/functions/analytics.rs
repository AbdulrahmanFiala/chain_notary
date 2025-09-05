use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
};
use ic_cdk::{update, query};
use candid::CandidType;
use serde::{Serialize, Deserialize as SerdeDeserialize};
use serde_json::json;
use crate::types::{Document, DocumentType};
use crate::storage::{get_document_safe};
use lopdf::Document as PdfDocument;

// Configuration constants
const GEMINI_API_KEY: &str = "AIzaSyD3nnVbZJRBuonqILwrQT_8Ju9ZQA2icSY"; // TODO: Secure this in production!
const GEMINI_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent";
const MAX_RESPONSE_BYTES: u64 = 500_000; // 500KB for comprehensive analysis
const REQUEST_CYCLES: u128 = 10_000_000_000; // 10 billion cycles
const MAX_PDF_TEXT_LENGTH: usize = 50_000; // Limit PDF text to ~50K characters to avoid API limits

// Gemini API response structures
#[derive(SerdeDeserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(SerdeDeserialize, Debug)]
struct Candidate {
    content: Content,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(SerdeDeserialize, Debug)]
struct Content {
    parts: Vec<Part>,
}

#[derive(SerdeDeserialize, Debug)]
struct Part {
    text: String,
}

// Public response types for the canister
#[derive(CandidType, Serialize, SerdeDeserialize, Clone, Debug)]
pub struct AnalyticsResponse {
    pub success: bool,
    pub analysis: String,
    pub error_message: String,
    pub analysis_type: String, // "pdf_analysis" or "data_analysis"
}

#[derive(CandidType, Serialize, SerdeDeserialize, Clone, Debug)]
pub struct AnalyticsRequest {
    pub document_id: Option<String>, // If provided, analyze the PDF content
    pub input_data: Option<String>,  // If no PDF, analyze this input data
    pub analysis_focus: String,      // "financial_summary", "risk_assessment", "market_insights", etc.
}

/// Main analytics function that handles both PDF and input data analysis
#[update]
pub async fn analyze_document_data(request: AnalyticsRequest) -> AnalyticsResponse {
    // Validate request
    if request.document_id.is_none() && request.input_data.is_none() {
        return AnalyticsResponse {
            success: false,
            analysis: String::new(),
            error_message: "Either document_id or input_data must be provided".to_string(),
            analysis_type: "error".to_string(),
        };
    }

    // Determine what to analyze
    let (content_to_analyze, analysis_type) = match (&request.document_id, &request.input_data) {
        // Priority 1: If document_id is provided, try to extract PDF content
        (Some(doc_id), _) => {
            match get_document_safe(doc_id) {
                Some(document) => {
                    if document.file_type == "application/pdf" {
                        // For now, we'll analyze the document metadata and financial data
                        // In a production system, you'd want to extract actual PDF text
                        let pdf_content = extract_document_content(&document);
                        (pdf_content, "pdf_analysis".to_string())
                    } else {
                        return AnalyticsResponse {
                            success: false,
                            analysis: String::new(),
                            error_message: "Document is not a PDF file".to_string(),
                            analysis_type: "error".to_string(),
                        };
                    }
                }
                None => {
                    return AnalyticsResponse {
                        success: false,
                        analysis: String::new(),
                        error_message: "Document not found".to_string(),
                        analysis_type: "error".to_string(),
                    };
                }
            }
        }
        // Priority 2: Use input data if no document_id or document not found
        (None, Some(input)) => (input.clone(), "data_analysis".to_string()),
        (None, None) => {
            return AnalyticsResponse {
                success: false,
                analysis: String::new(),
                error_message: "No content provided for analysis".to_string(),
                analysis_type: "error".to_string(),
            };
        }
    };

    // Perform the analysis
    match perform_gemini_analysis(&content_to_analyze, &request.analysis_focus).await {
        Ok(analysis) => AnalyticsResponse {
            success: true,
            analysis,
            error_message: String::new(),
            analysis_type,
        },
        Err(error) => AnalyticsResponse {
            success: false,
            analysis: String::new(),
            error_message: error,
            analysis_type: "error".to_string(),
        },
    }
}

/// Truncate text to avoid API limits while preserving meaningful content
fn truncate_text_smartly(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        return text.to_string();
    }

    // Try to truncate at a sentence boundary
    let truncated = &text[..max_length];
    if let Some(last_period) = truncated.rfind('.') {
        if last_period > max_length / 2 {
            // If we found a period in the latter half, truncate there
            return format!("{}.\n\n[Note: Document content truncated for analysis]", &truncated[..last_period]);
        }
    }

    // Fallback: truncate at word boundary
    if let Some(last_space) = truncated.rfind(' ') {
        return format!("{}\n\n[Note: Document content truncated for analysis]", &truncated[..last_space]);
    }

    // Last resort: hard truncate
    format!("{}\n\n[Note: Document content truncated for analysis]", truncated)
}

/// Extract text content from PDF binary data using lopdf
fn extract_pdf_text(pdf_data: &[u8]) -> Result<String, String> {
    // Try to parse PDF from bytes
    let pdf_doc = match PdfDocument::load_mem(pdf_data) {
        Ok(doc) => doc,
        Err(e) => return Err(format!("Failed to parse PDF document: {:?}", e)),
    };

    let mut extracted_text = String::new();
    
    // Get all pages in the document
    let pages = pdf_doc.get_pages();
    
    if pages.is_empty() {
        return Err("PDF document contains no pages".to_string());
    }
    
    // Extract text from each page
    for (page_id, _page_obj) in pages.iter() {
        match pdf_doc.extract_text(&[*page_id]) {
            Ok(page_text) => {
                let trimmed_text = page_text.trim();
                if !trimmed_text.is_empty() {
                    extracted_text.push_str(trimmed_text);
                    extracted_text.push_str("\n\n");
                }
            }
            Err(e) => {
                // Log the error but continue with other pages
                ic_cdk::println!("Warning: Failed to extract text from page {}: {:?}", page_id, e);
                // Continue processing other pages
                continue;
            }
        }
    }
    
    let cleaned_text = extracted_text.trim();
    if cleaned_text.is_empty() {
        Err("PDF contains no extractable text content".to_string())
    } else {
        // Truncate if too long to avoid API limits
        let final_text = truncate_text_smartly(cleaned_text, MAX_PDF_TEXT_LENGTH);
        Ok(final_text)
    }
}

/// Extract content from document for analysis
fn extract_document_content(document: &Document) -> String {
    // First, try to extract actual PDF content if it's a PDF file
    let pdf_content = if document.file_type == "application/pdf" {
        match extract_pdf_text(&document.file_data) {
            Ok(text) => {
                // Successfully extracted PDF text
                Some(text)
            }
            Err(e) => {
                // PDF extraction failed, log the error but continue with metadata
                ic_cdk::println!("PDF text extraction failed: {}", e);
                None
            }
        }
    } else {
        None
    };

    match &document.document_data {
        DocumentType::EarningRelease(earning_data) => {
            let mut content = format!(
                "Financial Document Analysis for: {}\n\
                Company: {}\n\
                Description: {}\n\
                Quarter: Q{} {}\n\n",
                document.company_name,
                document.company_name,
                document.description,
                earning_data.quarter,
                earning_data.year
            );

            // Add PDF content if available
            if let Some(ref pdf_text) = pdf_content {
                content.push_str("EXTRACTED PDF CONTENT:\n");
                content.push_str("=".repeat(50).as_str());
                content.push_str("\n");
                content.push_str(pdf_text);
                content.push_str("\n");
                content.push_str("=".repeat(50).as_str());
                content.push_str("\n\n");
            }

            // Add structured financial data
            content.push_str(&format!(
                "STRUCTURED FINANCIAL METRICS:\n\
                Income Statement:\n\
                - Gross Profit: ${:.2}\n\
                - Operating Profit: ${:.2}\n\
                - EBITDA: ${:.2}\n\
                - Profit Before Tax: ${:.2}\n\
                - Net Profit: ${:.2}\n\n\
                Balance Sheet:\n\
                - Total Assets: ${:.2}\n\
                - Total Equity: ${:.2}\n\
                - Total Liabilities: ${:.2}\n\
                - Total Liabilities and Equity: ${:.2}\n\n\
                Document Metadata:\n\
                - Document Name: {}\n\
                - File Type: {}\n\
                - File Size: {} bytes\n\
                - Analysis includes: {}",
                earning_data.consolidated_income_data.gross_profit,
                earning_data.consolidated_income_data.operating_profit,
                earning_data.consolidated_income_data.ebitda,
                earning_data.consolidated_income_data.profit_before_tax,
                earning_data.consolidated_income_data.net_profit,
                earning_data.consolidated_balance_sheet_data.total_assets,
                earning_data.consolidated_balance_sheet_data.total_equity,
                earning_data.consolidated_balance_sheet_data.total_liabilities,
                earning_data.consolidated_balance_sheet_data.total_liabilities_and_equity,
                document.name,
                document.file_type,
                document.file_size,
                if pdf_content.is_some() { "PDF text content + structured data" } else { "structured data only" }
            ));

            content
        }
    }
}

/// Perform the actual Gemini API analysis
async fn perform_gemini_analysis(content: &str, focus: &str) -> Result<String, String> {
    let url = format!("{}?key={}", GEMINI_ENDPOINT, GEMINI_API_KEY);

    // Create focused prompt based on analysis type
    let prompt = create_analysis_prompt(content, focus);

    let request_body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 0.7,
            "topK": 40,
            "topP": 0.95,
            "maxOutputTokens": 2048,
        },
        "safetySettings": [
            {
                "category": "HARM_CATEGORY_HARASSMENT",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE"
            },
            {
                "category": "HARM_CATEGORY_HATE_SPEECH",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE"
            },
            {
                "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE"
            },
            {
                "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE"
            }
        ]
    })
    .to_string()
    .into_bytes();

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::POST,
        headers: vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "ChainNotary-Analytics/1.0".to_string(),
            },
        ],
        body: Some(request_body),
        max_response_bytes: Some(MAX_RESPONSE_BYTES),
        transform: None,
    };

    match http_request(request, REQUEST_CYCLES).await {
        Ok((response,)) => handle_gemini_response(response),
        Err((code, msg)) => Err(format!(
            "HTTP request failed. RejectionCode: {:?}, Message: {}",
            code, msg
        )),
    }
}

/// Create analysis prompt based on focus area
fn create_analysis_prompt(content: &str, focus: &str) -> String {
    let base_instruction = "You are a professional financial analyst with expertise in earnings releases and financial statements. You have been provided with both extracted PDF content and structured financial data.";
    
    let (focus_instruction, analysis_instructions) = match focus {
        "financial_summary" => (
            "Provide a DETAILED financial summary of the document. Focus on comprehensive analysis of financial performance, trends, and key insights.",
            "
DETAILED SUMMARY GUIDELINES:
- Provide comprehensive financial analysis (aim for 400-600 words)
- Include detailed breakdown of all major financial metrics
- Analyze revenue trends, profitability ratios, and growth rates
- Examine balance sheet strength including liquidity and solvency ratios
- Discuss cash flow patterns and working capital management
- Compare current period with previous periods if data available
- Identify key financial strengths and areas of concern
- Provide sector/industry context where relevant
- Include percentage changes and financial ratios
- Use clear headings and bullet points for organization
- Compare PDF content with structured data for accuracy
- End with detailed overall assessment and outlook (Strong/Good/Concerning/Poor)"
        ),
        "investment_insights" => (
            "Provide structured investment insights based on the financial data. Focus on investment attractiveness, growth potential, and recommendations. DO NOT use any markdown formatting like ** ** or ## in your response.",
            "
INVESTMENT ANALYSIS STRUCTURE:
Investment Highlights
- List 3-4 key strengths that make this attractive to investors

Financial Performance Analysis  
- Revenue and profitability trends
- Balance sheet strength
- Cash flow analysis

Growth Prospects
- Market opportunities
- Competitive positioning
- Future outlook

Investment Recommendation
- Clear recommendation (Strong Buy/Buy/Hold/Sell)
- Target investor profile
- Key risks to monitor

FORMATTING RULES:
- Use plain text only, no markdown formatting
- Use clear section headings without ## or **
- Use bullet points with - for lists
- Use specific numbers and percentages
- Structure with clear headings and bullet points
- Do not use bold (**text**) or italic (*text*) formatting"
        ),
        "analysis_chart" => (
            "Generate data for creating 2-3 financial visualization charts. Provide the data in a structured format that can be used to create pie charts and other visualizations.",
            "
CHART DATA REQUIREMENTS:
You must provide chart data in this EXACT JSON format within your response:

```json
{
  \"charts\": [
    {
      \"title\": \"Revenue Breakdown\",
      \"type\": \"pie\",
      \"data\": [
        {\"label\": \"Operating Revenue\", \"value\": 85, \"color\": \"#3B82F6\"},
        {\"label\": \"Other Income\", \"value\": 15, \"color\": \"#10B981\"}
      ]
    },
    {
      \"title\": \"Asset Allocation\",
      \"type\": \"pie\", 
      \"data\": [
        {\"label\": \"Current Assets\", \"value\": 60, \"color\": \"#8B5CF6\"},
        {\"label\": \"Fixed Assets\", \"value\": 40, \"color\": \"#F59E0B\"}
      ]
    }
  ]
}
```

Create 2-3 relevant charts based on the financial data. Use percentages for pie charts. Include brief analysis text explaining the charts."
        ),
        _ => (
            "Provide a balanced analysis covering financial performance, risks, and opportunities using all available data sources.",
            "
ANALYSIS GUIDELINES:
- Use both PDF content and structured data in your analysis
- Highlight any discrepancies between PDF narrative and structured data
- Provide specific numbers and percentages where available
- Structure your response with clear headings and bullet points
- Include actionable insights and recommendations"
        ),
    };

    format!(
        "{}\n\n{}\n\n{}\n\nFinancial Data to Analyze:\n{}",
        base_instruction, focus_instruction, analysis_instructions, content
    )
}

/// Handle and parse Gemini API response
fn handle_gemini_response(response: HttpResponse) -> Result<String, String> {
    if response.status != 200u32 {
        let error_body = String::from_utf8(response.body.clone())
            .unwrap_or_else(|_| "Unable to decode error response".to_string());
        return Err(format!(
            "API request failed with status {}: {}",
            response.status, error_body
        ));
    }

    let body_str = String::from_utf8(response.body)
        .map_err(|e| format!("Failed to decode response as UTF-8: {:?}", e))?;

    match serde_json::from_str::<GeminiResponse>(&body_str) {
        Ok(parsed_response) => {
            if let Some(candidate) = parsed_response.candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    if part.text.trim().is_empty() {
                        return Err("Gemini returned empty analysis".to_string());
                    }
                    return Ok(part.text.clone());
                }
            }
            Err("No analysis content found in Gemini response".to_string())
        }
        Err(e) => Err(format!("Failed to parse Gemini response: {:?}", e)),
    }
}

/// Query function to get available analysis focus options
#[query]
pub fn get_analysis_focus_options() -> Vec<String> {
    vec![
        "financial_summary".to_string(),
        "investment_insights".to_string(),
        "analysis_chart".to_string(),
    ]
}

/// Query function to check if analytics service is available
#[query]
pub fn analytics_service_status() -> String {
    "Analytics service is available. Supported analysis types: PDF documents and input data.".to_string()
}
