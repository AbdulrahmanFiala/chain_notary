import DownLoadButton from '@/components/shared/DownLoadButton';
import LoadingSpinner from '@/components/shared/LoadingSpinner';
import getDocumentDetails from '@/services/documents/getDocumentDetails';
import getLabeledQuarer from '@/utils/getLabeledQuarer';
import { HomeOutlined } from "@ant-design/icons";
import { Principal } from '@dfinity/principal';
import { Button, Col, Divider, QRCode, Row, Typography } from 'antd';
import type { Document } from 'declarations/backend/backend.did';
import { Brain, Cross } from 'lucide-react';
import React, { useCallback, useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router';

const DocumentDetails: React.FC = () => {

  const navigate = useNavigate();
  const [documentDetails, setDocumentDetails] = useState<Document>({
    document_id: '',
    document_hash: '',
    name: '',
    description: '',
    collection_id: '',
    owner: Principal.fromText(import.meta.env.VITE_PRINCIPAL_ID),
    file_data: [],
    file_size: BigInt(0),
    file_type: '', document_data: {
      EarningRelease: {
        earning_release_id: '',
        consolidated_balance_sheet_data: {
          total_assets: 0,
          total_equity: 0,
          total_liabilities_and_equity: 0,
          total_liabilities: 0
        },
        consolidated_income_data: {
          ebitda: 0,
          gross_profit: 0,
          net_profit: 0,
          operating_profit: 0,
          profit_before_tax: 0
        },
        quarter: 0,
        year: 0
      }
    }, institution_id: '', company_name: ''
  });
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [headerData, setHeaderData] = useState<{ title?: string, message?: string }>({});
  const [searchParams] = useSearchParams();
  const document_id = searchParams?.get('document_id');
  const query_document_id = searchParams?.get('query_document_id');


  const onBackToHome = () => {
    navigate('/');
  }

  const onViewAnalytics = () => {
    navigate(`/document-analytics?document_id=${documentDetails.document_id}`);
  }

  const getNFTDetails = useCallback(async () => {
    try {
      setIsLoading(true);
      const resposeData = await getDocumentDetails(document_id || query_document_id || '');
      if (resposeData.length) setDocumentDetails(resposeData[0]);
      if (document_id) {
        setHeaderData({
          title: "Document Published Successfully",
          message: "Your document has been successfully published on the blockchain."
        })
      } else if (query_document_id) {
        setHeaderData({
          title: "Here is your document details.",
          message: "Here is your document details.",
        })
      }
    } catch (error) {
      console.error('Error fetching NFT details:', error);
      if (document_id) {
        setHeaderData({
          title: "Document Publishing Failed",
          message: "There was an error publishing your document. Please try again later.",
        })
      } else if (query_document_id) {
        setHeaderData({
          title: "Something went wrong",
          message: "There was an error getting your document. Please try again later.",
        })
      }
    } finally {
      setIsLoading(false);
    }
  }
    , [document_id, query_document_id]);

  useEffect(() => {
    getNFTDetails();
  }, [getNFTDetails])

  if (isLoading) return <LoadingSpinner />;

  return (<div className="min-h-screen bg-gray-50 py-12" >
    <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
      <div className="bg-white rounded-lg shadow-sm p-8 text-center">
        <div className={`${!documentDetails.document_id && 'bg-red-500 w-16 h-16'} rounded-full flex items-center justify-center mx-auto mb-6`}>
          {documentDetails.document_id ? <QRCode className="w-full" value={`${window.location.host}/document-details?document_id=${documentDetails.document_id}`} /> : <Cross className="w-8 h-8 text-white rotate-45" />}
        </div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4"> {headerData.title} </h2>
        <p className="text-gray-600 mb-8"> {headerData.message} </p>
        {documentDetails.document_id && <div className="bg-gray-50 rounded-lg p-6 mb-8">
          <div className="text-left space-y-6">
            <div>
              <Typography.Title level={4} className='text-center md:text-left'>Document Information</Typography.Title>
              <Row gutter={[24, 16]}>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">ID</p>
                  <Typography.Paragraph copyable className="flex justify-between text-gray-900 font-mono wrap-break-word text-center md:text-left">{documentDetails.document_id}</Typography.Paragraph>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Comapny</p>
                  <Typography.Paragraph className="flex justify-between text-gray-900 font-mono wrap-break-word text-center md:text-left">{documentDetails.company_name}</Typography.Paragraph>
                </Col>

                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Quarter</p>
                  <Typography.Paragraph className="flex justify-between text-gray-900 font-mono wrap-break-word text-center md:text-left">{getLabeledQuarer(documentDetails.document_data.EarningRelease.quarter)}</Typography.Paragraph>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Year</p>
                  <p className="text-gray-900 font-mono text-center md:text-left">{documentDetails.document_data.EarningRelease.year}</p>
                </Col>

                {documentDetails.description && <Col span={24}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Description</p>
                  <p className="text-gray-900 font-mono text-center md:text-left">{documentDetails.description}</p>
                </Col>}
              </Row>

              <Divider orientation='center' />
              <Typography.Title level={4} className='text-center md:text-left'>Consolidated Income Statement</Typography.Title>
              <Row gutter={[24, 16]}>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">EBITDA</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_income_data.ebitda}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Gross Profit</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_income_data.gross_profit}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Net Profit</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_income_data.net_profit}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Operating Profit</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_income_data.operating_profit}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Profit Before Tax</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_income_data.profit_before_tax}
                  </p>
                </Col>
              </Row>
              <Divider orientation='center' />
              <Typography.Title level={4} className='text-center md:text-left'>Consolidated Balance Sheet</Typography.Title>
              <Row gutter={[24, 16]}>

                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Equity</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_equity}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Liabilities and Equity</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_liabilities_and_equity}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Assets</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_assets}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Liabilities</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_liabilities}
                  </p>
                </Col>

              </Row>
            </div>
          </div>
        </div>}
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          {documentDetails.document_id && <DownLoadButton type='primary' file_data={documentDetails.file_data} file_type={documentDetails.file_type} file_name={documentDetails.document_id} >Download</DownLoadButton>}
          {/* <Button onClick={() => {
            window.open(`https://www.icpexplorer.org/#/search/${nftDetails.document_hash}`, '_blank');
          }} className="inline-flex items-center px-6 py-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
            <ExternalLink className="w-4 h-4 mr-2" />
            View on Explorer
          </Button> */}
          {documentDetails.document_id && (
            <Button
              onClick={onViewAnalytics}
              color='primary'
              variant='outlined'
              className="bg-blue-600 hover:bg-blue-700"
            >
              <Brain className="w-4 h-4 mr-2" />
              AI Analytics
            </Button>
          )}
          <Button
            onClick={onBackToHome}
            icon={<HomeOutlined />}
          >
            Back to Home
          </Button>
        </div>
      </div>
    </div>
  </div >)
}

export default DocumentDetails;
