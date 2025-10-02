import type { QueryDocumentsFilters } from "@/services/documents/queryDocuments";
import getAllInstitutions from "@/services/institutions/getAllInstitutions";
import { FilterOutlined } from "@ant-design/icons";
import { Button, Col, DatePicker, Form, Input, Row, Select } from "antd";
import type { Dayjs } from "dayjs";
import type { Institution } from "declarations/backend/backend.did";
import React, { useEffect, useState } from "react";

const { Option } = Select;
const { RangePicker } = DatePicker;

interface DocumentFilterProps {
  onFilter: (filters: QueryDocumentsFilters) => void;
  loading?: boolean;
}

interface FilterFormValues {
  doc_id?: string;
  institution_id?: string;
  document_type?: string;
  period?: Dayjs;
  dateRange?: [Dayjs, Dayjs];
  sort_by?: string;
  sort_order?: string;
  limit?: number;
}

const DocumentFilter: React.FC<DocumentFilterProps> = ({
  onFilter,
  loading = false,
}) => {
  const [form] = Form.useForm();
  const [institutions, setInstitutions] = useState<Institution[]>([]);

  useEffect(() => {
    const fetchInstitutions = async () => {
      try {
        const data = await getAllInstitutions();
        setInstitutions(data);
      } catch (error) {
        console.error("Failed to fetch institutions:", error);
      }
    };
    fetchInstitutions();
  }, []);

  const handleSubmit = (values: FilterFormValues) => {
    const filters: QueryDocumentsFilters = {};

    if (values.doc_id) filters.doc_id = values.doc_id;
    if (values.institution_id) filters.institution_id = values.institution_id;
    if (values.document_type) filters.document_type = values.document_type;
    if (values.period) {
      const year = values.period.year();
      const quarter = Math.floor(values.period.month() / 3) + 1;
      filters.year = year;
      filters.quarter = quarter;
    }

    if (values.dateRange && values.dateRange.length === 2) {
      filters.start_date = BigInt(values.dateRange[0].valueOf() * 1000000);
      filters.end_date = BigInt(values.dateRange[1].valueOf() * 1000000);
    }

    if (values.sort_by) filters.sort_by = values.sort_by;
    if (values.sort_order) filters.sort_order = values.sort_order;
    if (values.limit) filters.limit = BigInt(values.limit);

    // Always exclude file data for performance
    filters.include_file_data = true;

    onFilter(filters);
  };

  const handleReset = () => {
    form.resetFields();
    onFilter({});
  };

  return (
    <Form
      form={form}
      onFinish={handleSubmit}
      layout="vertical"
      className="bg-white p-6! rounded-lg shadow-sm"
    >
      <Row gutter={16}>
        {/* Column 1: Basic filters */}
        <Col xs={24} sm={12} md={8}>
          <Form.Item name="doc_id" label="Document ID">
            <Input placeholder="Enter document ID" />
          </Form.Item>

          <Form.Item name="institution_id" label="Institution">
            <Select placeholder="Select institution" allowClear>
              {institutions.map((inst) => (
                <Option key={inst.institution_id} value={inst.institution_id}>
                  {inst.name}
                </Option>
              ))}
            </Select>
          </Form.Item>

          <Form.Item name="document_type" label="Document Type">
            <Select placeholder="Select type" allowClear>
              <Option value="EarningRelease">Earning Release</Option>
            </Select>
          </Form.Item>
        </Col>

        {/* Column 2: Date/Time filters */}
        <Col xs={24} sm={12} md={8}>
          <Form.Item name="period" label="Period">
            <DatePicker
              placeholder="Select quarter"
              className="w-full"
              picker="quarter"
              allowClear
            />
          </Form.Item>

          <Form.Item name="dateRange" label="Publication Date Range">
            <RangePicker className="w-full" />
          </Form.Item>
        </Col>

        {/* Column 3: Sorting and actions */}
        <Col xs={24} sm={12} md={8}>
          <Form.Item name="sort_by" label="Sort By">
            <Select placeholder="Sort by" allowClear>
              <Option value="date">Date</Option>
              <Option value="name">Name</Option>
              <Option value="institution">Institution</Option>
            </Select>
          </Form.Item>

          <Form.Item name="sort_order" label="Sort Order">
            <Select placeholder="Sort order" allowClear>
              <Option value="asc">Ascending</Option>
              <Option value="desc">Descending</Option>
            </Select>
          </Form.Item>

          <Form.Item name="limit" label="Limit">
            <Select placeholder="Results per page" allowClear>
              <Option value={10}>10</Option>
              <Option value={25}>25</Option>
              <Option value={50}>50</Option>
              <Option value={100}>100</Option>
            </Select>
          </Form.Item>

          <div className="flex justify-end">
            <Button
              type="primary"
              htmlType="submit"
              loading={loading}
              icon={<FilterOutlined />}
              className="mr-2"
            >
              Apply Filters
            </Button>
            <Button disabled={loading} onClick={handleReset}>
              Reset
            </Button>
          </div>
        </Col>
      </Row>
    </Form>
  );
};

export default DocumentFilter;
