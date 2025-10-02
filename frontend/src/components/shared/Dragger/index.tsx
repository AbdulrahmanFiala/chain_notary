import { useAppSelector } from "@/store/hooks";
import computeFileHash from "@/utils/compileFileHash";
import getUint8Array from "@/utils/getUint8Array";
import { parseXBRL } from "@/utils/xbrlParser";
import { InboxOutlined } from "@ant-design/icons";
import { Upload, type FormInstance, type UploadProps } from "antd";
import type { FC } from "react";

type Props = {
  form: FormInstance;
};

const Dragger: FC<Props> = ({ form }) => {
  const { messageApi } = useAppSelector((state) => state.message);

  const props: UploadProps = {
    name: "file_data",
    listType: "picture",
    multiple: false,
    maxCount: 1,

    onChange: async (info) => {
      const { status, size, type: file_type, originFileObj } = info.file;
      if (status === "done") {
        const name = form.getFieldValue("name");
        const file_data = await getUint8Array(originFileObj as File);
        const file_hash = await computeFileHash(file_data);
        form.setFieldsValue({
          file_data,
          file_type,
          file_size: BigInt(size || 0),
          file_hash,
          name: name || info.file.name,
        });

        // Auto-fill form if XBRL file
        if (info.file.name?.toLowerCase().endsWith(".xbrl")) {
          try {
            const text = new TextDecoder().decode(file_data);
            const xbrlData = parseXBRL(text);

            const formData: Record<string, unknown> = {};
            if (xbrlData.companyName)
              formData.company_name = xbrlData.companyName;

            // Document data structure
            const earningReleaseData: Record<string, unknown> = {};
            if (xbrlData.year) earningReleaseData.year = xbrlData.year;
            if (xbrlData.quarter) earningReleaseData.quarter = xbrlData.quarter;

            // Balance sheet data
            const balanceSheetData: Record<string, number> = {};
            if (xbrlData.totalAssets)
              balanceSheetData.total_assets = xbrlData.totalAssets;
            if (xbrlData.totalLiabilities)
              balanceSheetData.total_liabilities = xbrlData.totalLiabilities;
            if (xbrlData.totalEquity)
              balanceSheetData.total_equity = xbrlData.totalEquity;
            if (xbrlData.totalLiabilitiesAndEquity)
              balanceSheetData.total_liabilities_and_equity =
                xbrlData.totalLiabilitiesAndEquity;

            if (Object.keys(balanceSheetData).length > 0) {
              earningReleaseData.consolidated_balance_sheet_data =
                balanceSheetData;
            }

            // Income data
            const incomeData: Record<string, number> = {};
            if (xbrlData.grossProfit)
              incomeData.gross_profit = xbrlData.grossProfit;
            if (xbrlData.netProfit) incomeData.net_profit = xbrlData.netProfit;
            if (xbrlData.operatingProfit)
              incomeData.operating_profit = xbrlData.operatingProfit;
            if (xbrlData.profitBeforeTax)
              incomeData.profit_before_tax = xbrlData.profitBeforeTax;
            if (xbrlData.ebitda) incomeData.ebitda = xbrlData.ebitda;

            if (Object.keys(incomeData).length > 0) {
              earningReleaseData.consolidated_income_data = incomeData;
            }

            if (Object.keys(earningReleaseData).length > 0) {
              formData.document_data = { EarningRelease: earningReleaseData };
            }

            form.setFieldsValue(formData);
            messageApi?.success(
              `${info.file.name} uploaded and form auto-filled successfully.`,
            );
          } catch (error) {
            console.error("XBRL parsing error:", error);
            messageApi?.success(
              `${info.file.name} file uploaded successfully.`,
            );
          }
        } else {
          messageApi?.success(`${info.file.name} file uploaded successfully.`);
        }
      } else if (status === "error") {
        messageApi?.error(`${info.file.name} file upload failed.`);
      }
    },
    onRemove() {
      form.setFieldsValue({
        file_data: [],
        file_type: "",
        file_size: BigInt(0),
        file_hash: "",
      });
    },
  };

  return (
    <Upload.Dragger {...props}>
      <p className="ant-upload-drag-icon">
        <InboxOutlined />
      </p>
      <p className="ant-upload-text">Click or drag document to notarize it</p>
      <p className="ant-upload-hint">
        Support for a single upload. Strictly prohibit from uploading unpersonal
        data or other band files.
      </p>
    </Upload.Dragger>
  );
};

export default Dragger;
