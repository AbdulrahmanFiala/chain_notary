import { Principal } from "@dfinity/principal";
import { backend } from "declarations/backend";
import type { Document } from "declarations/backend/backend.did";

export interface QueryDocumentsFilters {
  doc_id?: string;
  owner?: string;
  institution_id?: string;
  document_type?: string;
  quarter?: number;
  year?: number;
  start_date?: bigint;
  end_date?: bigint;
  offset?: bigint;
  limit?: bigint;
  sort_by?: string;
  sort_order?: string;
  include_file_data?: boolean;
}

export interface QueryDocumentsResult {
  documents: Document[];
  total_count: number;
}

const queryDocuments = async (
  filters: QueryDocumentsFilters = {},
): Promise<QueryDocumentsResult> => {
  const result = await backend.query_documents(
    filters.doc_id ? [filters.doc_id] : [],
    filters.owner ? [Principal.fromText(filters.owner)] : [],
    filters.institution_id ? [filters.institution_id] : [],
    filters.document_type ? [filters.document_type] : [],
    filters.quarter ? [filters.quarter] : [],
    filters.year ? [filters.year] : [],
    filters.start_date ? [filters.start_date] : [],
    filters.end_date ? [filters.end_date] : [],
    filters.offset ? [filters.offset] : [],
    filters.limit ? [filters.limit] : [],
    filters.sort_by ? [filters.sort_by] : [],
    filters.sort_order ? [filters.sort_order] : [],
    filters.include_file_data !== undefined ? [filters.include_file_data] : [],
  );

  return {
    documents: result[0],
    total_count: Number(result[1]),
  };
};

export default queryDocuments;
