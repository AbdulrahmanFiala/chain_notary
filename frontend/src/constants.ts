export const network = process.env.DFX_NETWORK;

export const identityProvider =
  network === "local"
    ? "http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:8080" // Mainnet
    : "https://identity.ic0.app"; // Local

export const mimeToExtension: { [key: string]: string } = {
  "text/plain": "txt",
  "application/pdf": "pdf",
  "image/png": "png",
  "image/jpeg": "jpg",
  "image/gif": "gif",
  "application/json": "json",
  "text/csv": "csv",
  "application/vnd.ms-excel": "xls",
  "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet": "xlsx",
  "application/msword": "doc",
  "application/vnd.openxmlformats-officedocument.wordprocessingml.document":
    "docx",
  "application/zip": "zip",
  "application/octet-stream": "bin",
};
