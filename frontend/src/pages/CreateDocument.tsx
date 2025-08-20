import Header from '@/components/Header';
import useFormValidation from '@/hooks/useFormValidation';
import type { FormData } from '@/Interfaces';
import createDocumentService from '@/services/documents/createDocument.service';
import { InboxOutlined } from '@ant-design/icons';
import { Button, Col, DatePicker, Flex, Form, Input, message, Row, Upload, type FormProps, type UploadProps } from 'antd';
import dayjs from 'dayjs';
import React, { useState } from 'react';
import { useNavigate } from 'react-router';

const CreateDocument: React.FC = () => {
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();
  const [form] = Form.useForm();
  const { isValid } = useFormValidation(form);

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
    setIsLoading(true);
    try {
      const mintedFile = await createDocumentService(formData);
      setIsLoading(false);
      navigate(`/certificate-success?document_id=${mintedFile?.document_id}`);
    } catch (error) {
      console.error("Error during file upload and document creation:", error);
      setIsLoading(false);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <>
      <Header />
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
                        disabled={(isValid && isLoading) || !isValid}
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

    </>

  );
};

export default CreateDocument;