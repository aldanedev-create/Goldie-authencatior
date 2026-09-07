import { isValidBase32 } from './totp';

export interface ParsedOtpUri {
  type: 'totp' | 'hotp';
  issuer: string;
  accountName: string;
  secret: string;
  algorithm: 'SHA1' | 'SHA256' | 'SHA512';
  digits: number;
  period: number;
  counter: number;
}

const MAX_LABEL_LENGTH = 256;

export function parseOtpUri(uriString: string): ParsedOtpUri {
  const trimmed = uriString.trim();
  if (!trimmed.toLowerCase().startsWith('otpauth://')) {
    throw new Error('Invalid protocol. Standard otpauth:// URI required.');
  }

  // A scanned QR code is untrusted input. Reject anything absurdly long
  // before even attempting to parse it as a URL.
  if (trimmed.length > 2000) {
    throw new Error('QR code content is too long to be a valid otpauth:// URI.');
  }

  let url: URL;
  try {
    url = new URL(trimmed);
  } catch {
    throw new Error('Malformed otpauth:// URI.');
  }
  const typeStr = url.host.toLowerCase();

  if (typeStr !== 'totp' && typeStr !== 'hotp') {
    throw new Error(`Unsupported OTP type: "${typeStr}". Expected totp or hotp.`);
  }

  // Label parsing: /Issuer:AccountName or /AccountName
  const pathname = decodeURIComponent(url.pathname.replace(/^\//, ''));
  let labelIssuer = '';
  let accountName = pathname;

  if (pathname.includes(':')) {
    const parts = pathname.split(':');
    labelIssuer = parts[0].trim();
    accountName = parts.slice(1).join(':').trim();
  }

  const params = url.searchParams;

  const secret = (params.get('secret') || '').replace(/\s+/g, '').toUpperCase();
  if (!secret) {
    throw new Error('URI is missing required "secret" parameter.');
  }
  if (!isValidBase32(secret)) {
    throw new Error('QR code secret is not valid Base32.');
  }

  const paramIssuer = params.get('issuer')?.trim() || '';
  const issuer = (paramIssuer || labelIssuer || 'Unknown Issuer').slice(0, MAX_LABEL_LENGTH);
  const trimmedAccountName = (accountName || 'Unknown Account').slice(0, MAX_LABEL_LENGTH);

  const rawAlgo = (params.get('algorithm') || 'SHA1').toUpperCase();
  let algorithm: 'SHA1' | 'SHA256' | 'SHA512' = 'SHA1';
  if (rawAlgo === 'SHA256' || rawAlgo === 'SHA512') {
    algorithm = rawAlgo;
  }

  const digitsRaw = parseInt(params.get('digits') || '6', 10);
  const periodRaw = parseInt(params.get('period') || '30', 10);
  const counterRaw = parseInt(params.get('counter') || '0', 10);

  // A malicious or malformed QR code could otherwise smuggle an out-of-range
  // digits/period value all the way to the Rust backend. The backend also
  // validates this, but failing fast here gives a clearer error and avoids
  // an unnecessary round trip.
  const digits = isNaN(digitsRaw) || digitsRaw < 6 || digitsRaw > 8 ? 6 : digitsRaw;
  const period = isNaN(periodRaw) || periodRaw < 10 || periodRaw > 300 ? 30 : periodRaw;
  const counter = isNaN(counterRaw) || counterRaw < 0 ? 0 : Math.floor(counterRaw);

  return {
    type: typeStr as 'totp' | 'hotp',
    issuer,
    accountName: trimmedAccountName,
    secret,
    algorithm,
    digits,
    period,
    counter,
  };
}