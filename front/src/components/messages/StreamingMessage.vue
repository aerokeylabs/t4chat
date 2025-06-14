<script setup lang="ts">
import Prose from '@/components/Prose.vue';
import { Button } from '@/components/ui/button';
import { X } from 'lucide-vue-next';

defineProps<{
  message: string;
  cancelled?: boolean;
  canCancel?: boolean;
}>();

const emit = defineEmits<{
  cancel: [];
}>();

function handleCancel() {
  emit('cancel');
}
</script>

<template>
  <div class="text-primary-foreground mb-12 bg-pink-800 relative">
    <div v-if="cancelled" class="text-muted-foreground italic mb-2 px-4 pt-2">
      Message cancelled
    </div>
    <Prose :source="message" />
    <div v-if="canCancel && !cancelled" class="absolute top-2 right-2">
      <Button
        variant="ghost"
        size="sm"
        @click="handleCancel"
        class="h-6 w-6 p-0 hover:bg-red-600/20"
        title="Cancel message"
      >
        <X class="h-4 w-4" />
      </Button>
    </div>
    <div v-if="!cancelled && message" class="absolute bottom-2 right-2">
      <div class="animate-pulse text-xs text-muted-foreground">
        Generating...
      </div>
    </div>
  </div>
</template>
