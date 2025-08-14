import { InboxOutlined } from '@ant-design/icons';
import { Principal } from '@dfinity/principal';
import { Button, Col, DatePicker, Flex, Form, FormProps, Input, message, Row, Upload, UploadProps } from 'antd';
import dayjs from 'dayjs';
import { canisterId, createActor } from 'declarations/backend';
import React, { useState } from 'react';
import { useNavigate } from 'react-router';
import { FormData } from '../Interfaces';

const CreateCertificate: React.FC = () => {
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();
  const backend = createActor(canisterId);
  const [form] = Form.useForm()
  const props: UploadProps = {
    name: 'fileData',
    listType: 'picture',
    multiple: false,
    maxCount: 1,
    onChange(info) {
      const { status } = info.file;
      if (status === 'done') {
        form.setFieldsValue({ fileData: info.file, nftName: info.file.uid, dateOfRewarding: dayjs(info.file.lastModifiedDate) });
        message.success(`${info.file.name} file uploaded successfully.`);
      } else if (status === 'error') {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
    onRemove() {
      form.setFieldsValue({ fileData: null });

    }
  };

  const handleSubmit: FormProps<FormData>['onFinish'] = async (formData) => {
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
      const ownerPrincipal = Principal.fromText("aaaaa-aa");

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
          recipient: [{ name: formData.rewarderName || "Unknown Recipient", id: [formData.nationalId || ""], email: [formData.email || ""] }],
        }
      );
      setIsLoading(false);
      navigate(`/certificate-success?document_id=${mintedFile.document_id}`);
    } catch (error) {
      console.error("Error during file upload and document creation:", error);
      setIsLoading(false);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12">
      <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="bg-white rounded-lg shadow-sm p-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6">
            Create Certificate NFT
          </h2>

          <Form className="form space-y-6" layout='vertical' onFinish={handleSubmit} form={form}>
            <Row gutter={16}>
              <Col span={24}>
                <Form.Item
                  name="fileData"
                  hasFeedback
                  rules={[{ required: true, message: 'Please upload a document to notarize!' }]}
                >
                  <Upload.Dragger {...props}>
                    <p className="ant-upload-drag-icon">
                      <InboxOutlined />
                    </p>
                    <p className="ant-upload-text">Click or drag document to notarize it</p>
                    <p className="ant-upload-hint">
                      Support for a single upload. Strictly prohibit from uploading unpersonal data or other band files.
                    </p>
                  </Upload.Dragger>
                </Form.Item>
              </Col>
              <Col span={24}>
                <Form.Item
                  label="NFT Name"
                  name="nftName"
                  hasFeedback
                  rules={[{ required: true, message: 'Please input the NFT name!' }]}
                >
                  <Input
                    id="nftName"
                    name="nftName"
                  />
                </Form.Item>
              </Col>
              <Col span={24}>
                <Form.Item
                  label="NFT Description"
                  name="nftDescription"
                  hasFeedback
                >

                  <Input.TextArea
                    id="nftDescription"
                    name="nftDescription"
                    rows={4}
                  />
                </Form.Item>
              </Col>
              <Col xs={{ span: 24 }} md={{ span: 12 }}>
                <Form.Item
                  label="Rewarder Name"
                  name="rewarderName"
                  hasFeedback
                  rules={[{ required: true, message: 'Please input the rewarder name!' }]}
                >
                  <Input
                    id="rewarderName"
                    name="rewarderName"
                  />
                </Form.Item>
              </Col>
              <Col xs={{ span: 24 }} md={{ span: 12 }}>
                <Form.Item
                  label="Date of Rewarding"
                  name="dateOfRewarding"
                  hasFeedback
                  rules={[{ required: true, message: 'Please select the date of rewarding!' }]}
                >
                  <DatePicker
                    id="dateOfRewarding"
                    className='w-full'
                  />
                </Form.Item>
              </Col>
              <Col xs={{ span: 24 }} md={{ span: 12 }}>
                <Form.Item
                  label="Email Address"
                  name="email"
                  hasFeedback
                  rules={[{ required: true, message: 'Please input the email address!' }, { type: 'email', message: 'Please enter a valid email address!' }]}
                >
                  <Input
                    type='email'
                    id="email"
                    name="email"
                  />
                </Form.Item>
              </Col>
              <Col xs={{ span: 24 }} md={{ span: 12 }}>
                <Form.Item
                  label="National ID"
                  name="nationalId"
                  hasFeedback
                  rules={[{ required: true, message: 'Please input the national ID!' }, { pattern: /^[0-9]{8,}$/, message: 'National ID must be at least 8 digits long!' }]}
                >
                  <Input
                    id="nationalId"
                    name="nationalId"
                  />
                </Form.Item>
              </Col>
              <Col span={24}>
                <Form.Item className='mb-0!'>
                  <Flex gap="small" align="center" justify='space-between' wrap>
                    <Button
                      disabled={isLoading}
                      htmlType='reset'
                    >
                      Reset
                    </Button>
                    <Button
                      disabled={isLoading}
                      loading={isLoading}
                      variant='filled'
                      color='primary'
                      htmlType='submit'
                    >
                      {!isLoading && 'Mint NFT'}
                    </Button>
                  </Flex>
                </Form.Item>
              </Col>
            </Row>
          </Form>
        </div>
      </div>
    </div>
  );
};

export default CreateCertificate;