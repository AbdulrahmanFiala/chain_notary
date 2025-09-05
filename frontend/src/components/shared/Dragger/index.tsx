import computeFileHash from "@/utils/compileFileHash";
import getUint8Array from "@/utils/getUint8Array";
import { InboxOutlined } from '@ant-design/icons';
import { message, Upload, type FormInstance, type UploadProps } from "antd";
import type { FC } from "react";

type Props = {
  form: FormInstance
}


const Dragger: FC<Props> = ({ form }) => {

  const props: UploadProps = {
    name: 'file_data',
    listType: 'picture',
    multiple: false,
    maxCount: 1,

    onChange: async (info) => {
      const { status, size, type: file_type, originFileObj } = info.file;
      if (status === 'done') {
        const name = form.getFieldValue('name');
        const file_data = await getUint8Array(originFileObj as File)
        const file_hash = await computeFileHash(file_data);
        form.setFieldsValue({ file_data, file_type, file_size: BigInt(size || 0), file_hash, name: name || info.file.uid });
        message.success(`${info.file.name} file uploaded successfully.`);
      } else if (status === 'error') {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
    onRemove() {
      form.setFieldsValue({ file_data: [], file_type: '', file_size: BigInt(0), file_hash: '' });
    }
  };

  return (
    <Upload.Dragger {...props}>
      <p className="ant-upload-drag-icon">
        <InboxOutlined />
      </p>
      <p className="ant-upload-text">Click or drag document to notarize it</p>
      <p className="ant-upload-hint">
        Support for a single upload. Strictly prohibit from uploading unpersonal data or other band files.
      </p>
    </Upload.Dragger>
  )
}

export default Dragger