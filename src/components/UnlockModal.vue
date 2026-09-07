<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useAuthenticatorStore } from '@/stores/authenticator';
import { KeyRound, ShieldCheck, Loader2 } from 'lucide-vue-next';

const store = useAuthenticatorStore();

const password = ref('');
const confirmPassword = ref('');
const localError = ref<string | null>(null);
const isSubmitting = ref(false);
const checkedVault = ref(false);

// Vault existence is only known after store.init() resolves.
onMounted(async () => {
  if (!store.vaultExists) {
    await store.init();
  }
  checkedVault.value = true;
});

const isCreateMode = computed(() => checkedVault.value && !store.vaultExists);

async function handleSubmit() {
  localError.value = null;

  if (isCreateMode.value) {
    if (password.value.length < 8) {
      localError.value = 'Master password must be at least 8 characters.';
      return;
    }
    if (password.value !== confirmPassword.value) {
      localError.value = 'Passwords do not match.';
      return;
    }
    isSubmitting.value = true;
    try {
      await store.createVault(password.value);
    } catch {
      // errorMessage is already set on the store
    } finally {
      isSubmitting.value = false;
    }
  } else {
    if (!password.value) {
      localError.value = 'Enter your master password.';
      return;
    }
    isSubmitting.value = true;
    try {
      const success = await store.unlock(password.value);
      if (!success && !store.errorMessage) {
        localError.value = 'Incorrect password.';
      }
    } finally {
      isSubmitting.value = false;
    }
  }

  password.value = '';
  confirmPassword.value = '';
}
</script>

<template>
  <div class="absolute inset-0 z-20 flex items-center justify-center bg-[#070B14]/90 backdrop-blur-sm">
    <div class="w-full max-w-xs mx-4 p-5 rounded-xl bg-[#101A2B] border border-subtle-blue glow-blue space-y-4">
      <div class="flex flex-col items-center text-center space-y-2">
        <div class="p-3 rounded-full bg-[#0B1220] border border-[#38BDF8]/30 text-[#38BDF8] glow-blue-sm">
          <component :is="isCreateMode ? ShieldCheck : KeyRound" class="w-6 h-6" />
        </div>
        <h2 class="text-sm font-semibold text-slate-100">
          {{ isCreateMode ? 'Create Your Vault' : 'Unlock Vault' }}
        </h2>
        <p class="text-[11px] text-slate-400 max-w-[220px]">
          {{
            isCreateMode
              ? 'Choose a strong master password. It cannot be recovered if lost.'
              : 'Enter your master password to decrypt your accounts.'
          }}
        </p>
      </div>

      <form @submit.prevent="handleSubmit" class="space-y-3">
        <input
          v-model="password"
          type="password"
          autofocus
          :placeholder="isCreateMode ? 'New master password' : 'Master password'"
          class="w-full px-3 py-2 bg-[#0B1220] border border-slate-800 rounded-lg text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-[#38BDF8]/50"
        />

        <input
          v-if="isCreateMode"
          v-model="confirmPassword"
          type="password"
          placeholder="Confirm master password"
          class="w-full px-3 py-2 bg-[#0B1220] border border-slate-800 rounded-lg text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-[#38BDF8]/50"
        />

        <div
          v-if="localError || store.errorMessage"
          class="p-2.5 rounded-lg bg-[#FB7185]/10 border border-[#FB7185]/30 text-[#FB7185] text-[11px]"
        >
          {{ localError || store.errorMessage }}
        </div>

        <button
          type="submit"
          :disabled="isSubmitting"
          class="w-full flex items-center justify-center space-x-2 py-2 rounded-lg text-xs font-medium bg-[#101A2B] text-[#38BDF8] border border-[#38BDF8]/40 hover:bg-[#38BDF8]/10 transition-all glow-blue-sm disabled:opacity-50"
        >
          <Loader2 v-if="isSubmitting" class="w-3.5 h-3.5 animate-spin" />
          <span>{{ isCreateMode ? 'Create Vault' : 'Unlock' }}</span>
        </button>
      </form>
    </div>
  </div>
</template>
