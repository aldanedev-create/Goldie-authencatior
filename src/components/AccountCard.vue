<script setup lang="ts">
import { ref, computed } from 'vue';
import { AccountDisplay } from '@/services/vault';
import { useAuthenticatorStore } from '@/stores/authenticator';
import { Copy, Check, Trash2 } from 'lucide-vue-next';

const props = defineProps<{
  account: AccountDisplay;
}>();

const store = useAuthenticatorStore();
const copied = ref(false);
const showDeleteConfirm = ref(false);

const formattedCode = computed(() => {
  const raw = props.account.current_code;
  if (raw.length === 6) {
    return `${raw.slice(0, 3)} ${raw.slice(3)}`;
  }
  if (raw.length === 8) {
    return `${raw.slice(0, 4)} ${raw.slice(4)}`;
  }
  return raw;
});

// Calculate progress percentage and color transition based on time remaining
const progress = computed(() => {
  const period = props.account.period || 30;
  return (props.account.seconds_remaining / period) * 100;
});

const isExpiring = computed(() => props.account.seconds_remaining <= 5);

async function handleCopy() {
  await store.copyCode(props.account.current_code);
  copied.value = true;
  setTimeout(() => {
    copied.value = false;
  }, 2000);
}

function handleDelete() {
  store.removeAccount(props.account.id);
  showDeleteConfirm.value = false;
}
</script>

<template>
  <div
    class="relative group rounded-xl bg-[#101A2B] border border-subtle-blue p-4 transition-all duration-200 hover:border-[#38BDF8]/40 hover:glow-blue-sm"
  >
    <!-- Card Header: Issuer & Action Options -->
    <div class="flex items-start justify-between">
      <div class="flex items-center space-x-3">
        <div class="w-8 h-8 rounded-lg bg-[#0B1220] border border-slate-800 flex items-center justify-center text-[#38BDF8] font-bold text-xs">
          {{ account.issuer.slice(0, 2).toUpperCase() }}
        </div>
        <div>
          <h3 class="text-sm font-semibold text-slate-100 tracking-wide">{{ account.issuer }}</h3>
          <p class="text-xs text-slate-400 font-mono text-[11px] truncate max-w-[180px]">{{ account.account_name }}</p>
        </div>
      </div>

      <button
        @click="showDeleteConfirm = true"
        class="text-slate-500 hover:text-[#FB7185] p-1 rounded-md transition-colors opacity-0 group-hover:opacity-100"
        title="Delete account"
      >
        <Trash2 class="w-3.5 h-3.5" />
      </button>
    </div>

    <!-- OTP Display Row -->
    <div class="mt-4 flex items-center justify-between">
      <div class="flex items-baseline space-x-2">
        <span class="font-mono text-2xl font-bold tracking-widest text-slate-50 select-all">
          {{ formattedCode }}
        </span>
      </div>

      <!-- Copy Button -->
      <button
        @click="handleCopy"
        class="flex items-center space-x-1 px-3 py-1.5 rounded-lg text-xs font-medium bg-[#0B1220] border border-slate-800 text-slate-300 hover:text-[#38BDF8] hover:border-[#38BDF8]/40 transition-all"
      >
        <Check v-if="copied" class="w-3.5 h-3.5 text-[#22D3EE]" />
        <Copy v-else class="w-3.5 h-3.5" />
        <span>{{ copied ? 'Copied' : 'Copy' }}</span>
      </button>
    </div>

    <!-- Countdown Progress Bar -->
    <div v-if="account.otp_type === 'totp'" class="mt-3 flex items-center space-x-2">
      <div class="flex-1 h-1.5 bg-[#0B1220] rounded-full overflow-hidden border border-slate-800/80">
        <div
          class="h-full transition-all duration-1000 ease-linear rounded-full"
          :class="isExpiring ? 'bg-[#F472B6] glow-pink' : 'bg-[#38BDF8] glow-blue-sm'"
          :style="{ width: `${progress}%` }"
        />
      </div>
      <span
        class="text-[10px] font-mono font-medium"
        :class="isExpiring ? 'text-[#F472B6]' : 'text-slate-400'"
      >
        {{ account.seconds_remaining }}s
      </span>
    </div>

    <!-- Delete Confirmation Modal Overlay -->
    <div
      v-if="showDeleteConfirm"
      class="absolute inset-0 bg-[#070B14]/95 rounded-xl p-4 flex flex-col justify-center items-center text-center space-y-3 z-10"
    >
      <p class="text-xs text-slate-200">Remove <span class="font-semibold text-slate-100">{{ account.issuer }}</span>?</p>
      <div class="flex space-x-2">
        <button
          @click="handleDelete"
          class="px-3 py-1 rounded bg-[#FB7185]/20 text-[#FB7185] border border-[#FB7185]/40 text-xs font-medium hover:bg-[#FB7185]/30"
        >
          Confirm
        </button>
        <button
          @click="showDeleteConfirm = false"
          class="px-3 py-1 rounded bg-[#101A2B] text-slate-400 border border-slate-800 text-xs hover:bg-slate-800"
        >
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>