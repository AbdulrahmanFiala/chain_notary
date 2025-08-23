import LoadingSpinner from '@/components/shared/LoadingSpinner.tsx';
import getAnalytics, { getAnalysisFocusOptions, type AnalyticsResponse } from '@/services/analytics/getAnalytics';
import { Alert, Button, Card, Col, Row, Select, Typography } from 'antd';
import { ArrowLeft, Brain, FileText, TrendingUp } from 'lucide-react';
import React, { useCallback, useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router';

const { Title, Paragraph, Text } = Typography;
const { Option } = Select;

const DocumentAnalytics: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const document_id = searchParams?.get('document_id');
  
  const [analyticsData, setAnalyticsData] = useState<AnalyticsResponse | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>('');
  const [analysisFocus, setAnalysisFocus] = useState<string>('financial_summary');
  const [focusOptions, setFocusOptions] = useState<string[]>([]);

  const onBackToDocument = () => {
    navigate(`/document-details?document_id=${document_id}`);
  };

  const loadFocusOptions = useCallback(async () => {
    try {
      const options = await getAnalysisFocusOptions();
      setFocusOptions(options);
    } catch (error) {
      console.error('Error loading focus options:', error);
      // Set default options if API fails
      setFocusOptions(['financial_summary', 'risk_assessment', 'market_insights', 'investment_analysis']);
    }
  }, []);

  const performAnalysis = useCallback(async () => {
    if (!document_id) {
      setError('No document ID provided');
      return;
    }

    setIsLoading(true);
    setError('');
    
    try {
      const response = await getAnalytics({
        document_id: document_id,
        analysis_focus: analysisFocus,
      });
      
      if (response.success) {
        setAnalyticsData(response);
      } else {
        setError(response.error_message || 'Analysis failed');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error occurred');
    } finally {
      setIsLoading(false);
    }
  }, [document_id, analysisFocus]);

  useEffect(() => {
    loadFocusOptions();
  }, [loadFocusOptions]);

  useEffect(() => {
    if (document_id && focusOptions.length > 0) {
      performAnalysis();
    }
  }, [document_id, focusOptions.length, performAnalysis]);

  const formatAnalysisText = (text: string) => {
    // Split by double newlines for paragraphs
    const paragraphs = text.split('\n\n');
    
    return paragraphs.map((paragraph, index) => {
      // Check if it's a header (starts with ##, ###, etc.)
      if (paragraph.trim().startsWith('#')) {
        const level = paragraph.match(/^#+/)?.[0].length || 1;
        const headerText = paragraph.replace(/^#+\s*/, '');
        return (
          <Title key={index} level={Math.min(level + 1, 5)} className="mt-6 mb-3">
            {headerText}
          </Title>
        );
      }
      
      // Check if it's a bullet point
      if (paragraph.trim().startsWith('•') || paragraph.trim().startsWith('-')) {
        const items = paragraph.split('\n').filter(item => item.trim());
        return (
          <ul key={index} className="mb-4 ml-4">
            {items.map((item, itemIndex) => (
              <li key={itemIndex} className="mb-1">
                {item.replace(/^[•\-]\s*/, '')}
              </li>
            ))}
          </ul>
        );
      }
      
      // Regular paragraph
      return (
        <Paragraph key={index} className="mb-4 text-gray-700">
          {paragraph}
        </Paragraph>
      );
    });
  };

  const getAnalysisIcon = (analysisType: string) => {
    switch (analysisType) {
      case 'financial_summary':
        return <TrendingUp className="w-5 h-5 text-blue-500" />;
      case 'risk_assessment':
        return <FileText className="w-5 h-5 text-red-500" />;
      case 'market_insights':
        return <Brain className="w-5 h-5 text-green-500" />;
      default:
        return <Brain className="w-5 h-5 text-purple-500" />;
    }
  };

  const getAnalysisFocusLabel = (focus: string) => {
    switch (focus) {
      case 'financial_summary':
        return 'Financial Summary';
      case 'risk_assessment':
        return 'Risk Assessment';
      case 'market_insights':
        return 'Market Insights';
      case 'investment_analysis':
        return 'Investment Analysis';
      default:
        return focus.replace('_', ' ').replace(/\b\w/g, l => l.toUpperCase());
    }
  };

  if (!document_id) {
    return (
      <div className="min-h-screen bg-gray-50 py-12">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <Alert
            message="No Document ID"
            description="Please provide a valid document ID to view analytics."
            type="error"
            showIcon
          />
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="mb-8">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <Button
                onClick={onBackToDocument}
                icon={<ArrowLeft className="w-4 h-4" />}
                className="flex items-center"
              >
                Back to Document
              </Button>
              <div>
                <Title level={2} className="mb-2">
                  <Brain className="w-8 h-8 inline-block mr-3 text-blue-600" />
                  AI Document Analytics
                </Title>
                <Text className="text-gray-600">
                  Document ID: <Text code>{document_id}</Text>
                </Text>
              </div>
            </div>
          </div>
        </div>

        {/* Analysis Focus Selector */}
        <Card className="mb-6">
          <Row gutter={[16, 16]} align="middle">
            <Col xs={24} sm={8}>
              <Text strong>Analysis Focus:</Text>
            </Col>
            <Col xs={24} sm={10}>
              <Select
                value={analysisFocus}
                onChange={(value) => setAnalysisFocus(value)}
                className="w-full"
                placeholder="Select analysis focus"
              >
                {focusOptions.map((option) => (
                  <Option key={option} value={option}>
                    {getAnalysisFocusLabel(option)}
                  </Option>
                ))}
              </Select>
            </Col>
            <Col xs={24} sm={6}>
              <Button
                type="primary"
                onClick={performAnalysis}
                loading={isLoading}
                disabled={!document_id}
                className="w-full"
              >
                {isLoading ? 'Analyzing...' : 'Analyze'}
              </Button>
            </Col>
          </Row>
        </Card>

        {/* Loading State */}
        {isLoading && (
          <Card>
            <div className="text-center py-12">
              <LoadingSpinner />
              <Paragraph className="mt-4 text-gray-600">
                AI is analyzing your document... This may take a few moments.
              </Paragraph>
            </div>
          </Card>
        )}

        {/* Error State */}
        {error && !isLoading && (
          <Alert
            message="Analysis Error"
            description={error}
            type="error"
            showIcon
            className="mb-6"
          />
        )}

        {/* Analytics Results */}
        {analyticsData && analyticsData.success && !isLoading && (
          <Card>
            <div className="mb-6">
              <div className="flex items-center space-x-3 mb-4">
                {getAnalysisIcon(analysisFocus)}
                <Title level={3} className="mb-0">
                  {getAnalysisFocusLabel(analysisFocus)} Analysis
                </Title>
              </div>
              <div className="bg-blue-50 p-3 rounded-lg">
                <Text className="text-blue-700">
                  Analysis Type: <Text strong>{analyticsData.analysis_type}</Text>
                  {analyticsData.analysis_type === 'pdf_analysis' && (
                    <div className="mt-2">
                      <Text className="text-green-600 text-sm">
                        ✓ Includes extracted PDF content + structured data
                      </Text>
                    </div>
                  )}
                  {analyticsData.analysis_type === 'data_analysis' && (
                    <div className="mt-2">
                      <Text className="text-orange-600 text-sm">
                        ⚠ Based on structured data only (PDF extraction not available)
                      </Text>
                    </div>
                  )}
                </Text>
              </div>
            </div>

            <div className="prose max-w-none">
              {formatAnalysisText(analyticsData.analysis)}
            </div>

            <div className="mt-8 pt-6 border-t border-gray-200">
              <Text className="text-sm text-gray-500">
                Analysis generated using AI. Please verify important financial decisions with professional advisors.
              </Text>
            </div>
          </Card>
        )}

        {/* Empty State */}
        {!analyticsData && !isLoading && !error && (
          <Card>
            <div className="text-center py-12">
              <Brain className="w-16 h-16 text-gray-400 mx-auto mb-4" />
              <Title level={4} className="text-gray-600">
                Ready to Analyze
              </Title>
              <Paragraph className="text-gray-500">
                Select an analysis focus and click "Analyze" to get AI insights for this document.
              </Paragraph>
            </div>
          </Card>
        )}
      </div>
    </div>
  );
};

export default DocumentAnalytics;
