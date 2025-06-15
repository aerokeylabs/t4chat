<script setup lang="ts">
import { Toaster } from '@/components/ui/sonner';
import { useConvexAuth } from '@/composables/convex';
import { RouterView } from 'vue-router';

const isAuthenticated = useConvexAuth();
</script>

<template>
  <Toaster />
  <RouterView v-if="isAuthenticated" />
  <main v-else>not authenticated</main>
</template>
