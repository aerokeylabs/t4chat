<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { ScrollArea } from '@/components/ui/scroll-area';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { useMutation, useReactiveQuery } from '@/composables/convex';
import type { Id } from '@/convex/_generated/dataModel';
import { api } from '@/convex/_generated/api';
import { ref, computed } from 'vue';
import { TrashIcon } from 'lucide-vue-next';
import { toast } from 'vue-sonner';
import moment from 'moment';

const deleteThreadMutation = useMutation(api.threads.deleteThreadById);

const query = ref('');
const queryArgs = computed(() => ({
  query: query.value,
}));
const { data: threadsData } = useReactiveQuery(api.threads.getThreads, queryArgs);

const threadIds = computed(() => threadsData.value?.threads.map((t) => t._id) || []);
const { data: messageCounts } = useReactiveQuery(
  api.threads.getMessageCounts,
  computed(() => ({ threadIds: threadIds.value })),
);

const formattedThreads = computed(() => {
  if (!threadsData.value) return [];

  return threadsData.value.threads
    .map((thread) => ({
      id: thread._id,
      title: thread.title || 'Untitled conversation',
      date: moment(thread._creationTime).format('YYYY-MM-DD'),
      messages: messageCounts.value?.[thread._id] || 0,
      ...thread,
    }))
    .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
});

const deleteThread = async (threadId: string) => {
  try {
    await deleteThreadMutation({ threadId: threadId as Id<'threads'> });
    toast.success('Conversation deleted');
  } catch (error) {
    console.error('Error deleting conversation:', error);
    toast.error('Failed to delete conversation');
  }
};

const deleteAllThreads = async () => {
  if (!threadsData.value) return;

  try {
    const deletePromises = threadsData.value.threads.map((thread) => deleteThread(thread._id));

    await Promise.all(deletePromises);
    toast.success('All conversations deleted');
    showDeleteAllDialog.value = false;
  } catch (error) {
    console.error('Error deleting all conversations:', error);
    toast.error('Failed to delete all conversations');
  }
};
const selectedThreads = ref<string[]>([]);
const showDeleteDialog = ref(false);
const showDeleteAllDialog = ref(false);

const hasConversations = computed(() => formattedThreads.value.length > 0);

function toggleSelectAll(checked: boolean | 'indeterminate') {
  if (checked === true) {
    selectedThreads.value = [...new Set([...selectedThreads.value, ...formattedThreads.value.map(thread => thread.id)])];
  } else {
    const threadIds = formattedThreads.value.map(thread => thread.id);
    selectedThreads.value = selectedThreads.value.filter(id => !threadIds.includes(id));
  }
}

function toggleSelection(id: string) {
  const index = selectedThreads.value.indexOf(id);
  if (index === -1) {
    selectedThreads.value.push(id);
  } else {
    selectedThreads.value.splice(index, 1);
  }
}

async function deleteSelected() {
  try {
    const deletePromises = selectedThreads.value.map((id) => deleteThread(id));

    await Promise.all(deletePromises);
    selectedThreads.value = [];
    showDeleteDialog.value = false;
  } catch (error) {
    console.error('Error deleting selected conversations:', error);
    toast.error('Failed to delete selected conversations');
  }
}

function exportSelected() {
  const selectedData = formattedThreads.value.filter((thread) => selectedThreads.value.includes(thread.id));

  const exportData = selectedData.map((thread) => ({
    id: thread.id,
    title: thread.title,
    date: thread.date,
    createdAt: thread.createdAt,
  }));

  const dataStr = JSON.stringify(exportData, null, 2);
  const dataBlob = new Blob([dataStr], { type: 'application/json' });
  const url = URL.createObjectURL(dataBlob);

  const a = document.createElement('a');
  a.href = url;
  a.download = `t4chat-history-export-${new Date().toISOString().slice(0, 10)}.json`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}
</script>

