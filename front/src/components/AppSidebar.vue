<script setup lang="ts">
import IconInput from '@/components/IconInput.vue';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
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
import { useQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import { SignInButton, useUser } from '@clerk/vue';
import { SearchIcon } from 'lucide-vue-next';
import { computed } from 'vue';
import { RouterLink, useRouter } from 'vue-router';
import moment from 'moment';

defineEmits<{
  (e: 'new-chat'): void;
}>();

const { isLoaded, isSignedIn, user } = useUser();

const router = useRouter();

function navigateToAccount() {
  router.push('/settings/account');
}

const { data, error } = useQuery(api.threads.getThreads);

type Thread = NonNullable<typeof data.value>['threads'][number];

// group threads by day
const threads = computed(() => {
  if (!data.value) return [];

  if (error.value) {
    console.error('Error fetching threads:', error);
    return [];
  }

  const groupedThreads = new Map<
    string,
    {
      date: string;
      threads: Thread[];
    }
  >();

  data.value.threads.forEach((thread) => {
    // skip empty titles to skip uninitialized threads
    if (thread.title == null) return;
    // Use local timezone for the date
    const threadDate = new Date(thread.createdAt);
    const date = threadDate.toLocaleDateString('en-US'); // Format as local date
    const existing = groupedThreads.get(date) ?? { date, threads: [] };
    existing.threads.push(thread);
    groupedThreads.set(date, existing);
  });

  return Array.from(groupedThreads).map(([_, { date, threads }]) => ({
    date: moment(date, 'MM/DD/YYYY').local().calendar(null, {
      lastDay: '[Yesterday]',
      sameDay: '[Today]',
      nextDay: '[Tomorrow]',
      lastWeek: '[last] dddd',
      nextWeek: 'dddd',
      sameElse: 'L',
    }),
    threads,
  }));
});
</script>

<template>
  <Teleport to="body">
    <div class="sidebar-controls-container">
      <SidebarTrigger />
    </div>
  </Teleport>

  <TooltipProvider>
  <Sidebar>
    <SidebarHeader class="flex h-16 items-center justify-center">
      <span>thingy</span>
    </SidebarHeader>

    <SidebarContent class="custom-scrollbar no-horizontal-scroll">
      <SidebarMenuItem class="px-2">
        <RouterLink to="/chat" custom v-slot="{ navigate }">
          <Button variant="outline" class="w-full" @click="navigate">New Chat</Button>
        </RouterLink>
      </SidebarMenuItem>

      <SidebarMenuItem class="px-2">
        <IconInput type="text" placeholder="Search your threads...">
          <SearchIcon />
        </IconInput>
      </SidebarMenuItem>

      <SidebarGroup v-for="group in threads">
        <SidebarGroupLabel class="px-2">
          {{ group.date }}
        </SidebarGroupLabel>

        <SidebarMenuItem v-for="thread in group.threads" :key="thread._id">
          <RouterLink :to="`/chat/${thread._id}`" custom v-slot="{ navigate }">
            <Tooltip>
              <TooltipTrigger asChild>
                <Button 
                  variant="ghost" 
                  class="sidebar-button w-full justify-start px-2" 
                  @click="navigate"
                >
                  <span class="truncate-text">
                  {{ thread.title }}
                  </span>
                </Button>
              </TooltipTrigger>
              <TooltipContent side="right">
                {{ thread.title }}
              </TooltipContent>
            </Tooltip>
          </RouterLink>
        </SidebarMenuItem>
      </SidebarGroup>
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
  </TooltipProvider>
</template>

<style>
.sidebar-controls-container {
  position: fixed;

  top: 0;
  left: calc(var(--spacing) * 2);
  height: calc(var(--spacing) * 16);

  display: flex;
}

.no-horizontal-scroll {
  overflow-x: hidden;
}

.sidebar-button {
  position: relative;
  display: flex;
}

.truncate-text {
  display: block;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
