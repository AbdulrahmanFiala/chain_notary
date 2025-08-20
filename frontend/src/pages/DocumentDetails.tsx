import LoadingSpinner from '@/components/shared/LoadingSpinner.tsx';
import getDocumentDetails from '@/services/documents/getDocumentDetails';
import { Button, Col, Divider, QRCode, Row, Typography } from 'antd';
import { isEmpty } from 'lodash';
import { Check, Cross, Home } from 'lucide-react';
import React, { useCallback, useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router';

const DocumentDetails: React.FC = () => {

  const navigate = useNavigate();
  const [nftDetails, setNftDetails] = useState({
    document_id: '',
    description: '',
    recipient: [{ name: '', id: '', email: '' }]
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
      if (!isEmpty(resposeData)) setNftDetails(resposeData);
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
        <div className={`w-16 h-16 ${!isEmpty(nftDetails) ? 'bg-green-500' : 'bg-red-500'} rounded-full flex items-center justify-center mx-auto mb-6`}>

          {!isEmpty(nftDetails) ? <Check className="w-8 h-8 text-white" /> : <Cross className="w-8 h-8 text-white rotate-45" />}
        </div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4">
          {!isEmpty(nftDetails) ? "NFT Minted Successfully" : "NFT Minting Failed"}
        </h2>
        <p className="text-gray-600 mb-8">
          {!isEmpty(nftDetails) ? "Your document has been successfully minted as an NFT on the blockchain." : "There was an error minting your document as an NFT. Please try again later."}
        </p>
        {!isEmpty(nftDetails) && <div className="bg-gray-50 rounded-lg p-6 mb-8">
          <div className="text-left space-y-6">

            {/* <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Personal Information</h3>
                <div className="space-y-3">
                  <div>
                    <p className="text-sm text-gray-500">National ID Number</p>
                    <p className="text-gray-900">1234567890</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Birth Date</p>
                    <p className="text-gray-900">Jan 1, 1990</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Birthplace</p>
                    <p className="text-gray-900">Cairo, Egypt</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Nationality</p>
                    <p className="text-gray-900">Egyptian</p>
                  </div>
                </div>
              </div>
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Academic Information</h3>
                <div className="space-y-3">
                  <div>
                    <p className="text-sm text-gray-500">NFT Name</p>
                    <p className="text-gray-900">Bachelor of Engineering</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Specialization</p>
                    <p className="text-gray-900">Computer Science</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">GPA</p>
                    <p className="text-gray-900">3.8/4.0</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Graduation Project GPA</p>
                    <p className="text-gray-900">4.0/4.0</p>
                  </div>
                </div>
              </div>
            </div> */}
            <div>
              <Row gutter={[16, 16]}>
                <Col xs={{ order: 2, span: 24 }} md={{ order: 1, span: 12 }}>
                  <Typography.Title level={4} className='text-center md:text-left'>Blockchain Information</Typography.Title>
                  <Row gutter={[16, 16]}>
                    <Col span={24}>
                      <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">NFT ID</p>
                      <Typography.Paragraph copyable className="flex justify-between text-gray-900 font-mono wrap-break-word text-center md:text-left">{nftDetails.document_id}</Typography.Paragraph>
                    </Col>
                    {nftDetails.description && <Col span={24}>
                      <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">NFT Description</p>
                      <p className="text-gray-900 font-mono text-center md:text-left">{nftDetails.description}</p>
                    </Col>}
                  </Row>
                </Col>
                <Col xs={{ order: 1, span: 24 }} md={{ order: 2, span: 12 }}>
                  <div className="flex align-start md:justify-end justify-center">
                    <QRCode className="w-full" value={`${window.location.host}/certificate-success?document_id=${nftDetails.document_id}`} />
                  </div>
                </Col>
              </Row>


              <Divider orientation='center' />
              <Typography.Title level={4} className='text-center md:text-left'>Recipient Information</Typography.Title>
              <Row gutter={[16, 16]}>
                {nftDetails.recipient[0].name && <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Name</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {nftDetails.recipient[0].name}
                  </p>
                </Col>}
                {nftDetails.recipient[0].id && <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">ID</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {nftDetails.recipient[0].id}
                  </p>
                </Col>}
                {nftDetails.recipient[0].email && <Col xs={{ span: 24 }} md={{ span: 12 }}>
                  <p className="text-sm font-medium text-gray-500 mb-1 text-center md:text-left">Email Address</p>
                  <p className="text-gray-900 font-mono text-sm break-all text-center md:text-left">
                    {nftDetails.recipient[0].email}
                  </p>
                </Col>}
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
