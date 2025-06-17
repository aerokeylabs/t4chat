<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command';
import Theme from '@/components/commands/Theme.vue';

const open = ref(false);
const themeDialogOpen = ref(false);

const router = useRouter();

// Command groups and items
const pages = [
  { name: 'Home', path: '/' },
  { name: 'Settings', path: '/settings' },
  { name: 'Account Settings', path: '/settings/account' },
  { name: 'Customization', path: '/settings/customization' },
  { name: 'Models', path: '/settings/models' },
  { name: 'API Keys', path: '/settings/keys' },
  { name: 'History & Sync', path: '/settings/history' },
  { name: 'Attachments', path: '/settings/attachments' },
];

function onKeyDown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault();
    open.value = !open.value;
  }
}

// Handle command selection
function runCommand(command: string) {
  // Navigate to page
  if (command.startsWith('/')) {
    router.push(command);
  }

  if (command === 'new-chat') {
    router.push('/chat');
  }

  if (command === 'open-theme') {
    themeDialogOpen.value = true;
  }

  open.value = false;
}

// Register and cleanup keyboard event listener
onMounted(() => {
  window.addEventListener('keydown', onKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', onKeyDown);
});
</script>

<template>
  <CommandDialog v-model:open="open">
    <Command>
      <CommandInput placeholder="Type a command or search..." />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>
        <CommandGroup heading="Navigation">
          <CommandItem v-for="page in pages" :key="page.path" :value="page.path" @select="() => runCommand(page.path)">
            <span>{{ page.name }}</span>
          </CommandItem>
        </CommandGroup>
        <CommandGroup heading="Actions">
          <CommandItem value="new-chat" @select="() => runCommand('new-chat')">
            <span>New Chat</span>
          </CommandItem>
          <CommandItem value="clear-history" @select="() => runCommand('clear-history')">
            <span>Clear History</span>
          </CommandItem>
          <CommandItem value="open-theme" @select="() => runCommand('open-theme')">
            <span>Theme Settings</span>
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  </CommandDialog>

  <!-- Theme Dialog -->
  <div v-if="themeDialogOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <div class="bg-background w-80 rounded-lg shadow-lg">
      <div class="flex items-center justify-between border-b p-3">
        <h3 class="text-lg font-medium">Theme Settings</h3>
        <button @click="themeDialogOpen = false" class="hover:bg-muted rounded-full p-1">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
      <Theme @close="themeDialogOpen = false" />
    </div>
  </div>
</template>
