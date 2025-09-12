import useFormValidation from '@/hooks/useFormValidation';
import { Button, Form, Input } from 'antd';
import type { FC } from 'react';
import { useNavigate } from 'react-router';

const QueryDocument: FC = () => {

  const [form] = Form.useForm();
  const navigate = useNavigate();
  const { isValid } = useFormValidation(form);

  return (
    <div className="bg-white flex flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8 gap-6">
      <h1 className="text-center text-2xl font-bold mt-10">Query Document Page</h1>
      <p className="text-center mt-4">This page will allow users to query documents.</p>
      <Form
        className="max-w-md mx-auto mt-10 flex justify-center items-center space-y-4 flex-row gap-4 p-10"
        form={form}
        onFinish={(values: { documentId: string, query: string }) => navigate(`/document/${values.documentId}/view`)}>
        <Form.Item name="documentId" rules={[{ required: true, message: 'Please input the Document ID!' }]}>
          <Input placeholder="Enter Document ID" />
        </Form.Item>
        <Form.Item name="query">
          <Button type="primary" htmlType="submit" disabled={!isValid}>
            Query
          </Button>
        </Form.Item>
      </Form>
    </div >
  )
}

export default QueryDocument