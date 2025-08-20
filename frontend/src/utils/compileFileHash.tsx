export const computeFileHash = async (
  uint8Array: BufferSource,
  algorithm: AlgorithmIdentifier = 'SHA-256'
): Promise<string> => {
  const hashBuffer = await crypto.subtle.digest(algorithm, uint8Array);
  const documentHash = Array.from(new Uint8Array(hashBuffer)).map(b => b.toString(16).padStart(2, '0')).join('');
  return documentHash;
}

export default computeFileHash;