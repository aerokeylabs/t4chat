<script setup lang="ts">
import Theme from '@/components/Theme.vue';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { useSettings } from '@/composables/settings';
import { useClerk, useUser } from '@clerk/vue';
import { useResizeObserver } from '@vueuse/core';
import { ArrowLeftIcon, SunIcon } from 'lucide-vue-next';
import { ref, useTemplateRef, watch } from 'vue';
import { RouterLink, RouterView, useRoute, useRouter } from 'vue-router';

const clerk = useClerk();
const { user } = useUser();

const navRoutes = [
  ['/settings/account', 'Account'],
  ['/settings/customization', 'Customization'],
  ['/settings/history', 'History & Sync'],
  ['/settings/keys', 'API Keys'],
];

const route = useRoute();
const router = useRouter();

watch(
  route,
  () => {
    if (route.path === '/settings') router.push('/settings/account');
  },
  { immediate: true },
);

const settings = useSettings();

const navContainer = useTemplateRef('nav-container');
const navContainerHeight = ref(0);
useResizeObserver(navContainer, () => {
  navContainerHeight.value = navContainer.value?.clientHeight || 0;
});
</script>

<template>
  <div class="settings-page" :style="{ '--nav-container-height': `${navContainerHeight}px` }">
    <header>
      <RouterLink to="/" custom v-slot="{ navigate }">
        <Button variant="ghost" @click="navigate">
          <ArrowLeftIcon class="mr-2" />
          Back to Chat
        </Button>
      </RouterLink>

      <div class="flex items-center gap-2">
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

        <Button variant="ghost" @click="clerk?.signOut()">Sign Out</Button>
      </div>
    </header>

    <div ref="nav-container" class="nav-container">
      <div>
        <nav class="bg-secondary/30 flex flex-row flex-wrap gap-2 rounded-lg p-1">
          <RouterLink v-for="[to, name] in navRoutes" :to custom v-slot="{ navigate, isActive }">
            <Button variant="ghost" size="sm" class="grow" :class="{ 'bg-secondary/50': isActive }" @click="navigate">
              {{ name }}
            </Button>
          </RouterLink>
        </nav>
      </div>
    </div>

    <div class="scroller custom-scrollbar">
      <div class="user-info" :class="{ 'hide-personal-info': settings.hidePersonalInfo }">
        <div v-if="user" class="flex flex-col items-center">
          <Avatar class="personal-info mb-4 size-36">
            <AvatarImage :src="user.imageUrl" />
            <AvatarFallback>U</AvatarFallback>
          </Avatar>

          <h1 class="personal-info text-2xl font-bold">{{ user.fullName }}</h1>
          <p class="personal-info text-muted-foreground">{{ user.primaryEmailAddress }}</p>
        </div>
      </div>

      <div class="content-container">
        <main class="settings custom-scrollbar">
          <RouterView />
        </main>
      </div>
    </div>
  </div>
</template>

<style>
.settings-page {
  position: relative;
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100vh;
  overflow: hidden;

  --header-height: calc(var(--spacing) * 25);
  --user-info-width: calc(var(--spacing) * 72);

  > header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;

    z-index: 10;

    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: calc(var(--spacing) * 8);
    height: var(--header-height);

    background: var(--background);
  }

  > .nav-container {
    position: absolute;
    top: var(--header-height);
    width: 100%;
    z-index: 5;

    display: flex;
    justify-content: center;
    padding-inline: calc(var(--spacing) * 2);

    > div {
      margin-left: var(--user-info-width);
      max-width: var(--container-4xl);
      background: var(--background);
      width: 100%;
    }
  }

  > .scroller {
    display: flex;
    flex-direction: row;
    justify-content: center;

    overflow-y: auto;
    overflow-x: hidden;

    width: 100%;
    height: 100%;
    min-height: 0;

    padding-top: calc(var(--header-height) + var(--nav-container-height));
    padding-left: calc(var(--spacing) * 2);
    padding-right: calc(var(--spacing) * 2);

    > .user-info {
      position: sticky;
      top: 0;
      width: var(--user-info-width);
      min-width: var(--user-info-width);

      &.hide-personal-info {
        .personal-info {
          filter: blur(18px);
          pointer-events: none;
          user-select: none;
        }
      }
    }

    > .content-container {
      position: relative;

      display: flex;
      flex-direction: column;
      align-items: center;
      width: 100%;
      max-width: var(--container-4xl);

      > .settings {
        display: flex;
        flex-direction: column;

        width: 100%;

        padding-inline: calc(var(--spacing) * 4);
        padding-block: calc(var(--spacing) * 8);

        > section {
          display: flex;
          flex-direction: column;
          width: 100%;
        }
      }
    }
  }
}

@media (width <= 48rem) {
  .settings-page .nav-container > div {
    margin-left: 0;
  }

  .user-info {
    display: none;
  }
}
</style>
