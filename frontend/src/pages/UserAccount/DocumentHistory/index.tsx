import { mimeToExtension } from "@/constants";
import getDocumentsByOwner from "@/services/documents/getDocumentsByOwner";
import { useAppSelector } from "@/store/hooks";
import { FileTextOutlined, LoadingOutlined, RightOutlined } from "@ant-design/icons";
import { Principal } from "@dfinity/principal";
import { Button, Spin } from "antd";
import type { DocumentSummary } from "declarations/backend/backend.did";
import { useCallback, useEffect, useState, type FC } from "react";
import { NavLink } from "react-router";

const DocumentHistory: FC = () => {

  const [documents, setDocuments] = useState<DocumentSummary[]>([]);
  const [isLoading, setIsloading] = useState<boolean>(false);
  const { principal } = useAppSelector(state => state.auth);

  const getDocuments = useCallback(async () => {
    setIsloading(true);
    try {
      const ownerPrincipal = Principal.fromText(principal);
      const data = await getDocumentsByOwner(ownerPrincipal);
      setDocuments(data);
    } catch (error) {
      console.error(error);

    } finally {
      setIsloading(false);
    }

  }, [principal]);

  useEffect(() => {
    getDocuments();
  }, [getDocuments]);

  return (
    <div className="bg-white p-6 rounded-lg shadow-sm">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold text-gray-900">Recent Document History</h2>
        {(!isLoading || documents?.length > 3) && <Button variant="link" color="primary" className="text-sm font-medium">View All</Button>}
      </div>
      <ul className="divide-y divide-gray-200">
        {documents?.map(({ id, document_name, file_type, publication_date }) => (
          <li key={id} className="py-4 flex items-center justify-between">
            <div className="flex items-center">
              <FileTextOutlined style={{ fontSize: '24px', color: 'var(--color-blue-600)' }} className="me-6" />
              <div>
                <p className="font-medium text-gray-900">{document_name ? `${document_name}.${mimeToExtension[file_type]}` : 'Unnamed Document'}</p>
                <p className="text-sm text-gray-500">{`Notarized on: ${publication_date}`}</p>
              </div>
            </div>
            <NavLink to={`/document/${id}/view`}><RightOutlined className="cursor-pointer" /></NavLink>

          </li>
        ))}
        {documents?.length === 0 && <li className="py-4 text-center text-gray-500">{isLoading ? <Spin indicator={<LoadingOutlined spin />} /> : "No documents found."}</li>}
      </ul>
    </div>

  )
}

export default DocumentHistory