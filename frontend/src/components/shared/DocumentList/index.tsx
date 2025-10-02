import { DownloadOutlined, EyeOutlined } from "@ant-design/icons";
import { Button, Popover, Space, Table, Tag, Typography } from "antd";
import type { ColumnsType } from "antd/es/table";
import type { Document } from "declarations/backend/backend.did";
import { startCase } from "lodash";
import { type FC } from "react";
import DownLoadButton from "../DownLoadButton";

const { Text } = Typography;

interface DocumentListProps {
  documents: Document[];
  totalCount: number;
  loading?: boolean;
  currentPage?: number;
  pageSize?: number;
  onPageChange?: (page: number, pageSize: number) => void;
  onViewDocument?: (documentId: string) => void;
}

const DocumentList: FC<DocumentListProps> = ({
  documents,
  totalCount,
  loading = false,
  currentPage = 1,
  pageSize = 10,
  onPageChange,
  onViewDocument,
}) => {
  const formatDate = (timestamp: bigint) => {
    try {
      return new Date(Number(timestamp) / 1_000_000).toLocaleDateString(
        "en-US",
        {
          month: "short",
          day: "2-digit",
          year: "numeric",
        },
      );
    } catch {
      return "Invalid Date";
    }
  };

  const getDocumentTypeLabel = (doc: Document) => {
    if (doc.document_data.EarningRelease) {
      const data = doc.document_data.EarningRelease;
      return `Q${data.quarter} ${data.year}`;
    }
    return "Unknown";
  };

  const columns: ColumnsType<Document> = [
    {
      title: "Document ID",
      dataIndex: "document_id",
      key: "document_id",
      ellipsis: true,
      width: 200,
      render: (_, record) => (
        <Text copyable={{ text: record.document_id }}>
          {record.document_id.split("_")[1]}
        </Text>
      ),
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      ellipsis: true,
      width: 200,
      sorter: (a, b) => a.document_id.localeCompare(b.document_id),
    },
    {
      title: "Institution",
      dataIndex: "institution_id",
      key: "institution_id",
      ellipsis: true,
      width: 150,
      sorter: (a, b) => a.institution_id.localeCompare(b.institution_id),
    },
    {
      title: "Type",
      key: "type",
      render: (_, record) => (
        <Tag color="green">
          {startCase(Object.keys(record.document_data)[0])}
        </Tag>
      ),
      width: 100,
    },
    {
      title: "Period",
      key: "period",
      render: (_, record) => (
        <Tag color="blue">{getDocumentTypeLabel(record)}</Tag>
      ),
      width: 100,
    },
    {
      title: "Publication Date",
      dataIndex: "publication_date",
      key: "publication_date",
      render: (date: bigint) => formatDate(date),
      width: 120,
      sorter: (a, b) => Number(a.publication_date) - Number(b.publication_date),
    },
    {
      title: "File Size",
      dataIndex: "file_size",
      key: "file_size",
      render: (size: bigint) => `${(Number(size) / 1024 / 1024).toFixed(2)} MB`,
      width: 100,
    },
    {
      title: "Actions",
      key: "actions",
      render: (_, record) => (
        <Space size="small">
          <Popover content="View">
            <Button
              type="link"
              size="small"
              onClick={() => onViewDocument?.(record.document_id)}
            >
              <EyeOutlined />
            </Button>
          </Popover>
          <Popover content="Download">
            <DownLoadButton
              type="link"
              size="small"
              file_data={record.file_data}
              file_name={record.name}
              file_type={record.file_type}
              icon={<DownloadOutlined />}
            />
          </Popover>
        </Space>
      ),
      width: 80,
    },
  ];

  return (
    <div className="bg-white rounded-lg shadow-sm">
      <Table
        columns={columns}
        dataSource={documents}
        rowKey="document_id"
        loading={loading}
        pagination={{
          current: currentPage,
          pageSize,
          total: totalCount,
          showSizeChanger: true,
          showQuickJumper: true,
          showTotal: (total, range) =>
            `${range[0]}-${range[1]} of ${total} documents`,
          onChange: onPageChange,
          pageSizeOptions: ["10", "25", "50", "100"],
        }}
        scroll={{ x: 800 }}
        size="middle"
      />
      {documents.length === 0 && !loading && (
        <div className="text-center py-8">
          <Text type="secondary">
            No documents found matching the criteria.
          </Text>
        </div>
      )}
    </div>
  );
};

export default DocumentList;
