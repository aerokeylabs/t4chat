import '@/assets/css/index.css';
import '@/assets/css/md.css';

import { createApp } from 'vue';

import App from './App.vue';
import { getMarkdownProcessor } from './lib/shiki';

createApp(App).mount('#app');

// load markdown processor early
console.time('init markdown processor');
getMarkdownProcessor().then(() => {
  console.timeEnd('init markdown processor');
});
