<script setup lang="ts">
import { useAuthenticatorStore } from '@/stores/authenticator';
import { Shield, Lock, Info, ArrowLeft } from 'lucide-vue-next';

const emit = defineEmits<{
  (e: 'done'): void;
}>();

const store = useAuthenticatorStore();
</script>

<template>
  <div class="h-full flex flex-col space-y-4">
    <div class="flex items-center justify-between pb-3 border-b border-slate-800/80">
      <button
        @click="emit('done')"
        class="flex items-center space-x-1 text-xs text-slate-400 hover:text-slate-200"
      >
        <ArrowLeft class="w-3.5 h-3.5" />
        <span>Back to Vault</span>
      </button>
      <h2 class="text-xs font-semibold text-slate-200">Settings & Security</h2>
    </div>

    <!-- Security Information Section -->
    <div class="space-y-3">
      <div class="p-3 rounded-xl bg-[#101A2B] border border-subtle-blue space-y-2">
        <div class="flex items-center space-x-2 text-[#38BDF8]">
          <Shield class="w-4 h-4" />
          <h3 class="text-xs font-semibold">Vault Security</h3>
        </div>
        <p class="text-[11px] text-slate-400">
          Your secrets are encrypted using AES-256-GCM and derived via Argon2id. Keys are further protected using native Windows DPAPI.
        </p>
      </div>

      <div class="p-3 rounded-xl bg-[#101A2B] border border-slate-800 space-y-2">
        <div class="flex items-center space-x-2 text-slate-300">
          <Info class="w-4 h-4 text-[#F472B6]" />
          <h3 class="text-xs font-semibold">Privacy & Local Storage</h3>
        </div>
        <ul class="text-[11px] text-slate-400 space-y-1 list-disc list-inside">
          <li>100% Offline operation</li>
          <li>No background network telemetry</li>
          <li>Zero remote tracking</li>
        </ul>
      </div>
    </div>

    <!-- Lock Action Button -->
    <div class="pt-2">
      <button
        @click="store.lockVault"
        class="w-full flex items-center justify-center space-x-2 py-2 rounded-lg bg-[#FB7185]/10 border border-[#FB7185]/30 text-[#FB7185] hover:bg-[#FB7185]/20 text-xs font-medium transition-all"
      >
        <Lock class="w-3.5 h-3.5" />
        <span>Lock Vault Immediately</span>
      </button>
    </div>
  </div>
</template>