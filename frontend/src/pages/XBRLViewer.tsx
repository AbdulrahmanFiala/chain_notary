import getDocumentDetails from "@/services/documents/getDocumentDetails";
import { parseXBRL, type XBRLData } from "@/utils/xbrlParser";
import {
  InboxOutlined,
  NodeIndexOutlined,
  TableOutlined,
} from "@ant-design/icons";
import {
  Button,
  Card,
  message,
  Spin,
  Table,
  Tabs,
  Tree,
  Typography,
  Upload,
} from "antd";
import type { ColumnsType } from "antd/es/table";
import type { DataNode } from "antd/es/tree";
import { useEffect, useState, type FC } from "react";
import { useNavigate, useParams } from "react-router";

const { Dragger } = Upload;
const { Title } = Typography;

interface TableData {
  key: string;
  element: string;
  value: string | number;
  context?: string;
}

const XBRLViewer: FC = () => {
  const [xbrlData, setXbrlData] = useState<XBRLData | null>(null);
  const [loading, setLoading] = useState(false);
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  useEffect(() => {
    if (id) {
      fetchDocumentData(id);
    }
  }, [id]);

  const fetchDocumentData = async (documentId: string) => {
    setLoading(true);
    try {
      const response = await getDocumentDetails(documentId);
      if (response.length > 0) {
        const document = response[0];
        if (document) {
          const fileData = new Uint8Array(document.file_data);
          const text = new TextDecoder().decode(fileData);
          const parsedData = parseXBRL(text);
          setXbrlData(parsedData);
          message.success("Document loaded successfully");
        }
      }
    } catch {
      message.error("Failed to load document");
    } finally {
      setLoading(false);
    }
  };

  const handleFileUpload = async (file: File) => {
    try {
      const text = await file.text();
      const parsedData = parseXBRL(text);
      setXbrlData(parsedData);
      message.success(`${file.name} parsed successfully`);
    } catch {
      message.error("Failed to parse XBRL file");
    }
    return false;
  };

  const getTableData = (): TableData[] => {
    if (!xbrlData) return [];

    return [
      {
        key: "1",
        element: "Company Name",
        value: xbrlData.companyName || "N/A",
      },
      { key: "2", element: "Year", value: xbrlData.year || "N/A" },
      { key: "3", element: "Quarter", value: xbrlData.quarter || "N/A" },
      {
        key: "4",
        element: "Total Assets",
        value: xbrlData.totalAssets || "N/A",
        context: "AsOf_2024-12-31",
      },
      {
        key: "5",
        element: "Total Liabilities",
        value: xbrlData.totalLiabilities || "N/A",
        context: "AsOf_2024-12-31",
      },
      {
        key: "6",
        element: "Total Equity",
        value: xbrlData.totalEquity || "N/A",
        context: "AsOf_2024-12-31",
      },
      {
        key: "7",
        element: "Gross Profit",
        value: xbrlData.grossProfit || "N/A",
        context: "Period_2024",
      },
      {
        key: "8",
        element: "Net Profit",
        value: xbrlData.netProfit || "N/A",
        context: "Period_2024",
      },
      {
        key: "9",
        element: "Operating Profit",
        value: xbrlData.operatingProfit || "N/A",
        context: "Period_2024",
      },
      {
        key: "10",
        element: "Profit Before Tax",
        value: xbrlData.profitBeforeTax || "N/A",
        context: "Period_2024",
      },
      {
        key: "11",
        element: "EBITDA",
        value: xbrlData.ebitda || "N/A",
        context: "Period_2024",
      },
    ];
  };

  const getTreeData = (): DataNode[] => {
    if (!xbrlData) return [];

    return [
      {
        title: "Company Information",
        key: "company",
        children: [
          {
            title: `Name: ${xbrlData.companyName || "N/A"}`,
            key: "company-name",
          },
          { title: `Year: ${xbrlData.year || "N/A"}`, key: "company-year" },
          {
            title: `Quarter: ${xbrlData.quarter || "N/A"}`,
            key: "company-quarter",
          },
        ],
      },
      {
        title: "Balance Sheet",
        key: "balance-sheet",
        children: [
          {
            title: `Total Assets: ${xbrlData.totalAssets || "N/A"}`,
            key: "assets",
          },
          {
            title: `Total Liabilities: ${xbrlData.totalLiabilities || "N/A"}`,
            key: "liabilities",
          },
          {
            title: `Total Equity: ${xbrlData.totalEquity || "N/A"}`,
            key: "equity",
          },
        ],
      },
      {
        title: "Income Statement",
        key: "income-statement",
        children: [
          {
            title: `Gross Profit: ${xbrlData.grossProfit || "N/A"}`,
            key: "gross-profit",
          },
          {
            title: `Operating Profit: ${xbrlData.operatingProfit || "N/A"}`,
            key: "operating-profit",
          },
          {
            title: `Profit Before Tax: ${xbrlData.profitBeforeTax || "N/A"}`,
            key: "profit-before-tax",
          },
          {
            title: `Net Profit: ${xbrlData.netProfit || "N/A"}`,
            key: "net-profit",
          },
          { title: `EBITDA: ${xbrlData.ebitda || "N/A"}`, key: "ebitda" },
        ],
      },
    ];
  };

  const columns: ColumnsType<TableData> = [
    { title: "Element", dataIndex: "element", key: "element" },
    { title: "Value", dataIndex: "value", key: "value" },
    { title: "Context", dataIndex: "context", key: "context" },
  ];

  const tabItems = [
    {
      key: "table",
      icon: <TableOutlined />,
      label: "Table View",
      children: (
        <Table
          columns={columns}
          dataSource={getTableData()}
          pagination={false}
          size="small"
        />
      ),
    },
    {
      key: "structure",
      label: "Structure View",
      icon: <NodeIndexOutlined />,
      children: <Tree treeData={getTreeData()} defaultExpandAll showLine />,
    },
  ];

  return (
    <div className="bg-gray-50 py-12">
      <div className="max-w-4xl mx-auto ">
        <Button className="mb-2" color="default" variant="outlined" onClick={() => navigate(-1)}>
          Back
        </Button>
        <Card className="px-4 sm:px-6 lg:px-8">
          <Title level={2}>XBRL File Viewer</Title>

          {loading ? (
            <div className="text-center py-8">
              <Spin size="large" />
            </div>
          ) : !xbrlData ? (
            <Dragger
              accept=".xbrl"
              beforeUpload={handleFileUpload}
              showUploadList={false}
            >
              <p className="ant-upload-drag-icon">
                <InboxOutlined />
              </p>
              <p className="ant-upload-text">Click or drag XBRL file to view</p>
              <p className="ant-upload-hint">Support for .xbrl files only</p>
            </Dragger>
          ) : (
            <Tabs items={tabItems} defaultActiveKey="table" />
          )}
        </Card>
      </div>
    </div>
  );
};

export default XBRLViewer;
