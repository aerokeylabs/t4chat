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

// State for command dialog visibility
const open = ref(false);

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

  // Close the command dialog
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
        </CommandGroup>
      </CommandList>
    </Command>
  </CommandDialog>
</template>
