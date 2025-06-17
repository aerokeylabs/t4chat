<script setup lang="ts">
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command';
import { useReactiveQuery } from '@/composables/convex';
import { useCommandMenu } from '@/composables/useCommandMenu';
import { api } from '@/convex/_generated/api';
import { debouncedRef, useMagicKeys, whenever } from '@vueuse/core';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';

const menu = useCommandMenu();
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

const { Ctrl_K, Meta_K } = useMagicKeys({
  passive: false,
  onEventFired(e) {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k' && e.type === 'keydown') e.preventDefault();
  },
});

whenever(Ctrl_K, menu.toggle);
whenever(Meta_K, menu.toggle);

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

  menu.close();
}

const query = ref('');
const debouncedQuery = debouncedRef(query, 150);
const args = computed(() => ({ query: debouncedQuery.value }));

const { data } = useReactiveQuery(api.threads.getThreads, args);
</script>

<template>
  <CommandDialog v-model:open="menu.open">
    <Command>
      <CommandInput placeholder="Type a command or search..." v-model="query" />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>

        <CommandGroup heading="Navigation">
          <CommandItem v-for="page in pages" :key="page.path" :value="page.path" @select="runCommand(page.path)">
            <span>{{ page.name }}</span>
          </CommandItem>
        </CommandGroup>

        <CommandGroup heading="Actions">
          <CommandItem value="new-chat" @select="runCommand('new-chat')">
            <span>New Chat</span>
          </CommandItem>
          <CommandItem value="clear-history" @select="runCommand('clear-history')">
            <span>Clear History</span>
          </CommandItem>
          <CommandItem value="open-theme" @select="runCommand('open-theme')">
            <span>Theme Settings</span>
          </CommandItem>
        </CommandGroup>

        <CommandGroup v-if="data?.threads != null && data?.threads.length > 0" heading="Threads">
          <CommandItem
            v-for="thread in data?.threads ?? []"
            :key="thread._id"
            :value="`/chat/${thread._id}`"
            @select="() => runCommand(`/chat/${thread._id}`)"
          >
            <span>{{ thread.title }}</span>
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  </CommandDialog>
</template>
