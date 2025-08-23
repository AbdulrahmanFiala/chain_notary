/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_PRINCIPAL_ID: string;
  // add more VITE_ vars here if you need
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
