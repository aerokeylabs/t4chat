import '@/assets/css/index.css';
import '@/assets/css/md.css';
import 'vue-sonner/style.css';

import { convex } from '@/plugins/convex';
import { dark } from '@clerk/themes';
import { clerkPlugin } from '@clerk/vue';
import { createPinia } from 'pinia';
import { createApp } from 'vue';

import App from '@/App.vue';
import { getMarkdownProcessor } from '@/lib/shiki';
import { router } from '@/router';

const CLERK_KEY = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
if (!CLERK_KEY) {
  throw new Error('`VITE_CLERK_PUBLISHABLE_KEY` not set in env');
}

const CONVEX_URL = import.meta.env.VITE_CONVEX_URL;
if (!CONVEX_URL) {
  throw new Error('`VITE_CONVEX_URL` not set in env');
}

createApp(App)
  .use(router)
  .use(createPinia())
  .use(clerkPlugin, { publishableKey: CLERK_KEY, appearance: { baseTheme: dark } })
  .use(convex, { convexUrl: CONVEX_URL })
  .mount('#app');

// load markdown processor early
console.time('init markdown processor');
getMarkdownProcessor().then(() => {
  console.timeEnd('init markdown processor');
});
