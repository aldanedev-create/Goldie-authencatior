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

export function parseOtpUri(uriString: string): ParsedOtpUri {
  const trimmed = uriString.trim();
  if (!trimmed.toLowerCase().startsWith('otpauth://')) {
    throw new Error('Invalid protocol. Standard otpauth:// URI required.');
  }

  const url = new URL(trimmed);
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

  const secret = (params.get('secret') || '').replace(/\s+/g, '');
  if (!secret) {
    throw new Error('URI is missing required "secret" parameter.');
  }

  const paramIssuer = params.get('issuer')?.trim() || '';
  const issuer = paramIssuer || labelIssuer || 'Unknown Issuer';

  const rawAlgo = (params.get('algorithm') || 'SHA1').toUpperCase();
  let algorithm: 'SHA1' | 'SHA256' | 'SHA512' = 'SHA1';
  if (rawAlgo === 'SHA256' || rawAlgo === 'SHA512') {
    algorithm = rawAlgo;
  }

  const digits = parseInt(params.get('digits') || '6', 10);
  const period = parseInt(params.get('period') || '30', 10);
  const counter = parseInt(params.get('counter') || '0', 10);

  return {
    type: typeStr as 'totp' | 'hotp',
    issuer,
    accountName,
    secret,
    algorithm,
    digits: isNaN(digits) ? 6 : digits,
    period: isNaN(period) ? 30 : period,
    counter: isNaN(counter) ? 0 : counter,
  };
}