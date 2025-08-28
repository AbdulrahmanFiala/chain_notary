import { mimeToExtension } from '@/constants';
import { DownloadOutlined } from '@ant-design/icons';
import { Button } from 'antd';
import type { BaseButtonProps } from 'antd/es/button/button';
import { type FC } from 'react';

type Props = {
  file_data: Uint8Array | number[],
  file_type: string,
  file_name: string
} & BaseButtonProps

const DownLoadButton: FC<Props> = (props) => {
  const { file_data, file_type, file_name, children } = props;

  const handleDownload = () => {
    if (!file_data || file_data.length === 0) {
      console.error("No data provided for download.");
      return;
    }

    try {
      // Infer extension if not given
      let finalFileName = file_name;
      if (!finalFileName) {
        const ext = mimeToExtension[file_type] || "bin";
        finalFileName = `download.${ext}`;
      } else if (!finalFileName.includes(".")) {
        const ext = mimeToExtension[file_type] || "bin";
        finalFileName = `${finalFileName}.${ext}`;
      }

      // Convert byte array to a Blob
      const blob = new Blob([new Uint8Array(file_data)], { type: file_type });

      // Create temporary object URL
      const url = window.URL.createObjectURL(blob);

      // Create anchor element & trigger download
      const a = document.createElement("a");
      a.href = url;
      a.download = finalFileName;
      a.style.display = "none";
      document.body.appendChild(a);
      a.click();

      // Cleanup
      document.body.removeChild(a);
      window.URL.revokeObjectURL(url);
    } catch (err) {
      console.error("Download failed:", err);
    }
  };

  return (
    <Button className="inline-flex items-center px-6 py-3 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500" {...props} onClick={handleDownload} icon={<DownloadOutlined />}>
      {children}
    </Button>
  )
}

export default DownLoadButton