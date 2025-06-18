<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useLocalStorage } from '@vueuse/core';

// Mock attachments data
interface Attachment {
  id: string;
  filename: string;
  size: number; // in bytes
  type: string;
  dateUploaded: string;
  url: string;
}

const useAttachmentsStore = defineStore('attachments', () => {
  // Initial state will be empty - this is just for demo
  const attachments = useLocalStorage<Attachment[]>('attachments_list', [
    {
      id: '1',
      filename: 'document.pdf',
      size: 2500000,
      type: 'application/pdf',
      dateUploaded: '2025-06-10',
      url: '#',
    },
    {
      id: '2',
      filename: 'image.png',
      size: 1200000,
      type: 'image/png',
      dateUploaded: '2025-06-12',
      url: '#',
    },
    {
      id: '3',
      filename: 'data.csv',
      size: 500000,
      type: 'text/csv',
      dateUploaded: '2025-06-14',
      url: '#',
    },
  ]);

  function deleteAttachment(id: string) {
    const index = attachments.value.findIndex((att) => att.id === id);
    if (index !== -1) {
      attachments.value.splice(index, 1);
    }
  }

  function deleteAllAttachments() {
    attachments.value = [];
  }

  return {
    attachments,
    deleteAttachment,
    deleteAllAttachments,
  };
});

const attachmentsStore = useAttachmentsStore();
const showDeleteDialog = ref(false);
const selectedAttachment = ref<Attachment | null>(null);
const showDeleteAllDialog = ref(false);

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function getFileIcon(type: string): string {
  if (type.startsWith('image/')) return '🖼️';
  if (type === 'application/pdf') return '📄';
  if (type === 'text/csv') return '📊';
  return '📁';
}
</script>

<template>
  <section class="flex flex-col gap-6">
    <div>
      <h1 class="text-2xl font-bold">Attachments</h1>
      <p class="text-muted-foreground">View and manage files you've uploaded during chats.</p>
    </div>

    <div class="rounded-lg border p-6">
      <div class="mb-6 flex items-center justify-between">
        <h2 class="text-xl font-semibold">Your Attachments</h2>

        <Dialog v-model:open="showDeleteAllDialog" v-if="attachmentsStore.attachments.length > 0">
          <DialogTrigger asChild>
            <Button variant="destructive" size="sm"> Delete All Attachments </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Delete All Attachments?</DialogTitle>
              <DialogDescription>
                This will permanently delete all your uploaded files. This action cannot be undone.
              </DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <Button variant="outline" @click="showDeleteAllDialog = false">Cancel</Button>
              <Button
                variant="destructive"
                @click="
                  () => {
                    attachmentsStore.deleteAllAttachments();
                    showDeleteAllDialog = false;
                  }
                "
              >
                Delete All
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <div v-if="attachmentsStore.attachments.length === 0" class="py-12 text-center">
        <p class="text-muted-foreground">No attachments found.</p>
      </div>

      <div v-else class="grid gap-4">
        <Card v-for="attachment in attachmentsStore.attachments" :key="attachment.id" class="overflow-hidden">
          <CardContent class="p-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="text-2xl" aria-hidden="true">
                  {{ getFileIcon(attachment.type) }}
                </div>
                <div>
                  <h3 class="font-medium">{{ attachment.filename }}</h3>
                  <p class="text-muted-foreground text-sm">
                    {{ formatFileSize(attachment.size) }} · {{ attachment.dateUploaded }}
                  </p>
                </div>
              </div>

              <div class="flex gap-2">
                <Button variant="outline" size="sm" asChild>
                  <a :href="attachment.url" download>Download</a>
                </Button>

                <Dialog v-model:open="showDeleteDialog">
                  <DialogTrigger asChild>
                    <Button variant="destructive" size="sm" @click="selectedAttachment = attachment"> Delete </Button>
                  </DialogTrigger>
                  <DialogContent v-if="selectedAttachment">
                    <DialogHeader>
                      <DialogTitle>Delete Attachment?</DialogTitle>
                      <DialogDescription>
                        Are you sure you want to delete {{ selectedAttachment.filename }}? This action cannot be undone.
                      </DialogDescription>
                    </DialogHeader>
                    <DialogFooter>
                      <Button variant="outline" @click="showDeleteDialog = false">Cancel</Button>
                      <Button
                        variant="destructive"
                        @click="
                          () => {
                            if (selectedAttachment) {
                              attachmentsStore.deleteAttachment(selectedAttachment.id);
                              showDeleteDialog = false;
                            }
                          }
                        "
                      >
                        Delete
                      </Button>
                    </DialogFooter>
                  </DialogContent>
                </Dialog>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </section>
</template>
