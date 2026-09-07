import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import {
  checkVaultExists,
  initializeVault,
  unlockVault,
  lockVault as apiLockVault,
  getAccountCodes,
  addAccount as apiAddAccount,
  deleteAccount as apiDeleteAccount,
  copyToClipboard,
  AccountDisplay,
  AccountInput,
} from '@/services/vault';

export const useAuthenticatorStore = defineStore('authenticator', () => {
  const vaultExists = ref<boolean>(false);
  const isUnlocked = ref<boolean>(false);
  const accounts = ref<AccountDisplay[]>([]);
  const searchQuery = ref<string>('');
  const errorMessage = ref<string | null>(null);

  let refreshTimer: number | null = null;

  const filteredAccounts = computed(() => {
    const q = searchQuery.value.trim().toLowerCase();
    if (!q) return accounts.value;
    return accounts.value.filter(
      (acc) =>
        acc.issuer.toLowerCase().includes(q) ||
        acc.account_name.toLowerCase().includes(q)
    );
  });

  async function init() {
    try {
      vaultExists.value = await checkVaultExists();
    } catch (e: any) {
      errorMessage.value = e?.toString() || 'Failed to check vault status.';
    }
  }

  async function createVault(password: string) {
    errorMessage.value = null;
    try {
      await initializeVault(password);
      vaultExists.value = true;
      isUnlocked.value = true;
      startCodePolling();
    } catch (e: any) {
      errorMessage.value = e?.toString() || 'Failed to create vault.';
      throw e;
    }
  }

  async function unlock(password: string): Promise<boolean> {
    errorMessage.value = null;
    try {
      const success = await unlockVault(password);
      if (success) {
        isUnlocked.value = true;
        await refreshCodes();
        startCodePolling();
      }
      return success;
    } catch (e: any) {
      errorMessage.value = e?.toString() || 'Failed to unlock vault.';
      return false;
    }
  }

  async function lockVault() {
    stopCodePolling();
    try {
      await apiLockVault();
    } catch (_) {}
    isUnlocked.value = false;
    accounts.value = [];
  }

  async function refreshCodes() {
    if (!isUnlocked.value) return;
    try {
      accounts.value = await getAccountCodes();
    } catch (e: any) {
      if (e?.toString().includes('Vault is locked')) {
        lockVault();
      }
    }
  }

  function startCodePolling() {
    stopCodePolling();
    refreshCodes();
    refreshTimer = window.setInterval(() => {
      refreshCodes();
    }, 1000);
  }

  function stopCodePolling() {
    if (refreshTimer) {
      clearInterval(refreshTimer);
      refreshTimer = null;
    }
  }

  async function addNewAccount(account: AccountInput) {
    errorMessage.value = null;
    try {
      await apiAddAccount(account);
      await refreshCodes();
    } catch (e: any) {
      errorMessage.value = e?.toString() || 'Failed to add account.';
      throw e;
    }
  }

  async function removeAccount(id: string) {
    errorMessage.value = null;
    try {
      await apiDeleteAccount(id);
      await refreshCodes();
    } catch (e: any) {
      errorMessage.value = e?.toString() || 'Failed to delete account.';
    }
  }

  async function copyCode(code: string) {
    try {
      await copyToClipboard(code);
    } catch (e: any) {
      errorMessage.value = 'Failed to copy to clipboard.';
    }
  }

  return {
    vaultExists,
    isUnlocked,
    accounts,
    filteredAccounts,
    searchQuery,
    errorMessage,
    init,
    createVault,
    unlock,
    lockVault,
    refreshCodes,
    addNewAccount,
    removeAccount,
    copyCode,
  };
});