import type { Principal } from '@dfinity/principal';
import { backend } from 'declarations/backend';
import type { DocumentSummary } from 'declarations/backend/backend.did';

const getDocumentsByOwner = async (owner: Principal): Promise<DocumentSummary[]> => {
  if (!owner) throw new Error('owner is required');  
  const resposeData = await backend.get_documents_by_owner(owner)
  return resposeData;
}

export default getDocumentsByOwner;