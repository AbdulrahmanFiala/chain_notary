import { backend } from 'declarations/backend';
import type { Document } from 'declarations/backend/backend.did';

const getDocumentDetails = async (documentId: string): Promise<Document | []> => {
  if (!documentId) throw new Error('Document ID is required');
  const resposeData = await backend.get_document_metadata(documentId)
  // TODO: Handle the case where the response data is empty or null
  console.log('Response Data: ', resposeData);
  return resposeData.length ? resposeData?.[0] : [];
}

export default getDocumentDetails;