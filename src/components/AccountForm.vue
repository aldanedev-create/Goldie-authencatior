<script setup lang="ts">
import { ref } from 'vue';
import { AccountInput } from '@/services/vault';
import { isValidBase32 } from '@/services/totp';

const emit = defineEmits<{
  (e: 'save', account: AccountInput): void;
  (e: 'cancel'): void;
}>();

const issuer = ref('');
const accountName = ref('');
const secret = ref('');
const otpType = ref<'totp' | 'hotp'>('totp');
const algorithm = ref<'SHA1' | 'SHA256' | 'SHA512'>('SHA1');
const digits = ref<number>(6);
const period = ref<number>(30);
const counter = ref<number>(0);
const validationError = ref<string | null>(null);

function handleSubmit() {
  validationError.value = null;

  if (!issuer.value.trim()) {
    validationError.value = 'Issuer name is required.';
    return;
  }
  if (!accountName.value.trim()) {
    validationError.value = 'Account username/email is required.';
    return;
  }
  if (!isValidBase32(secret.value)) {
    validationError.value = 'Invalid secret key format. Must be a valid Base32 string.';
    return;
  }
  if (otpType.value === 'hotp' && (!Number.isInteger(counter.value) || counter.value < 0)) {
    validationError.value = 'Starting counter must be a non-negative whole number.';
    return;
  }

  const newAccount: AccountInput = {
    id: crypto.randomUUID(),
    issuer: issuer.value.trim(),
    account_name: accountName.value.trim(),
    secret: secret.value.trim().toUpperCase().replace(/\s+/g, ''),
    otp_type: otpType.value,
    algorithm: algorithm.value,
    digits: digits.value,
    // The backend only requires period > 0 for TOTP accounts; HOTP ignores
    // it, so this default is harmless but keeps the field non-zero either way.
    period: otpType.value === 'totp' ? period.value : 30,
    counter: otpType.value === 'hotp' ? counter.value : 0,
  };

  emit('save', newAccount);
}
</script>

<template>
  <form @submit.prevent="handleSubmit" class="space-y-4">
    <div v-if="validationError" class="p-2.5 rounded-lg bg-[#FB7185]/10 border border-[#FB7185]/30 text-[#FB7185] text-xs">
      {{ validationError }}
    </div>

    <div>
      <label class="block text-xs font-medium text-slate-300 mb-1">Type</label>
      <div class="flex bg-[#0B1220] p-1 rounded-lg border border-slate-800">
        <button
          type="button"
          @click="otpType = 'totp'"
          class="flex-1 py-1.5 rounded-md text-xs font-medium transition-colors"
          :class="otpType === 'totp' ? 'bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/30' : 'text-slate-400 hover:text-slate-200'"
        >
          Time-based (TOTP)
        </button>
        <button
          type="button"
          @click="otpType = 'hotp'"
          class="flex-1 py-1.5 rounded-md text-xs font-medium transition-colors"
          :class="otpType === 'hotp' ? 'bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/30' : 'text-slate-400 hover:text-slate-200'"
        >
          Counter-based (HOTP)
        </button>
      </div>
    </div>

    <div>
      <label class="block text-xs font-medium text-slate-300 mb-1">Issuer Name</label>
      <input
        v-model="issuer"
        type="text"
        maxlength="256"
        placeholder="e.g. GitHub, Google, AWS"
        class="w-full px-3 py-2 bg-[#0B1220] border border-slate-800 rounded-lg text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-[#38BDF8]/50"
      />
    </div>

    <div>
      <label class="block text-xs font-medium text-slate-300 mb-1">Account Name / Email</label>
      <input
        v-model="accountName"
        type="text"
        maxlength="256"
        placeholder="e.g. user@example.com"
        class="w-full px-3 py-2 bg-[#0B1220] border border-slate-800 rounded-lg text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-[#38BDF8]/50"
      />
    </div>

    <div>
      <label class="block text-xs font-medium text-slate-300 mb-1">Secret Key (Base32)</label>
      <input
        v-model="secret"
        type="password"
        placeholder="JBSWY3DPEHPK3PXP"
        class="w-full px-3 py-2 bg-[#0B1220] border border-slate-800 rounded-lg text-xs font-mono text-slate-100 placeholder-slate-500 focus:outline-none focus:border-[#38BDF8]/50"
      />
    </div>

    <!-- Advanced Configuration Row -->
    <div class="grid grid-cols-3 gap-2 pt-2 border-t border-slate-800/60">
      <div>
        <label class="block text-[10px] text-slate-400 mb-1">Algorithm</label>
        <select
          v-model="algorithm"
          class="w-full px-2 py-1.5 bg-[#0B1220] border border-slate-800 rounded-md text-xs text-slate-300 focus:outline-none"
        >
          <option value="SHA1">SHA-1</option>
          <option value="SHA256">SHA-256</option>
          <option value="SHA512">SHA-512</option>
        </select>
      </div>

      <div>
        <label class="block text-[10px] text-slate-400 mb-1">Digits</label>
        <select
          v-model="digits"
          class="w-full px-2 py-1.5 bg-[#0B1220] border border-slate-800 rounded-md text-xs text-slate-300 focus:outline-none"
        >
          <option :value="6">6 digits</option>
          <option :value="8">8 digits</option>
        </select>
      </div>

      <div v-if="otpType === 'totp'">
        <label class="block text-[10px] text-slate-400 mb-1">Period (s)</label>
        <input
          v-model.number="period"
          type="number"
          min="10"
          max="120"
          class="w-full px-2 py-1.5 bg-[#0B1220] border border-slate-800 rounded-md text-xs text-slate-300 focus:outline-none"
        />
      </div>
      <div v-else>
        <label class="block text-[10px] text-slate-400 mb-1">Starting Counter</label>
        <input
          v-model.number="counter"
          type="number"
          min="0"
          step="1"
          class="w-full px-2 py-1.5 bg-[#0B1220] border border-slate-800 rounded-md text-xs text-slate-300 focus:outline-none"
        />
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center justify-end space-x-2 pt-3">
      <button
        type="button"
        @click="emit('cancel')"
        class="px-3 py-1.5 rounded-lg text-xs text-slate-400 hover:text-slate-200 bg-[#0B1220] border border-slate-800"
      >
        Cancel
      </button>
      <button
        type="submit"
        class="px-4 py-1.5 rounded-lg text-xs font-medium bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/40 hover:bg-[#38BDF8]/10 glow-blue-sm"
      >
        Save Account
      </button>
    </div>
  </form>
</template>