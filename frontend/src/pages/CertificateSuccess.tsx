import LoadingSpinner from '#components/shared/LoadingSpinner.tsx';
import { Col, Divider, Row, Typography } from 'antd';
import { canisterId, createActor } from 'declarations/backend';
import { isEmpty } from 'lodash';
import { Check, Home } from 'lucide-react';
import React, { useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router';

const CertificateSuccess: React.FC = () => {

  const navigate = useNavigate();
  const backend = createActor(canisterId);
  const [nftDetails, setNftDetails] = useState<any>(null)
  const [searchParams] = useSearchParams();
  const document_id = searchParams.get('document_id');
  const onBackToHome = () => {
    navigate('/');
  }

  const getNFTDetails = async () => {
    // This function would typically fetch the NFT details based on the document_id
    // For this example, we will return static data
    const nftDetails = await backend.get_document_metadata(document_id || '');
    setNftDetails(nftDetails[0])
  }

  useEffect(() => {
    getNFTDetails();
  }, [document_id])


  if (isEmpty(nftDetails)) return <LoadingSpinner />;

  return (<div className="min-h-screen bg-gray-50 py-12" >
    <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
      <div className="bg-white rounded-lg shadow-sm p-8 text-center">
        <div className="w-16 h-16 bg-green-500 rounded-full flex items-center justify-center mx-auto mb-6">
          <Check className="w-8 h-8 text-white" />
        </div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4">
          NFT Minted Successfully
        </h2>
        <p className="text-gray-600 mb-8">
          Your certificate NFT has been successfully created and stored on the ICP blockchain.
        </p>
        <div className="bg-gray-50 rounded-lg p-6 mb-8">
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
              <Typography.Title level={4}>Blockchain Information</Typography.Title>
              <Row gutter={[16, 16]} className="text-left">
                <Col span={24}>
                  <p className="text-sm font-medium text-gray-500 mb-1">NFT ID</p>
                  <p className="text-gray-900 font-mono">{nftDetails.document_id}</p>
                </Col>
                {nftDetails.description && <Col span={24}>
                  <p className="text-sm font-medium text-gray-500 mb-1">NFT Description</p>
                  <p className="text-gray-900 font-mono">{nftDetails.description}</p>
                </Col>}

              </Row>

              <Divider orientation='center' />
              <Typography.Title level={4}>Recipient Information</Typography.Title>
              <Row gutter={[16, 16]} className="text-left">
                {nftDetails.recipient[0].name && <Col span={12}>
                  <p className="text-sm font-medium text-gray-500 mb-1">Name</p>
                  <p className="text-gray-900 font-mono text-sm break-all">
                    {nftDetails.recipient[0].name}
                  </p>
                </Col>}
                {nftDetails.recipient[0].id && <Col span={12}>
                  <p className="text-sm font-medium text-gray-500 mb-1">ID</p>
                  <p className="text-gray-900 font-mono text-sm break-all">
                    {nftDetails.recipient[0].id}
                  </p>
                </Col>}
                {nftDetails.recipient[0].email && <Col span={12}>
                  <p className="text-sm font-medium text-gray-500 mb-1">Email Address</p>
                  <p className="text-gray-900 font-mono text-sm break-all">
                    {nftDetails.recipient[0].email}
                  </p>
                </Col>}
              </Row>

            </div>
          </div>
        </div>
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          {/* <button className="inline-flex items-center px-6 py-3 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
            <Download className="w-4 h-4 mr-2" />
            Download Metadata
          </button> */}
          {/* <button onClick={() => {
            window.open(`https://www.icpexplorer.org/#/search/${nftDetails.document_hash}`, '_blank');
          }} className="inline-flex items-center px-6 py-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
            <ExternalLink className="w-4 h-4 mr-2" />
            View on Explorer
          </button> */}
          <button
            onClick={onBackToHome}
            className="inline-flex items-center px-6 py-3 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            <Home className="w-4 h-4 mr-2" />
            Back to Home
          </button>
        </div>
      </div>
    </div>
  </div >)
}

export default CertificateSuccess;
