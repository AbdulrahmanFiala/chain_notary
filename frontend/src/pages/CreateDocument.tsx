import Header from '@/components/Header';
import useFormValidation from '@/hooks/useFormValidation';
import createDocumentService from '@/services/documents/createDocument.service';
import computeFileHash from '@/utils/compileFileHash';
import getUint8Array from '@/utils/getUint8Array';
import { InboxOutlined } from '@ant-design/icons';
import { Button, Col, Flex, Form, Input, InputNumber, message, Row, Typography, Upload, type FormProps, type UploadProps } from 'antd';
import type { Document } from 'declarations/backend/backend.did';
import React, { useState } from 'react';
import { useNavigate } from 'react-router';

const CreateDocument: React.FC = () => {
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();
  const [form] = Form.useForm<Document>();
  const { isValid } = useFormValidation(form);

  const props: UploadProps = {
    name: 'file_data',
    listType: 'picture',
    multiple: false,
    maxCount: 1,

    onChange: async (info) => {
      const { status, size, type: file_type, originFileObj } = info.file;
      if (status === 'done') {
        const file_data = await getUint8Array(originFileObj as File)
        const document_hash = await computeFileHash(file_data);
        form.setFieldsValue({ file_data, file_type, file_size: BigInt(size || 0), document_hash, name: info.file.uid });
        message.success(`${info.file.name} file uploaded successfully.`);
      } else if (status === 'error') {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
    onRemove() {
      form.setFieldsValue({ file_data: [], file_type: '', file_size: BigInt(0), document_hash: '', name: '' });
    }
  };

  const handleSubmit: FormProps<Document>['onFinish'] = async (values) => {
    setIsLoading(true);
    try {
      const mintedFile = await createDocumentService(values);
      setIsLoading(false);
      navigate(`/document-details?document_id=${mintedFile?.document_id}`);
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
      <div className="bg-gray-50 py-12">
        <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="bg-white rounded-lg shadow-sm p-8">
            <h2 className="text-2xl font-bold text-gray-900 mb-6">
              Publish Earning Release
            </h2>

            <Form className="form space-y-6" layout='vertical' onFinish={handleSubmit} form={form}>
              <Row gutter={[16, 16]} className='mb-0'>
                <Col span={24}>
                  <Form.Item
                    name="file_data"
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
                    label="Earning Release Name"
                    name="name"
                    hasFeedback
                    rules={[{ required: true, message: 'Please input the NFT name!' }]}
                  >
                    <Input />
                  </Form.Item>
                </Col>
                <Col span={24}>
                  <Form.Item
                    label="Earning Release Description"
                    name="description"
                    initialValue={''}
                    hasFeedback
                  >
                    <Input.TextArea rows={4} />
                  </Form.Item>
                </Col>
                <Col span={24}>
                  <div className='bg-gray-50 p-4! rounded-lg'>
                    <Typography.Title level={5} className='mb-4'>Earning Release Data</Typography.Title>
                    <Row gutter={[16, 16]}>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="Company Name"
                          name="company_name"
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the company name!' }]}
                        >
                          <Input />
                        </Form.Item>
                      </Col>
                        <Col xs={{ span: 24 }} md={{ span: 12 }}>
                          <Form.Item
                            label="Quarter"
                            name={['document_data', 'EarningRelease', 'quarter']}
                            hasFeedback
                            rules={[{ required: true, message: 'Please input the quarter!' }]}
                          >
                            <InputNumber className='w-full!' min={1} max={4} />
                          </Form.Item>
                        </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="Year"
                          name={['document_data', 'EarningRelease', 'year']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the year!' }]}
                        >
                          <InputNumber className='w-full!' min={2000} max={new Date().getFullYear()} />
                        </Form.Item>
                      </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="Total Equity"
                          name={['document_data', 'EarningRelease', 'consolidated_balance_sheet_data', 'total_equity']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the total equity!' }]}
                        >
                          <InputNumber className='w-full!'
                          />
                        </Form.Item>
                      </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="Total Liabilities and Equity"
                          name={['document_data', 'EarningRelease', 'consolidated_balance_sheet_data', 'total_liabilities_and_equity']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the total liabilities and equity!' }]}>
                          <InputNumber className='w-full!' />
                        </Form.Item>
                      </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="Total Assets"
                          name={['document_data', 'EarningRelease', 'consolidated_balance_sheet_data', 'total_assets']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the total assets!' }]}
                        >
                          <InputNumber className='w-full!'
                          />
                        </Form.Item></Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>

                        <Form.Item
                          label="Total Liabilities"
                          name={['document_data', 'EarningRelease', 'consolidated_balance_sheet_data', 'total_liabilities']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the total liabilities!' }]}
                        >
                          <InputNumber className='w-full!'
                          />
                        </Form.Item>
                      </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="EBITDA"
                          name={['document_data', 'EarningRelease', 'consolidated_income_data', 'ebitda']}
                          hasFeedback

                          rules={[{ required: true, message: 'Please input the EBIDA!' }]}
                        >
                          <InputNumber className='w-full!'
                          /></Form.Item></Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="Gross Profit"
                          name={['document_data', 'EarningRelease', 'consolidated_income_data', 'gross_profit']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the total equity!' }]}>
                          <InputNumber className='w-full!' />
                        </Form.Item>
                      </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>
                        <Form.Item
                          label="Net Profit"
                          name={['document_data', 'EarningRelease', 'consolidated_income_data', 'net_profit']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the net profit!' }]}
                        >
                          <InputNumber className='w-full!'
                          />
                        </Form.Item>
                      </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>

                        <Form.Item
                          label="Operating Profit"
                          name={['document_data', 'EarningRelease', 'consolidated_income_data', 'operating_profit']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the operating profit' }]}
                        >
                          <InputNumber className='w-full!'
                          />
                        </Form.Item>
                      </Col>
                      <Col xs={{ span: 24 }} md={{ span: 12 }}>

                        <Form.Item
                          label="Profit Before Tax"
                          name={['document_data', 'EarningRelease', 'consolidated_income_data', 'profit_before_tax']}
                          hasFeedback
                          rules={[{ required: true, message: 'Please input the profit before tax!' }]}
                        >
                          <InputNumber className='w-full!'
                          />
                        </Form.Item>
                      </Col>
                    </Row>
                  </div>
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

              </Row >
              <Form.Item
                name="document_id"
                hidden
                initialValue={""}
              ></Form.Item>
              <Form.Item
                name="institution_id"
                hidden
                initialValue={''}
              ></Form.Item>
              <Form.Item
                name="document_hash"
                hidden
                initialValue={''}
              ></Form.Item>
              <Form.Item
                name="file_size"
                hidden
                initialValue={BigInt(0)}
              ></Form.Item>
              <Form.Item
                name="file_type"
                hidden
                initialValue={''}
              ></Form.Item>
              <Form.Item
                name={["document_data", "EarningRelease", "earning_release_id"]}
                hidden
                initialValue={''}
              ></Form.Item>
            </Form >
          </div >
        </div >
      </div >

    </>

  );
};

export default CreateDocument;