<script setup lang="ts">
import CommandMenu from '@/components/CommandMenu.vue';
import LoginPage from '@/components/LoginPage.vue';
import { Toaster } from '@/components/ui/sonner';
import { TooltipProvider } from '@/components/ui/tooltip';
import { useConvexAuth } from '@/composables/convex';
import { RouterView } from 'vue-router';

const { isLoading, isSignedIn } = useConvexAuth();
</script>

<template>
  <TooltipProvider disable-hoverable-content>
    <Toaster />
    <CommandMenu />
    <RouterView v-if="isSignedIn" />
    <main v-else-if="isLoading">Loading...</main>
    <LoginPage v-else>not authenticated</LoginPage>
  </TooltipProvider>
</template>
