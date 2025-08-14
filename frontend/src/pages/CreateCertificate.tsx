import { InboxOutlined } from '@ant-design/icons';
import { Principal } from '@dfinity/principal';
import { Button, DatePicker, Form, Input, message, UploadProps } from 'antd';
import Dragger from 'antd/es/upload/Dragger';
import dayjs from 'dayjs';
import { canisterId, createActor } from 'declarations/backend';
import React, { useState } from 'react';
import { useNavigate } from 'react-router';
import { FormData } from '../Interfaces';

const CreateCertificate: React.FC = () => {
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();
  const backend = createActor(canisterId);
  const props: UploadProps = {
    name: 'file',
    multiple: false,
    maxCount: 1,
    onChange(info) {
      const { status } = info.file;
      if (status === 'done') {
        setFormData(prev => ({ ...prev, fileData: info.file, nftName: info.file.uid, dateOfRewarding: dayjs(info.file.lastModifiedDate) }));
        message.success(`${info.file.name} file uploaded successfully.`);
      } else if (status === 'error') {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
    onDrop(e) {
      console.log('Dropped files', e.dataTransfer.files);
    },
    onRemove() {
      setFormData(prev => ({ ...prev, fileData: null }));
    }
  };

  const [formData, setFormData] = useState<FormData>({
    nftName: '',
    nftDescription: '',
    birthDate: null,
    issuerAddress: '',
    nationalId: '',
    rewarderName: '',
    birthplace: '',
    nationality: '',
    dateOfRewarding: null,
    issuerName: '',
    fileData: null
  });


  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: typeof (value) === 'number' ? +value : value
    }));
  };

  const handleDateChange = (name: any) => (date: dayjs.ConfigType) => {
    setFormData(prev => ({
      ...prev,
      [name]: date
    }));
  };


  const handleSubmit = async () => {
    try {
      setIsLoading(true);
      const fileObj = formData.fileData?.originFileObj;
      if (!fileObj) {
        message.error('Please upload a file.');
        return;
      }

      // Read the file as ArrayBuffer and convert to Uint8Array
      const arrayBuffer = await fileObj.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      // Calculate a document hash (example using SHA-256)
      const hashBuffer = await crypto.subtle.digest('SHA-256', uint8Array);
      const documentHash = Array.from(new Uint8Array(hashBuffer)).map(b => b.toString(16).padStart(2, '0')).join('');
      const ownerPrincipal = Principal.fromText(formData.issuerAddress || "aaaaa-aa");

      const mintedFile = await backend.upload_file_and_publish_document(
        uint8Array,
        fileObj.type,
        {
          collection_id: [],
          document_id: formData.nftName || "default_id",
          owner: ownerPrincipal,
          name: formData.nftName || "Untitled Document",
          description: [formData.nftDescription || "No description provided"],
          image_url: [],
          document_hash: documentHash,
          file_size: fileObj.size || 0,
          file_type: fileObj.type || "application/octet-stream",
          file_data: [uint8Array],
          recipient: [{ name: formData.rewarderName || "Unknown Recipient", id: ["29001001011230"], email: ["replay@test.com"] }],
        }
      );
      setIsLoading(false);
      navigate(`/certificate-success?document_id=${mintedFile.document_id}`);
    } catch (error) {
      console.error("Error during file upload and document creation:", error);
      setIsLoading(false);
    }
  };

  const handleCancel = () => {
    setFormData({
      nftName: '',
      nftDescription: '',
      birthDate: '',
      issuerAddress: '',
      nationalId: '',
      rewarderName: '',
      birthplace: '',
      nationality: '',
      dateOfRewarding: '',
      issuerName: '',
      fileData: null
    });
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12">
      <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="bg-white rounded-lg shadow-sm p-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6">
            Create Certificate NFT
          </h2>

          <Form className="form space-y-6">
            <div>
              <Dragger {...props}>
                <p className="ant-upload-drag-icon">
                  <InboxOutlined />
                </p>
                <p className="ant-upload-text">Click or drag document to notarize it</p>
                <p className="ant-upload-hint">
                </p>
              </Dragger>
            </div>
            <div>
              <label htmlFor="nftName" className="block text-sm font-medium text-gray-700 mb-2">
                NFT Name
              </label>
              <Input
                id="nftName"
                name="nftName"
                value={formData.nftName}
                onChange={handleInputChange}
                className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                required
              />
            </div>

            <div>
              <label htmlFor="nftDescription" className="block text-sm font-medium text-gray-700 mb-2">
                NFT Description
              </label>
              <Input.TextArea
                id="nftDescription"
                name="nftDescription"
                value={formData.nftDescription}
                onChange={handleInputChange}
                rows={4}
                className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                required
              />
            </div>

            <div>
              <label htmlFor="rewarderName" className="block text-sm font-medium text-gray-700 mb-2">
                Rewarder Name
              </label>
              <Input
                id="rewarderName"
                name="rewarderName"
                value={formData.rewarderName}
                onChange={handleInputChange}
                className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                required
              />
            </div>

            <div>
              <label htmlFor="dateOfRewarding" className="block text-sm font-medium text-gray-700 mb-2">
                Date of Rewarding
              </label>
              <div className="relative">
                <DatePicker
                  id="dateOfRewarding"
                  value={formData.dateOfRewarding}
                  onChange={date => handleDateChange("dateOfRewarding")(date)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  required
                />
              </div>
            </div>
          </Form>

          <div className="flex flex-col sm:flex-row gap-4 justify-center pt-6">
            <Button
              disabled={isLoading}

              onClick={handleCancel}
              className="px-8 py-3 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              Cancel
            </Button>
            <Button
              disabled={isLoading}
              loading={isLoading}
              variant='filled'
              color='primary'
              onClick={handleSubmit}
              className="px-8 py-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              Mint NFT
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default CreateCertificate;