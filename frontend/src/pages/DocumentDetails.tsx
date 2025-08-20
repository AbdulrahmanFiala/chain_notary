import LoadingSpinner from '@/components/shared/LoadingSpinner.tsx';
import getDocumentDetails from '@/services/documents/getDocumentDetails';
import { Principal } from '@dfinity/principal';
import { Button, Col, Divider, QRCode, Row, Typography } from 'antd';
import type { Document } from 'declarations/backend/backend.did';
import { isEmpty } from 'lodash';
import { Check, Cross, Home } from 'lucide-react';
import React, { useCallback, useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router';

const DocumentDetails: React.FC = () => {

  const navigate = useNavigate();
  const [documentDetails, setDocumentDetails] = useState<Document>({
    document_id: '',
    document_hash: [''],
    name: '',
    description: [''],
    collection_id: [],
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
    }, institution_id: []
  });
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [searchParams] = useSearchParams();
  const document_id = searchParams?.get('document_id');

  const onBackToHome = () => {
    navigate('/');
  }

  const getNFTDetails = useCallback(async () => {
    // This function would typically fetch the NFT details based on the document_id
    // For this example, we will return static data
    try {
      setIsLoading(true);
      const resposeData = await getDocumentDetails(document_id || '');
      if (resposeData.length) setDocumentDetails(resposeData[0]);
      setIsLoading(false);
    } catch (error) {
      console.error('Error fetching NFT details:', error);
      setIsLoading(false);
    }
  }, [document_id]);

  useEffect(() => {
    getNFTDetails();
  }, [getNFTDetails])


  if (isLoading) return <LoadingSpinner />;

  return (<div className="min-h-screen bg-gray-50 py-12" >
    <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
      <div className="bg-white rounded-lg shadow-sm p-8 text-center">
        <div className={`w-16 h-16 ${!isEmpty(documentDetails) ? 'bg-green-500' : 'bg-red-500'} rounded-full flex items-center justify-center mx-auto mb-6`}>

          {!isEmpty(documentDetails) ? <Check className="w-8 h-8 text-white" /> : <Cross className="w-8 h-8 text-white rotate-45" />}
        </div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4">
          {!isEmpty(documentDetails) ? "NFT Minted Successfully" : "NFT Minting Failed"}
        </h2>
        <p className="text-gray-600 mb-8">
          {!isEmpty(documentDetails) ? "Your document has been successfully minted as an NFT on the blockchain." : "There was an error minting your document as an NFT. Please try again later."}
        </p>
        {!isEmpty(documentDetails) && <div className="bg-gray-50 rounded-lg p-6 mb-8">
          <div className="text-left space-y-6">
            <div>
              <Row gutter={[16, 16]}>
                <Col xs={{ order: 2, span: 24 }} md={{ order: 1, span: 12 }}>
                  <Typography.Title level={4} className='text-center md:text-left'>Blockchain Information</Typography.Title>
                  <Row gutter={[16, 16]}>
                    <Col span={24}>
                      <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Document ID</p>
                      <Typography.Paragraph copyable className="flex justify-between text-gray-900 font-mono wrap-break-word text-center md:text-left">{documentDetails.document_id}</Typography.Paragraph>
                    </Col>
                    {documentDetails.description[0] && <Col span={24}>
                      <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Document Description</p>
                      <p className="text-gray-900 font-mono text-center md:text-left">{documentDetails.description[0]}</p>
                    </Col>}
                  </Row>
                </Col>
                <Col xs={{ order: 1, span: 24 }} md={{ order: 2, span: 12 }}>
                  <div className="flex align-start md:justify-end justify-center">
                    <QRCode className="w-full" value={`${window.location.host}/document-details?document_id=${documentDetails.document_id}`} />
                  </div>
                </Col>
              </Row>

              <Divider orientation='center' />
              <Typography.Title level={4} className='text-center md:text-left'>Consolidated Income Information</Typography.Title>
              <Row gutter={[16, 16]}>
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
              <Typography.Title level={4} className='text-center md:text-left'>Consolidated Balance Sheet Information</Typography.Title>

              <Row gutter={[16,16]}>
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
          {/* <Button className="inline-flex items-center px-6 py-3 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
            <Download className="w-4 h-4 mr-2" />
            Download Metadata
          </Button> */}
          {/* <Button onClick={() => {
            window.open(`https://www.icpexplorer.org/#/search/${nftDetails.document_hash}`, '_blank');
          }} className="inline-flex items-center px-6 py-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
            <ExternalLink className="w-4 h-4 mr-2" />
            View on Explorer
          </Button> */}
          <Button
            onClick={onBackToHome}
          >
            <Home className="w-4 h-4 mr-2" />
            Back to Home
          </Button>
        </div>
      </div>
    </div>
  </div >)
}

export default DocumentDetails;
