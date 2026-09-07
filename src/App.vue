<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useAuthenticatorStore } from '@/stores/authenticator';
import Home from '@/views/Home.vue';
import AddAccount from '@/views/AddAccount.vue';
import Settings from '@/views/Settings.vue';
import UnlockModal from '@/components/UnlockModal.vue';
import { Lock, Shield, Plus, Settings as SettingsIcon, Search, KeyRound } from 'lucide-vue-next';

const store = useAuthenticatorStore();
const currentView = ref<'home' | 'add' | 'settings'>('home');

// Inactivity Auto-Lock Timer (3 minutes default)
let idleTimer: number | null = null;
const IDLE_TIMEOUT_MS = 3 * 60 * 1000;

function resetIdleTimer() {
  if (idleTimer) clearTimeout(idleTimer);
  if (store.isUnlocked) {
    idleTimer = window.setTimeout(() => {
      store.lockVault();
    }, IDLE_TIMEOUT_MS);
  }
}

function handleUserActivity() {
  resetIdleTimer();
}

onMounted(async () => {
  window.addEventListener('mousemove', handleUserActivity);
  window.addEventListener('keydown', handleUserActivity);
  window.addEventListener('click', handleUserActivity);
  
  await store.init();
});

onUnmounted(() => {
  window.removeEventListener('mousemove', handleUserActivity);
  window.removeEventListener('keydown', handleUserActivity);
  window.removeEventListener('click', handleUserActivity);
  if (idleTimer) clearTimeout(idleTimer);
});

const isHomeView = computed(() => currentView.value === 'home');
</script>

<template>
  <div class="flex flex-col h-screen w-screen bg-[#070B14] text-[#F8FAFC] overflow-hidden select-none">
    <!-- Header -->
    <header class="flex items-center justify-between px-5 py-3.5 bg-[#0B1220] border-b border-subtle-blue">
      <div class="flex items-center space-x-2.5 cursor-pointer" @click="currentView = 'home'">
        <div class="relative flex items-center justify-center w-8 h-8 rounded-lg bg-[#101A2B] border border-[#38BDF8]/30 text-[#38BDF8] glow-blue-sm">
          <Shield class="w-4 h-4" />
          <div class="absolute -top-0.5 -right-0.5 w-2 h-2 rounded-full bg-[#F472B6]" />
        </div>
        <div>
          <h1 class="text-sm font-semibold tracking-wide text-slate-100 flex items-center gap-1.5">
            Aegis Vault
          </h1>
          <p class="text-[10px] text-slate-400 tracking-wider uppercase font-medium">Secure • Private • Offline</p>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center space-x-2">
        <button
          v-if="store.isUnlocked"
          @click="currentView = currentView === 'add' ? 'home' : 'add'"
          class="flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/30 hover:bg-[#38BDF8]/10 hover:border-[#38BDF8]/60 transition-all duration-150 glow-blue-sm"
          title="Add New Account"
        >
          <Plus class="w-3.5 h-3.5" />
          <span>{{ currentView === 'add' ? 'Vault' : 'Add' }}</span>
        </button>

        <button
          v-if="store.isUnlocked"
          @click="currentView = currentView === 'settings' ? 'home' : 'settings'"
          class="p-1.5 rounded-lg text-slate-400 hover:text-slate-200 bg-[#101A2B] border border-slate-800 hover:border-slate-700 transition-colors"
          title="Settings"
        >
          <SettingsIcon class="w-4 h-4" />
        </button>

        <button
          v-if="store.isUnlocked"
          @click="store.lockVault"
          class="p-1.5 rounded-lg text-slate-400 hover:text-[#FB7185] bg-[#101A2B] border border-slate-800 hover:border-[#FB7185]/30 transition-colors"
          title="Lock Vault Now"
        >
          <Lock class="w-4 h-4" />
        </button>
      </div>
    </header>

    <!-- Sub-header Search Bar (Visible only when unlocked and on Home view) -->
    <div v-if="store.isUnlocked && isHomeView" class="px-5 py-3 bg-[#070B14] border-b border-slate-800/60">
      <div class="relative flex items-center">
        <Search class="absolute left-3 w-3.5 h-3.5 text-slate-500" />
        <input
          v-model="store.searchQuery"
          type="text"
          placeholder="Search accounts or issuers..."
          class="w-full pl-9 pr-3 py-1.5 bg-[#0B1220] border border-slate-800 rounded-lg text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-[#38BDF8]/50 transition-colors"
        />
      </div>
    </div>

    <!-- Main View Dynamic Slot -->
    <main class="flex-1 overflow-y-auto p-5 relative">
      <template v-if="store.isUnlocked">
        <Home v-if="currentView === 'home'" @navigate="currentView = $event" />
        <AddAccount v-else-if="currentView === 'add'" @done="currentView = 'home'" />
        <Settings v-else-if="currentView === 'settings'" @done="currentView = 'home'" />
      </template>

      <!-- Locked State Content Placeholder -->
      <div v-else class="h-full flex flex-col items-center justify-center text-center p-6 space-y-4">
        <div class="p-4 rounded-full bg-[#101A2B] border border-[#38BDF8]/20 text-[#38BDF8] glow-blue">
          <KeyRound class="w-8 h-8" />
        </div>
        <div>
          <h2 class="text-base font-semibold text-slate-200">Vault Encrypted</h2>
          <p class="text-xs text-slate-400 mt-1 max-w-xs">
            Enter your master password or PIN to decrypt stored authentication tokens.
          </p>
        </div>
      </div>
    </main>

    <!-- Unlock Modal Overlay -->
    <UnlockModal v-if="!store.isUnlocked" />
  </div>
</template>