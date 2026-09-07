<script setup lang="ts">
import { useAuthenticatorStore } from '@/stores/authenticator';
import AccountCard from '@/components/AccountCard.vue';
import { Shield, Plus } from 'lucide-vue-next';

const store = useAuthenticatorStore();
const emit = defineEmits<{
  (e: 'navigate', view: 'add'): void;
}>();
</script>

<template>
  <div class="h-full">
    <!-- Active Accounts List View -->
    <div v-if="store.filteredAccounts.length > 0" class="grid grid-cols-1 gap-3">
      <AccountCard
        v-for="acc in store.filteredAccounts"
        :key="acc.id"
        :account="acc"
      />
    </div>

    <!-- Empty State View -->
    <div v-else-if="store.accounts.length === 0" class="h-full flex flex-col items-center justify-center text-center p-6 space-y-4">
      <div class="w-12 h-12 rounded-full bg-[#101A2B] border border-[#38BDF8]/30 flex items-center justify-center text-[#38BDF8] glow-blue-sm">
        <Shield class="w-6 h-6" />
      </div>
      <div>
        <h3 class="text-sm font-semibold text-slate-200">No accounts yet</h3>
        <p class="text-xs text-slate-400 mt-1 max-w-xs">
          Add your first authentication account to generate secure 2FA codes.
        </p>
      </div>
      <button
        @click="emit('navigate', 'add')"
        class="flex items-center space-x-1.5 px-4 py-2 rounded-lg text-xs font-medium bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/40 hover:bg-[#38BDF8]/10 transition-all glow-blue-sm"
      >
        <Plus class="w-4 h-4" />
        <span>Add Account</span>
      </button>
    </div>

    <!-- Search No Results View -->
    <div v-else class="h-full flex flex-col items-center justify-center text-center p-6 space-y-2">
      <p class="text-xs text-slate-400">No accounts matching "{{ store.searchQuery }}"</p>
    </div>
  </div>
</template>