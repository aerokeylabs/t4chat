<script setup lang="ts">
import IconInput from '@/components/IconInput.vue';
import { Button } from '@/components/ui/button';
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenuItem,
  SidebarTrigger,
} from '@/components/ui/sidebar';
import { SearchIcon } from 'lucide-vue-next';
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
        <Button variant="outline" class="w-full">New Chat</Button>
      </SidebarMenuItem>

      <SidebarMenuItem class="px-2">
        <IconInput type="text" placeholder="Search your threads...">
          <SearchIcon />
        </IconInput>
      </SidebarMenuItem>
    </SidebarContent>

    <SidebarFooter />
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
