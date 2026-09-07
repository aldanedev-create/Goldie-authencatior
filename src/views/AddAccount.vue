<script setup lang="ts">
import { ref } from 'vue';
import { useAuthenticatorStore } from '@/stores/authenticator';
import { parseOtpUri } from '@/services/otpUri';
import { AccountInput } from '@/services/vault';
import AccountForm from '@/components/AccountForm.vue';
import QRScanner from '@/components/QRScanner.vue';
import { QrCode, Edit3, ArrowLeft } from 'lucide-vue-next';

const emit = defineEmits<{
  (e: 'done'): void;
}>();

const store = useAuthenticatorStore();
const activeTab = ref<'qr' | 'manual'>('qr');
const errorMessage = ref<string | null>(null);

async function handleQrScan(qrData: string) {
  errorMessage.value = null;
  try {
    const parsed = parseOtpUri(qrData);
    const newAcc: AccountInput = {
      id: crypto.randomUUID(),
      issuer: parsed.issuer,
      account_name: parsed.accountName,
      secret: parsed.secret,
      otp_type: parsed.type,
      algorithm: parsed.algorithm,
      digits: parsed.digits,
      period: parsed.period,
      counter: parsed.counter,
    };
    await store.addNewAccount(newAcc);
    emit('done');
  } catch (err: any) {
    errorMessage.value = err?.message || 'Failed to parse QR code.';
  }
}

async function handleManualSave(account: AccountInput) {
  errorMessage.value = null;
  try {
    await store.addNewAccount(account);
    emit('done');
  } catch (err: any) {
    errorMessage.value = err?.toString() || 'Failed to save account.';
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header Navigation -->
    <div class="flex items-center justify-between pb-3 border-b border-slate-800/80 mb-4">
      <button
        @click="emit('done')"
        class="flex items-center space-x-1 text-xs text-slate-400 hover:text-slate-200"
      >
        <ArrowLeft class="w-3.5 h-3.5" />
        <span>Back to Vault</span>
      </button>
      <h2 class="text-xs font-semibold text-slate-200">Add New Account</h2>
    </div>

    <!-- Mode Selector Tabs -->
    <div class="flex bg-[#0B1220] p-1 rounded-lg border border-slate-800 mb-4">
      <button
        @click="activeTab = 'qr'"
        class="flex-1 flex items-center justify-center space-x-1.5 py-1.5 rounded-md text-xs font-medium transition-colors"
        :class="activeTab === 'qr' ? 'bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/30' : 'text-slate-400 hover:text-slate-200'"
      >
        <QrCode class="w-3.5 h-3.5" />
        <span>Scan QR Code</span>
      </button>

      <button
        @click="activeTab = 'manual'"
        class="flex-1 flex items-center justify-center space-x-1.5 py-1.5 rounded-md text-xs font-medium transition-colors"
        :class="activeTab === 'manual' ? 'bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/30' : 'text-slate-400 hover:text-slate-200'"
      >
        <Edit3 class="w-3.5 h-3.5" />
        <span>Manual Entry</span>
      </button>
    </div>

    <div v-if="errorMessage" class="mb-3 p-2.5 rounded-lg bg-[#FB7185]/10 border border-[#FB7185]/30 text-[#FB7185] text-xs">
      {{ errorMessage }}
    </div>

    <!-- Dynamic Tab Content -->
    <div class="flex-1">
      <QRScanner
        v-if="activeTab === 'qr'"
        @scan="handleQrScan"
        @cancel="emit('done')"
      />
      <AccountForm
        v-else
        @save="handleManualSave"
        @cancel="emit('done')"
      />
    </div>
  </div>
</template>