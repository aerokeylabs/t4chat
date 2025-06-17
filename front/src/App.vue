<script setup lang="ts">
import { Toaster } from '@/components/ui/sonner';
import { useConvexAuth } from '@/composables/convex';
import { RouterView } from 'vue-router';
import CommandMenu from '@/components/CommandMenu.vue';
import { TooltipProvider } from '@/components/ui/tooltip';

const isAuthenticated = useConvexAuth();
</script>

<template>
  <TooltipProvider disable-hoverable-content>
    <Toaster />
    <CommandMenu />
    <RouterView v-if="isAuthenticated" />
    <main v-else>not authenticated</main>
  </TooltipProvider>
</template>
