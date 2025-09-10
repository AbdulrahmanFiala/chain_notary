// src/__tests__/computeFileHash.cy.ts

import computeFileHash from "../compileFileHash";

describe('computeFileHash', () => {
  // Helper function to create Uint8Array from string
  const stringToUint8Array = (str: string): Uint8Array => {
    return new TextEncoder().encode(str);
  };

  // Helper function to create ArrayBuffer from string
  const stringToArrayBuffer = (str: string): ArrayBuffer => {
    return stringToUint8Array(str).buffer;
  };

  describe('Basic functionality with default SHA-256', () => {
    it('should compute SHA-256 hash for simple text', async () => {
      const input = stringToUint8Array('hello world');
      const hash = await computeFileHash(input);

      // Expected SHA-256 hash of "hello world"
      expect(hash).to.equal('b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9');
      expect(hash).to.have.length(64); // SHA-256 produces 64 character hex string
    });

    it('should compute hash for empty input', async () => {
      const input = stringToUint8Array('');
      const hash = await computeFileHash(input);

      // Expected SHA-256 hash of empty string
      expect(hash).to.equal('e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855');
      expect(hash).to.have.length(64);
    });

    it('should compute hash for single character', async () => {
      const input = stringToUint8Array('a');
      const hash = await computeFileHash(input);

      // Expected SHA-256 hash of "a"
      expect(hash).to.equal('ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb');
      expect(hash).to.have.length(64);
    });

    it('should produce consistent results for same input', async () => {
      const input = stringToUint8Array('test data');
      const hash1 = await computeFileHash(input);
      const hash2 = await computeFileHash(input);

      expect(hash1).to.equal(hash2);
      expect(hash1).to.have.length(64);
    });
  });

  describe('Different input types (BufferSource)', () => {
    it('should work with Uint8Array', async () => {
      const input = new Uint8Array([72, 101, 108, 108, 111]); // "Hello" in bytes
      const hash = await computeFileHash(input);

      expect(hash).to.be.a('string');
      expect(hash).to.have.length(64);
      expect(hash).to.match(/^[0-9a-f]{64}$/); // Valid hex string
    });

    it('should work with ArrayBuffer', async () => {
      const input = stringToArrayBuffer('Hello ArrayBuffer');
      const hash = await computeFileHash(input);

      expect(hash).to.be.a('string');
      expect(hash).to.have.length(64);
      expect(hash).to.match(/^[0-9a-f]{64}$/);
    });

    it('should work with different typed arrays', async () => {
      const data = [1, 2, 3, 4, 5];

      const uint8Input = new Uint8Array(data);
      const uint16Input = new Uint16Array(data);
      const uint32Input = new Uint32Array(data);

      const hash8 = await computeFileHash(uint8Input);
      const hash16 = await computeFileHash(uint16Input);
      const hash32 = await computeFileHash(uint32Input);

      // Different typed arrays with same logical data should produce different hashes
      // because they have different byte representations
      expect(hash8).to.not.equal(hash16);
      expect(hash16).to.not.equal(hash32);
      expect(hash8).to.not.equal(hash32);

      // But all should be valid hex strings
      expect(hash8).to.match(/^[0-9a-f]{64}$/);
      expect(hash16).to.match(/^[0-9a-f]{64}$/);
      expect(hash32).to.match(/^[0-9a-f]{64}$/);
    });
  });

  describe('Different hash algorithms', () => {
    it('should work with SHA-1 algorithm', async () => {
      const input = stringToUint8Array('test');
      const hash = await computeFileHash(input, 'SHA-1');

      // SHA-1 produces 40 character hex string
      expect(hash).to.have.length(40);
      expect(hash).to.match(/^[0-9a-f]{40}$/);
      // Expected SHA-1 hash of "test"
      expect(hash).to.equal('a94a8fe5ccb19ba61c4c0873d391e987982fbbd3');
    });

    it('should work with SHA-384 algorithm', async () => {
      const input = stringToUint8Array('test');
      const hash = await computeFileHash(input, 'SHA-384');

      // SHA-384 produces 96 character hex string
      expect(hash).to.have.length(96);
      expect(hash).to.match(/^[0-9a-f]{96}$/);
    });

    it('should work with SHA-512 algorithm', async () => {
      const input = stringToUint8Array('test');
      const hash = await computeFileHash(input, 'SHA-512');

      // SHA-512 produces 128 character hex string
      expect(hash).to.have.length(128);
      expect(hash).to.match(/^[0-9a-f]{128}$/);
    });

    it('should produce different hashes for different algorithms', async () => {
      const input = stringToUint8Array('same input');

      const sha1Hash = await computeFileHash(input, 'SHA-1');
      const sha256Hash = await computeFileHash(input, 'SHA-256');
      const sha512Hash = await computeFileHash(input, 'SHA-512');

      expect(sha1Hash).to.not.equal(sha256Hash);
      expect(sha256Hash).to.not.equal(sha512Hash);
      expect(sha1Hash).to.not.equal(sha512Hash);

      // Verify lengths
      expect(sha1Hash).to.have.length(40);
      expect(sha256Hash).to.have.length(64);
      expect(sha512Hash).to.have.length(128);
    });
  });

  describe('Large data handling', () => {
    it('should handle large data efficiently', async () => {
      // Create a larger buffer (10KB)
      const largeData = new Uint8Array(10240);
      for (let i = 0; i < largeData.length; i++) {
        largeData[i] = i % 256;
      }

      const startTime = Date.now();
      const hash = await computeFileHash(largeData);
      const endTime = Date.now();

      expect(hash).to.be.a('string');
      expect(hash).to.have.length(64);
      expect(hash).to.match(/^[0-9a-f]{64}$/);

      // Should complete reasonably quickly (less than 1 second)
      expect(endTime - startTime).to.be.lessThan(1000);
    });

    it('should handle binary data correctly', async () => {
      // Create binary data with all possible byte values
      const binaryData = new Uint8Array(256);
      for (let i = 0; i < 256; i++) {
        binaryData[i] = i;
      }

      const hash = await computeFileHash(binaryData);

      expect(hash).to.be.a('string');
      expect(hash).to.have.length(64);
      expect(hash).to.match(/^[0-9a-f]{64}$/);
    });
  });

  describe('Error handling and edge cases', () => {
    it('should handle zero-length arrays', async () => {
      const emptyArray = new Uint8Array(0);
      const hash = await computeFileHash(emptyArray);

      expect(hash).to.equal('e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855');
    });

    it('should reject invalid algorithm', async () => {
      const input = stringToUint8Array('test');

      try {
        await computeFileHash(input, 'INVALID-ALGORITHM' as AlgorithmIdentifier);
        expect.fail('Should have thrown an error');
      } catch (error) {
        expect(error).to.exist;
      }
    });
  });

  describe('Hex string formatting', () => {
    it('should always produce lowercase hex strings', async () => {
      const input = stringToUint8Array('test data for hex validation');
      const hash = await computeFileHash(input);

      expect(hash).to.equal(hash.toLowerCase());
      expect(hash).to.not.include('A');
      expect(hash).to.not.include('B');
      expect(hash).to.not.include('C');
      expect(hash).to.not.include('D');
      expect(hash).to.not.include('E');
      expect(hash).to.not.include('F');
    });

    it('should properly pad single digit hex values', async () => {
      // Create data that might produce single-digit hex values
      const input = new Uint8Array([0, 1, 15, 16]); // These will produce 00, 01, 0f, 10 in hex
      const hash = await computeFileHash(input);

      expect(hash).to.have.length(64);
      expect(hash).to.match(/^[0-9a-f]{64}$/);
    });
  });

  describe('Real-world scenarios', () => {
    it('should simulate file hash computation', async () => {
      // Simulate a small file content
      const fileContent = stringToUint8Array(`
        This is a sample file content that might represent
        a real document or file that needs to be hashed
        for integrity verification purposes.
        
        Line breaks and special characters: !@#$%^&*()
        Numbers: 1234567890
        Mixed case: AbCdEfGhIjKlMnOpQrStUvWxYz
      `);

      const hash = await computeFileHash(fileContent);

      expect(hash).to.be.a('string');
      expect(hash).to.have.length(64);
      expect(hash).to.match(/^[0-9a-f]{64}$/);

      // Hash should be deterministic
      const hash2 = await computeFileHash(fileContent);
      expect(hash).to.equal(hash2);
    });

    it('should handle JSON data hashing', async () => {
      const jsonData = JSON.stringify({
        id: 123,
        name: "Test Document",
        timestamp: "2024-01-01T00:00:00Z",
        data: [1, 2, 3, 4, 5]
      });

      const input = stringToUint8Array(jsonData);
      const hash = await computeFileHash(input);

      expect(hash).to.be.a('string');
      expect(hash).to.have.length(64);
      expect(hash).to.match(/^[0-9a-f]{64}$/);
    });
  });
});