<script setup lang="ts">
import { SignUp, useAuth } from '@clerk/vue';

const { isSignedIn } = useAuth();
</script>

<template>
  <main class="flex h-screen w-full items-center justify-center" v-if="!isSignedIn">
    <SignUp />
  </main>
</template>