<template>
  <div class="flex flex-col gap-6">
    <div>
      <h1 class="text-2xl font-bold">History & Sync</h1>
      <p class="text-muted-foreground">Manage your chat history and synchronization preferences.</p>
    </div>

    <div class="flex max-h-[70vh] min-h-[400px] flex-col rounded-lg border">
      <div class="flex-shrink-0 border-b p-6">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold">Message History</h2>

          <div v-if="!threadsData" class="text-muted-foreground text-sm">Loading conversations...</div>
          <div class="flex gap-2" v-else-if="hasConversations">
            <Button variant="outline" size="sm" :disabled="selectedThreads.length === 0" @click="exportSelected">
              Export Selected
            </Button>

            <Dialog v-model:open="showDeleteDialog">
              <DialogTrigger as-child>
                <Button variant="destructive" size="sm" :disabled="selectedThreads.length === 0"> Delete Selected </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Delete Selected Conversations?</DialogTitle>
                  <DialogDescription>
                    This will permanently delete {{ selectedThreads.length }} selected conversation(s). This action
                    cannot be undone.
                  </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <Button variant="outline" @click="showDeleteDialog = false">Cancel</Button>
                  <Button variant="destructive" @click="deleteSelected">Delete</Button>
                </DialogFooter>
              </DialogContent>
            </Dialog>

            <Dialog v-model:open="showDeleteAllDialog">
              <DialogTrigger as-child>
                <Button variant="destructive" size="sm"> Delete All History </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Delete All Chat History?</DialogTitle>
                  <DialogDescription>
                    This will permanently delete all your chat history. This action cannot be undone.
                  </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <Button variant="outline" @click="showDeleteAllDialog = false">Cancel</Button>
                  <Button variant="destructive" @click="deleteAllThreads"> Delete All </Button>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </div>
        </div>
      </div>

      <div v-if="!threadsData" class="flex flex-1 items-center justify-center py-12 text-center">
        <p class="text-muted-foreground">Loading conversations...</p>
      </div>
      <div v-else-if="!hasConversations" class="flex flex-1 items-center justify-center py-12 text-center">
        <p class="text-muted-foreground">No conversation history found.</p>
      </div>

      <div v-else class="flex h-full flex-col overflow-hidden">
        <div
          class="bg-background/95 supports-[backdrop-filter]:bg-background/60 flex flex-shrink-0 items-center gap-2 border-b px-6 py-3 backdrop-blur"
        >
          <Checkbox
            :checked="formattedThreads.length > 0 && formattedThreads.every(t => selectedThreads.includes(t.id))"
            :indeterminate="formattedThreads.some(t => selectedThreads.includes(t.id)) && !formattedThreads.every(t => selectedThreads.includes(t.id))"
            @update:checked="toggleSelectAll"
            class="h-4 w-4"
          />
          <span class="text-muted-foreground text-sm font-medium">Select all</span>
        </div>

        <div class="flex-1 overflow-hidden">
          <ScrollArea class="h-full w-full">
            <div class="space-y-1 px-4 py-2">
              <div>
                <div
                  v-for="thread in formattedThreads"
                  :key="thread.id"
                  class="hover:bg-accent/50 group relative flex items-center rounded-md p-2"
                  :class="{ 'bg-accent/30': selectedThreads.includes(thread.id) }"
                >
                  <div class="flex w-full items-center gap-3">
                    <Checkbox
                      :checked="selectedThreads.includes(thread.id)"
                      @update:checked="() => toggleSelection(thread.id)"
                      class="h-4 w-4 flex-shrink-0"
                    />
                    <div class="min-w-0 flex-1">
                      <p class="truncate font-medium">{{ thread.title }}</p>
                      <div class="text-muted-foreground flex items-center gap-2 text-xs">
                        <span>{{ thread.date }}</span>
                        <span>•</span>
                        <span>{{ thread.messages }} message{{ thread.messages !== 1 ? 's' : '' }}</span>
                      </div>
                    </div>

                    <div class="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                      <Button variant="ghost" size="icon" class="h-7 w-7" @click.stop="() => deleteThread(thread.id)">
                        <TrashIcon class="h-3.5 w-3.5" />
                      </Button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </ScrollArea>
        </div>
      </div>
    </div>
  </div>
</template>
