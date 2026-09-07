<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { Html5Qrcode } from 'html5-qrcode';
import { AlertCircle } from 'lucide-vue-next';

const emit = defineEmits<{
  (e: 'scan', qrData: string): void;
  (e: 'cancel'): void;
}>();

const scanError = ref<string | null>(null);
let html5Qrcode: Html5Qrcode | null = null;

onMounted(async () => {
  try {
    html5Qrcode = new Html5Qrcode('qr-reader-container');
    await html5Qrcode.start(
      { facingMode: 'environment' },
      { fps: 10, qrbox: { width: 220, height: 220 } },
      (decodedText) => {
        stopScanner();
        emit('scan', decodedText);
      },
      () => {}
    );
  } catch (err: any) {
    scanError.value = 'Failed to access camera. Check device permissions.';
  }
});

async function stopScanner() {
  if (html5Qrcode && html5Qrcode.isScanning) {
    try {
      await html5Qrcode.stop();
      html5Qrcode.clear();
    } catch (_) {}
  }
}

onUnmounted(() => {
  stopScanner();
});
</script>

<template>
  <div class="flex flex-col items-center justify-center space-y-4">
    <div v-if="scanError" class="p-3 rounded-lg bg-[#FB7185]/10 border border-[#FB7185]/30 text-[#FB7185] text-xs flex items-center space-x-2">
      <AlertCircle class="w-4 h-4 shrink-0" />
      <span>{{ scanError }}</span>
    </div>

    <!-- QR Camera Video Viewport -->
    <div class="relative w-64 h-64 rounded-xl overflow-hidden border border-[#38BDF8]/40 bg-[#0B1220] glow-blue-sm">
      <div id="qr-reader-container" class="w-full h-full"></div>
      
      <!-- Overlay Scanner Framing UI -->
      <div class="absolute inset-0 border-2 border-dashed border-[#38BDF8]/60 pointer-events-none rounded-xl"></div>
    </div>

    <button
      @click="emit('cancel')"
      class="px-4 py-1.5 rounded-lg text-xs text-slate-400 hover:text-slate-200 bg-[#0B1220] border border-slate-800"
    >
      Cancel Scanner
    </button>
  </div>
</template>