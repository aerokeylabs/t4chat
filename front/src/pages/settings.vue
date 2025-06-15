<script setup lang="ts">
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import { useClerk, useUser } from '@clerk/vue';
import { ArrowLeftIcon, SunIcon } from 'lucide-vue-next';
import { RouterLink, RouterView } from 'vue-router';

const clerk = useClerk();
const { user } = useUser();

const navRoutes = [
  ['/settings/account', 'Account'],
  ['/settings/customization', 'Customization'],
  ['/settings/history', 'History & Sync'],
  ['/settings/models', 'Models'],
  ['/settings/keys', 'API Keys'],
  ['/settings/attachments', 'Attachments'],
  ['/settings/contact', 'Contact Us'],
];
</script>

<template>
  <header class="flex w-full justify-between p-8">
    <RouterLink to="/" custom v-slot="{ navigate }">
      <Button variant="ghost" @click="navigate">
        <ArrowLeftIcon class="mr-2" />
        Back to Chat
      </Button>
    </RouterLink>

    <div class="flex items-center gap-2">
      <Button variant="ghost" size="icon">
        <SunIcon />
      </Button>

      <Button variant="ghost" @click="clerk?.signOut()">Sign Out</Button>
    </div>
  </header>

  <div class="flex gap-2">
    <div class="w-96">
      <div v-if="user" class="flex flex-col items-center">
        <Avatar class="mb-4 size-36">
          <AvatarImage :src="user.imageUrl" />
          <AvatarFallback>U</AvatarFallback>
        </Avatar>

        <h1 class="text-2xl font-bold">{{ user.fullName }}</h1>
        <p class="text-muted-foreground">{{ user.primaryEmailAddress }}</p>
      </div>
    </div>

    <div class="px-4">
      <nav class="bg-secondary/30 flex flex-row flex-wrap gap-2 rounded-lg p-1">
        <RouterLink v-for="[to, name] in navRoutes" :to custom v-slot="{ navigate, isActive }">
          <Button variant="ghost" size="sm" class="grow" :class="{ 'bg-secondary/50': isActive }" @click="navigate">
            {{ name }}
          </Button>
        </RouterLink>
      </nav>

      <main class="settings">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style>
.settings {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;

  gap: var(--spacing);

  padding-block: calc(var(--spacing) * 4);
}
</style>
