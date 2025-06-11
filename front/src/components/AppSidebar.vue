<script setup lang="ts">
import IconInput from '@/components/IconInput.vue';
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

// group threads by day
const threads = computed(() => {
  if (!data.value) return [];

  if (error.value) {
    console.error('Error fetching threads:', error);
    return [];
  }

  const groupedThreads: Record<string, (typeof data.value.threads)[number][]> = {};

  data.value.threads.forEach((thread) => {
    const date = new Date(thread.createdAt).toISOString();
    if (!groupedThreads[date]) groupedThreads[date] = [];
    groupedThreads[date].push(thread);
  });

  return Object.entries(groupedThreads).map(([date, threads]) => ({
    date: moment(date).calendar(null, {
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

  <Sidebar>
    <SidebarHeader class="flex h-16 items-center justify-center">
      <span>thingy</span>
    </SidebarHeader>

    <SidebarContent>
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
            <Button variant="ghost" class="w-full justify-start px-2" @click="navigate">
              {{ thread.title }}
            </Button>
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
</template>

<style>
.sidebar-controls-container {
  position: fixed;

  top: 0;
  left: calc(var(--spacing) * 2);
  height: calc(var(--spacing) * 16);

  z-index: 1000;

  display: flex;
  align-items: center;
}
</style>
