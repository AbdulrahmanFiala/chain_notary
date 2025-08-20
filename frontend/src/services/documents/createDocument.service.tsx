import type { FormData } from "@/Interfaces";
import { Principal } from "@dfinity/principal";
import { message } from "antd";
import { backend } from "declarations/backend";

const createDocumentService = async (data: FormData) => {

  const fileObj = data.fileData?.originFileObj;
  if (!fileObj) {
    message.error('Please upload a file.');
    return;
  }

  const arrayBuffer = await fileObj.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);
  
  // Calculate a document hash (example using SHA-256)
  const hashBuffer = await crypto.subtle.digest('SHA-256', uint8Array);
  const documentHash = Array.from(new Uint8Array(hashBuffer)).map(b => b.toString(16).padStart(2, '0')).join('');

  const ownerPrincipal = Principal.fromText("y5ii6-maar4-sidi3-ftt7h-5f7vs-lgpdn-zw6hp-xdccz-vvuyx-d2mth-uqe");

  const mintedFile = await backend.upload_file_and_publish_document(
    uint8Array,
    fileObj.type,
    {
      collection_id: [],
      document_id: data.nftName || "default_id",
      owner: ownerPrincipal,
      name: data.nftName || "Untitled Document",
      description: [data.nftDescription || "No description provided"],
      image_url: [],
      document_hash: documentHash,
      file_size: BigInt(fileObj.size || 0),
      file_type: fileObj.type || "application/octet-stream",
      file_data: [uint8Array],
      recipient: [{ name: data.rewarderName || "Unknown Recipient", id: [data.nationalId || ""], email: [data.email || ""] }],
    }
  );
  return mintedFile;

}

export default createDocumentService;