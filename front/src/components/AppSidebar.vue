<script setup lang="ts">
import IconInput from '@/components/input/IconInput.vue';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenuItem,
  SidebarTrigger,
} from '@/components/ui/sidebar';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { useMutation, useReactiveQuery } from '@/composables/convex';
import { useCommandMenu } from '@/composables/useCommandMenu';
import type { Id } from '@/convex/_generated/dataModel';
import { api } from '@/convex/_generated/api';
import { SignInButton, useUser } from '@clerk/vue';
import { debouncedRef } from '@vueuse/core';
import { PinIcon, PinOffIcon, PlusIcon, SearchIcon, Settings2Icon, SunIcon, TrashIcon } from 'lucide-vue-next';
import moment from 'moment';
import { computed, ref } from 'vue';
import { toast } from 'vue-sonner';
import { RouterLink, useRoute, useRouter } from 'vue-router';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import Theme from '@/components/Theme.vue';

const menu = useCommandMenu();

const deleteThreadMutation = useMutation(api.threads.deleteThreadById);
const pinThreadMutation = useMutation(api.threads.pinThreadById);
const unpinThreadMutation = useMutation(api.threads.unpinThreadById);

const deleteThread = async (threadId: string, event: Event) => {
  event.preventDefault();
  event.stopPropagation();
  try {
    await deleteThreadMutation({ threadId: threadId as Id<'threads'> });

    toast.success('Thread deleted');

    if (route.params.thread === threadId) {
      router.push('/chat');
    }
  } catch (error) {
    console.error('Error deleting thread:', error);
    toast.error('Failed to delete thread');
  }
};

const togglePinThread = async (thread: Thread, event: Event) => {
  event.preventDefault();
  event.stopPropagation();
  try {
    if (thread.pinned) {
      await unpinThreadMutation({ threadId: thread._id as Id<'threads'> });
      toast.success('Thread unpinned');
    } else {
      await pinThreadMutation({ threadId: thread._id as Id<'threads'> });
      toast.success('Thread pinned');
    }
  } catch (error) {
    console.error('Error toggling pin status:', error);
    toast.error('Failed to update pin status');
  }
};

defineProps<{
  open: boolean;
}>();

const { isLoaded, isSignedIn, user } = useUser();

const router = useRouter();

function navigateToAccount() {
  router.push('/settings/account');
}

const query = ref('');
const debouncedQuery = debouncedRef(query, 150);
const args = computed(() => ({ query: debouncedQuery.value }));

const { data, error } = useReactiveQuery(api.threads.getThreads, args);

type Thread = NonNullable<typeof data.value>['threads'][number];

// group threads by day
const threads = computed(() => {
  if (!data.value) return [];

  if (error.value) {
    console.error('Error fetching threads:', error);
    return [];
  }

  // Separate pinned threads
  const pinnedThreads: Thread[] = [];
  const unpinnedThreads: Thread[] = [];

  data.value.threads.forEach((thread) => {
    if (thread.pinned) {
      pinnedThreads.push(thread);
    } else {
      unpinnedThreads.push(thread);
    }
  });

  // Group unpinned threads by date
  const groupedThreads = new Map<
    string,
    {
      date: string;
      threads: Thread[];
    }
  >();

  unpinnedThreads.forEach((thread) => {
    const threadDate = new Date(thread.createdAt);
    const date = threadDate.toLocaleDateString('en-US');
    const existing = groupedThreads.get(date) ?? { date, threads: [] };
    existing.threads.push(thread);
    groupedThreads.set(date, existing);
  });

  // Create result with Pinned section first if there are pinned threads
  const result = [];

  if (pinnedThreads.length > 0) {
    result.push({
      date: 'Pinned',
      threads: pinnedThreads,
    });
  }

  // Add the date-grouped threads
  const dateGroups = Array.from(groupedThreads).map(([_, { date, threads }]) => ({
    date: moment(date, 'MM/DD/YYYY').local().calendar(null, {
      lastDay: '[Yesterday]',
      sameDay: '[Today]',
      nextDay: '[Tomorrow]',
      lastWeek: '[Last] dddd',
      nextWeek: 'dddd',
      sameElse: 'L',
    }),
    threads,
  }));

  return [...result, ...dateGroups];
});

const route = useRoute();

const isOnNewPage = computed(() => {
  return route.path === '/chat';
});
</script>

