<script setup lang="ts">
import { Toaster } from '@/components/ui/sonner';
import { useConvexAuth } from '@/composables/convex';
import { RouterView } from 'vue-router';
import CommandMenu from '@/components/CommandMenu.vue';
import { TooltipProvider } from '@/components/ui/tooltip';

const { isLoading, isSignedIn } = useConvexAuth();
</script>

<template>
  <TooltipProvider disable-hoverable-content>
    <Toaster />
    <CommandMenu />
    <RouterView v-if="isSignedIn" />
    <main v-else-if="isLoading">Loading...</main>
    <main v-else>not authenticated</main>
  </TooltipProvider>
</template>
