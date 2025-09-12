import DownLoadButton from '@/components/shared/DownLoadButton';
import LoadingSpinner from '@/components/shared/LoadingSpinner';
import getDocumentDetails from '@/services/documents/getDocumentDetails';
import { useAppSelector } from '@/store/hooks';
import { formatNumberWithCommas } from '@/utils/formatNumberWithCommas';
import getLabeledQuarter from '@/utils/getLabeledQuarter';
import { HomeOutlined } from "@ant-design/icons";
import { Principal } from '@dfinity/principal';
import { Button, Col, Divider, QRCode, Row, Typography } from 'antd';
import type { Document } from 'declarations/backend/backend.did';
import { isEmpty } from 'lodash';
import { Brain, Cross } from 'lucide-react';
import { useCallback, useEffect, useState, type FC } from 'react';
import { useNavigate, useParams } from 'react-router';

const DocumentDetails: FC = () => {
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const { principal } = useAppSelector((state) => state.auth)
  const [documentDetails, setDocumentDetails] = useState<Document>({
    file_data: [],
    file_size: BigInt(0),
    file_type: '',
    document_data: {
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
    }, institution_id: '', company_name: '',
    description: '',
    document_id: '',
    file_hash: '',
    document_category: { EarningRelease: null },
    publication_date: BigInt(0),
    owner: Principal.fromText(principal),
    name: ''
  });
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const { id: document_id } = useParams<{ id: string }>();

  const getNFTDetails = useCallback(async () => {
    try {
      setIsLoading(true);
      const resposeData = await getDocumentDetails(document_id || '');
      if (resposeData.length) setDocumentDetails(resposeData[0]);
    } catch (error) {
      console.error('Error fetching NFT details:', error);
    } finally {
      setIsLoading(false);
    }
  }
    , [document_id]);

  useEffect(() => {
    getNFTDetails();
  }, [getNFTDetails])


  // While router is performing a navigation (including initial loader fetch on refresh)
  // show a loading spinner. If loader completed but returned null, show an error UI.
  if (isLoading) return <LoadingSpinner />;

  if (isEmpty(documentDetails)) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <Cross className="w-12 h-12 text-red-500 mx-auto mb-4" />
          <h2 className="text-xl font-semibold">Unable to load document</h2>
          <p className="text-gray-600">We couldn't fetch the document. It may not exist or the server returned an error.</p>
          <div className="mt-4">
            <Button onClick={() => navigate(-1)}>Go back</Button>
          </div>
        </div>
      </div>
    );
  }

  return (<div className="min-h-screen bg-gray-50 py-12" >
    <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
      <div className="bg-white rounded-lg shadow-sm p-8 text-center">
        <div className={`${!documentDetails.document_id && 'bg-red-500 w-16 h-16'} rounded-full flex items-center justify-center mx-auto mb-6`}>
          {documentDetails.document_id ? <QRCode className="w-full" value={`${window.location.host}/document/${documentDetails.document_id}/view`} /> : <Cross className="w-8 h-8 text-white rotate-45" />}
        </div>
        {!documentDetails.document_id && <h2 className="text-2xl font-bold text-gray-900 mb-4">Something went wrong</h2>}
        {!documentDetails.document_id && <p className="text-gray-600 mb-8">There was an error getting your document. Please try again later.</p>}
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
                  <Typography.Paragraph className="flex justify-between text-gray-900 font-mono wrap-break-word text-center md:text-left">{getLabeledQuarter(documentDetails.document_data.EarningRelease.quarter)}</Typography.Paragraph>
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
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_income_data.ebitda)}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Gross Profit</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_income_data.gross_profit)}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Net Profit</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_income_data.net_profit)}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Operating Profit</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_income_data.operating_profit)}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Profit Before Tax</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_income_data.profit_before_tax)}
                  </p>
                </Col>
              </Row>
              <Divider orientation='center' />
              <Typography.Title level={4} className='text-center md:text-left'>Consolidated Balance Sheet</Typography.Title>
              <Row gutter={[24, 16]}>

                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Equity</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_equity)}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Liabilities and Equity</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_liabilities_and_equity)}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Assets</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_assets)}
                  </p>
                </Col>
                <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Total Liabilities</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {formatNumberWithCommas(documentDetails?.document_data.EarningRelease.consolidated_balance_sheet_data.total_liabilities)}
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
              onClick={() => navigate(`/document/${id}/analytics`)}
              color='primary'
              variant='outlined'
              className="bg-blue-600 hover:bg-blue-700"
            >
              <Brain className="w-4 h-4 mr-2" />
              AI Analytics
            </Button>
          )}
          <Button
            onClick={() => navigate('/')}
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
