import { Principal } from "@dfinity/principal";
import { backend } from "declarations/backend";
import type { Document } from "declarations/backend/backend.did";

const createDocumentService = async (data: Document) => {
  const ownerPrincipal = Principal.fromText(import.meta.env.VITE_PRINCIPAL_ID);  
  const mintedFile = await backend.upload_file_and_publish_document(
    {
      collection_id: [],
      document_id: data.document_id,
      owner: ownerPrincipal,
      name: data.name,
      description: data.description,
      document_hash: data.document_hash,
      file_size: data.file_size,
      file_type: data.file_type,
      file_data: data.file_data,
      document_data: {
        EarningRelease: {
          earning_release_id: data.document_data.EarningRelease.earning_release_id,
          consolidated_balance_sheet_data: {
            total_assets: data.document_data.EarningRelease.consolidated_balance_sheet_data.total_assets,
            total_equity: data.document_data.EarningRelease.consolidated_balance_sheet_data.total_equity,
            total_liabilities_and_equity: data.document_data.EarningRelease.consolidated_balance_sheet_data.total_liabilities_and_equity,
            total_liabilities: data.document_data.EarningRelease.consolidated_balance_sheet_data.total_liabilities
          },
          consolidated_income_data: {
            ebitda: data.document_data.EarningRelease.consolidated_income_data.ebitda,
            gross_profit: data.document_data.EarningRelease.consolidated_income_data.gross_profit,
            net_profit: data.document_data.EarningRelease.consolidated_income_data.net_profit,
            operating_profit: data.document_data.EarningRelease.consolidated_income_data.operating_profit,
            profit_before_tax: data.document_data.EarningRelease.consolidated_income_data.profit_before_tax
          },
          year: data.document_data.EarningRelease.year,
          quarter: data.document_data.EarningRelease.quarter

        }
      },
      institution_id: [],
    }
  );
  return mintedFile;

}

export default createDocumentService;