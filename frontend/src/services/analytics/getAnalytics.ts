import { backend } from 'declarations/backend';

export interface AnalyticsRequest {
  document_id?: string;
  input_data?: string;
  analysis_focus: string;
  api_key: string;
}

export interface AnalyticsResponse {
  success: boolean;
  analysis: string;
  error_message: string;
  analysis_type: string;
}

/**
 * Get AI analytics for a document or input data
 * @param request - Analytics request parameters
 * @returns Promise<AnalyticsResponse>
 */
const getAnalytics = async (request: AnalyticsRequest): Promise<AnalyticsResponse> => {
  try {
    const backendRequest = {
      document_id: request.document_id ? [request.document_id] : [],
      input_data: request.input_data ? [request.input_data] : [],
      analysis_focus: request.analysis_focus,
      api_key: request.api_key,
    };

    const response = await backend.analyze_document_data(backendRequest);
    
    return {
      success: response.success,
      analysis: response.analysis,
      error_message: response.error_message,
      analysis_type: response.analysis_type,
    };
  } catch (error) {
    console.error('Error fetching analytics:', error);
    return {
      success: false,
      analysis: '',
      error_message: error instanceof Error ? error.message : 'Unknown error occurred',
      analysis_type: 'error',
    };
  }
};

/**
 * Get available analysis focus options
 * @returns Promise<string[]>
 */
export const getAnalysisFocusOptions = async (): Promise<string[]> => {
  try {
    return await backend.get_analysis_focus_options();
  } catch (error) {
    console.error('Error fetching analysis focus options:', error);
    return ['financial_summary', 'risk_assessment', 'market_insights', 'investment_analysis'];
  }
};


export default getAnalytics;
