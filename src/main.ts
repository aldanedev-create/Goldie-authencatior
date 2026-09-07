import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import './style.css';

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);

// Prevent unhandled renderer errors from crashing UI silently
app.config.errorHandler = (err, _instance, info) => {
  console.error('[Vue Error]:', err, info);
};

app.mount('#app');