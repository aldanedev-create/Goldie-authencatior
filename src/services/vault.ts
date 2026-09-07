import { invoke } from '@tauri-apps/api/core';

export interface AccountDisplay {
  id: string;
  issuer: string;
  account_name: string;
  otp_type: 'totp' | 'hotp';
  digits: number;
  period: number;
  counter: number;
  current_code: string;
  seconds_remaining: number;
}

export interface AccountInput {
  id: string;
  issuer: string;
  account_name: string;
  secret: string;
  otp_type: 'totp' | 'hotp';
  algorithm: 'SHA1' | 'SHA256' | 'SHA512';
  digits: number;
  period: number;
  counter: number;
}

export async function checkVaultExists(): Promise<boolean> {
  return await invoke<boolean>('check_vault_exists');
}

export async function initializeVault(password: string): Promise<void> {
  await invoke('initialize_vault', { password });
}

export async function unlockVault(password: string): Promise<boolean> {
  return await invoke<boolean>('unlock_vault', { password });
}

export async function lockVault(): Promise<void> {
  await invoke('lock_vault');
}

export async function getAccountCodes(): Promise<AccountDisplay[]> {
  return await invoke<AccountDisplay[]>('get_account_codes');
}

export async function addAccount(account: AccountInput): Promise<void> {
  await invoke('add_account', { account });
}

export async function deleteAccount(id: string): Promise<void> {
  await invoke('delete_account', { id });
}

export async function copyToClipboard(text: string): Promise<void> {
  await invoke('copy_to_clipboard', { text });
}