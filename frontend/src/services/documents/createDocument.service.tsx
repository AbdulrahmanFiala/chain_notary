import { backend } from "declarations/backend";
import type { Document } from "declarations/backend/backend.did";

const createDocumentService = async (data: Document) => {
  const mintedFile = await backend.upload_file_and_publish_document(data);
  return mintedFile;

}

export default createDocumentService;