<script setup lang="ts">
import { useReactiveQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import type { AttachmentPart } from '@/lib/types/convex';
import { computed } from 'vue';

const props = defineProps<{
  part: AttachmentPart;
}>();

const args = computed(() => ({ id: props.part.id }));
const { data } = useReactiveQuery(api.attachments.getUrl, args);
const isImage = computed(() => {
  if (!data.value) return false;
  return data.value.mimeType.startsWith('image/');
});
</script>

<template>
  <div v-if="data?.url != null" class="attachment-part">
    <img v-if="isImage" :src="data.url" />
    <a v-else :href="data.url" target="_blank" rel="noopener noreferrer">
      {{ data.name }}
    </a>
  </div>
</template>

<style>
.attachment-part > img {
  border-radius: var(--radius-md);
}
</style>
