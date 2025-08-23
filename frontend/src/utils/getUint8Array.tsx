const getUint8Array = async (file: File | Blob): Promise<Uint8Array<ArrayBuffer>> => {
  const arrayBuffer = await file.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);
  return uint8Array;
}

export default getUint8Array;