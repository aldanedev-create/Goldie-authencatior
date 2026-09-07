// Algorithm and digits helper types matching backend definitions
export type HashAlgo = 'SHA1' | 'SHA256' | 'SHA512';

// Base32 Clean & Decode
export function decodeBase32(input: string): Uint8Array {
  const alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
  const clean = input.trim().replace(/\s+/g, '').toUpperCase().replace(/=+$/, '');
  
  let bits = 0;
  let value = 0;
  const output = new Uint8Array(Math.floor((clean.length * 5) / 8));
  let index = 0;

  for (let i = 0; i < clean.length; i++) {
    const char = clean.charAt(i);
    const val = alphabet.indexOf(char);
    if (val === -1) {
      throw new Error(`Invalid Base32 character: ${char}`);
    }
    value = (value << 5) | val;
    bits += 5;

    if (bits >= 8) {
      output[index++] = (value >>> (bits - 8)) & 0xff;
      bits -= 8;
    }
  }

  return output;
}

// Validates whether a secret string is valid Base32 format
export function isValidBase32(secret: string): boolean {
  if (!secret || secret.trim().length === 0) return false;
  try {
    const bytes = decodeBase32(secret);
    return bytes.length > 0;
  } catch {
    return false;
  }
}