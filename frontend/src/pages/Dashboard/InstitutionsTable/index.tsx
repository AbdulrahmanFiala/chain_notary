import createInstitution, {
  type CreateInstitutionData,
} from "@/services/institutions/createInstitution";
import getAllInstitutions from "@/services/institutions/getAllInstitutions";
import { useAppSelector } from "@/store/hooks";
import type { TableProps } from "antd";
import { Button, Form, Input, Table } from "antd";
import type { Institution } from "declarations/backend/backend.did";
import React, { useCallback, useEffect, useState } from "react";

const columns: TableProps<Institution>["columns"] = [
  {
    title: "Institution ID",
    dataIndex: "institution_id",
    key: "institution_id",
  },
  {
    title: "Name",
    dataIndex: "name",
    key: "name",
    render: (text) => <a>{text}</a>,
  },
  {
    title: "Email",
    dataIndex: "email",
    key: "email",
  },
  {
    title: "Created At",
    dataIndex: "created_at",
    key: "created_at",
  },
];

const InstitutionsTable: React.FC = () => {
  const [institutions, setInstitutions] = useState<Institution[]>([]);
  const [loading, setLoading] = useState(true);
  const [form] = Form.useForm();
  const { messageApi } = useAppSelector((state) => state.message);

  const fetchInstitutions = useCallback(async () => {
    try {
      const data = await getAllInstitutions();
      const formattedData: Institution[] = data.map((institution, index) => ({
        key: index.toString(),
        institution_id: institution.institution_id,
        name: institution.name,
        email: institution.email,
        owner: institution.owner,
        created_at: institution.created_at,
      }));
      setInstitutions(formattedData);
    } catch (error) {
      console.error("Error fetching institutions:", error);
    } finally {
      setLoading(false);
    }
  }, []);

  const handleSubmit = async (values: CreateInstitutionData) => {
    try {
      setLoading(true);
      await createInstitution(values);
      messageApi?.success("Institution created successfully!");
      form.resetFields();
      await fetchInstitutions();
    } catch {
      messageApi?.error("Failed to create institution");
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchInstitutions();
  }, [fetchInstitutions]);

  return (
    <>
      <Form
        layout="inline"
        form={form}
        onFinish={handleSubmit}
        className="py-4! bg-gray-50 my-4! rounded"
      >
        <Form.Item
          name="name"
          label="Institution Name"
          rules={[
            { required: true, message: "Please input institution name!" },
          ]}
        >
          <Input placeholder="Enter institution name" />
        </Form.Item>

        <Form.Item
          name="email"
          label="Institution Email"
          rules={[
            { required: true, message: "Please input institution email!" },
            { type: "email", message: "Please enter a valid email!" },
          ]}
        >
          <Input placeholder="Enter institution email" />
        </Form.Item>

        <Form.Item>
          <Button
            loading={loading}
            disabled={loading}
            type="primary"
            htmlType="submit"
          >
            Create Institution
          </Button>
        </Form.Item>
      </Form>
      <Table<Institution>
        bordered
        columns={columns}
        dataSource={institutions}
        loading={loading}
      />
    </>
  );
};

export default InstitutionsTable;