<template>
  <Teleport to="body">
    <div class="sidebar-controls-container" :class="{ 'sidebar-open': open }">
      <SidebarTrigger />

      <div class="secondary-controls">
        <Button variant="ghost" size="icon-sm" @click="menu.toggle">
          <SearchIcon />
        </Button>

        <RouterLink to="/chat" custom v-slot="{ navigate }">
          <Button variant="ghost" size="icon-sm" @click="navigate" :disabled="isOnNewPage">
            <PlusIcon />
          </Button>
        </RouterLink>
      </div>
    </div>
  </Teleport>

  <Teleport to="body">
    <div class="top-right-controls">
      <Popover>
        <PopoverTrigger>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button variant="ghost" size="icon-sm">
                <SunIcon />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="bottom">Theme</TooltipContent>
          </Tooltip>
        </PopoverTrigger>

        <PopoverContent class="w-sm">
          <Theme />
        </PopoverContent>
      </Popover>

      <Tooltip>
        <TooltipTrigger asChild>
          <RouterLink to="/settings" custom v-slot="{ navigate }">
            <Button variant="ghost" size="icon-sm" @click="navigate">
              <Settings2Icon />
            </Button>
          </RouterLink>
        </TooltipTrigger>
        <TooltipContent side="bottom">Settings</TooltipContent>
      </Tooltip>
    </div>
  </Teleport>

  <Sidebar variant="inset">
    <SidebarHeader>
      <SidebarMenuItem class="flex h-16 items-center justify-center">
        <span>thingy</span>
      </SidebarMenuItem>

      <SidebarMenuItem class="px-1">
        <RouterLink to="/chat" custom v-slot="{ navigate }">
          <Button variant="outline" class="w-full" @click="navigate">New Chat</Button>
        </RouterLink>
      </SidebarMenuItem>

      <SidebarMenuItem class="px-1">
        <IconInput type="text" placeholder="Search your threads..." v-model="query">
          <SearchIcon />
        </IconInput>
      </SidebarMenuItem>
    </SidebarHeader>

    <SidebarContent class="custom-scrollbar">
      <SidebarGroup v-if="threads.length > 0" v-for="group in threads">
        <SidebarGroupLabel class="px-1">
          {{ group.date }}
        </SidebarGroupLabel>

        <SidebarMenuItem v-for="thread in group.threads" :key="thread._id">
          <RouterLink :to="`/chat/${thread._id}`" custom v-slot="{ navigate }">
            <Tooltip>
              <TooltipTrigger asChild>
                <Button variant="ghost" class="sidebar-button w-full justify-start px-2" @click="navigate">
                  <span class="thread-title">
                    {{ thread.title }}
                  </span>
                  <div class="action-buttons">
                    <Button
                      variant="ghost"
                      size="icon-sm"
                      class="pin-button"
                      @click="(e) => togglePinThread(thread, e)"
                    >
                      <PinIcon v-if="!thread.pinned" class="h-4 w-4" />
                      <PinOffIcon v-else class="h-4 w-4" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-sm"
                      class="delete-button"
                      @click="(e) => deleteThread(thread._id, e)"
                    >
                      <TrashIcon class="h-4 w-4" />
                    </Button>
                  </div>
                </Button>
              </TooltipTrigger>
              <TooltipContent side="right">
                {{ thread.title }}
              </TooltipContent>
            </Tooltip>
          </RouterLink>
        </SidebarMenuItem>
      </SidebarGroup>

      <SidebarMenuItem v-else-if="query.trim() != ''" class="flex items-center justify-center p-4 text-center">
        <span class="text-muted-foreground">No threads found for "{{ query }}"</span>
      </SidebarMenuItem>
    </SidebarContent>

    <SidebarFooter>
      <div class="flex items-center justify-center">
        <template v-if="isLoaded">
          <template v-if="isSignedIn && user">
            <Button
              variant="ghost"
              size="xl"
              class="flex w-full items-center justify-start gap-2 py-4"
              @click="navigateToAccount"
            >
              <Avatar>
                <AvatarImage :src="user.imageUrl" />
                <AvatarFallback>{{ (user.firstName ?? ' ')[0] }}{{ (user.lastName ?? ' ')[0] }}</AvatarFallback>
              </Avatar>
              <div>{{ user.fullName }}</div>
            </Button>
          </template>
          <template v-else>
            <SignInButton />
          </template>
        </template>
        <template v-else>
          <span>Loading</span>
        </template>
      </div>
    </SidebarFooter>
  </Sidebar>
</template>

<style>
.sidebar-controls-container {
  position: fixed;

  z-index: 1000;

  top: calc(var(--spacing) * 2);
  left: calc(var(--spacing) * 2);

  padding: var(--spacing);
  border-radius: var(--radius-sm);
  background: color-mix(in oklch, var(--secondary) 50%, transparent);

  display: flex;
  align-items: center;
  gap: var(--spacing);

  &.sidebar-open {
    background: transparent;
  }

  .secondary-controls {
    display: flex;
    align-items: center;
    gap: var(--spacing);
    opacity: 1;
    transition: opacity 0.2s ease-in-out;
  }

  &.sidebar-open .secondary-controls {
    opacity: 0;
  }
}

.top-right-controls {
  position: fixed;
  z-index: 1000;
  top: calc(var(--spacing) * 2);
  right: calc(var(--spacing) * 2);
  display: flex;
  align-items: center;
  gap: var(--spacing);
}

.sidebar-button {
  position: relative;
  display: flex;
  overflow: hidden;
  align-items: center;

  .thread-title {
    display: block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex-grow: 1;
    transition: width 0.2s ease;
    text-align: left;
    max-width: 100%;
  }

  .action-buttons {
    position: absolute;
    right: 8px;
    display: flex;
    gap: 0px;
    transform: translateX(100%);
    transition: transform 0.2s ease;
    flex-shrink: 0;
    padding-left: 4px;
    background-color: transparent;
  }

  &:hover {
    .action-buttons {
      transform: translateX(0);
    }

    .thread-title {
      max-width: calc(100% - 70px);
    }
  }

  .delete-button,
  .pin-button {
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  &:hover .delete-button,
  &:hover .pin-button {
    opacity: 0.7;
  }

  .delete-button:hover {
    color: var(--destructive-foreground);
  }

  .pin-button:hover {
    color: var(--chart-4);
  }
}

.delete-button {
  transition: opacity 0.2s ease;

  &:hover {
    color: var(--destructive);
  }
}
</style>
