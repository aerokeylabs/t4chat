<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { useConvex, useMutation, useReactiveQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import type { Id } from '@/convex/_generated/dataModel';
import type { AssistantMessage, Message } from '@/lib/types/convex';
import { TrashIcon } from 'lucide-vue-next';
import moment from 'moment';
import { computed, ref } from 'vue';
import { toast } from 'vue-sonner';

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

const convex = useConvex();

const getMessagesForThread = async (threadId: string): Promise<Message[]> => {
  try {
    const result = await convex.query(api.messages.getByThreadId, { threadId: threadId as Id<'threads'> });
    return (result?.messages || []) as unknown as Message[];
  } catch (error) {
    console.error('Error fetching messages for thread:', threadId, error);
    return [];
  }
};

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
const selectedThreads = ref<Id<'threads'>[]>([]);
const showDeleteDialog = ref(false);
const showDeleteAllDialog = ref(false);

const hasConversations = computed(() => formattedThreads.value.length > 0);

const selectAll = computed({
  get: () =>
    formattedThreads.value.length > 0 && formattedThreads.value.every((t) => selectedThreads.value.includes(t.id)),
  set: (value: boolean) => {
    if (value) {
      const newSelections = formattedThreads.value.map((t) => t.id).filter((id) => !selectedThreads.value.includes(id));
      selectedThreads.value = [...selectedThreads.value, ...newSelections];
    } else {
      const threadIds = new Set(formattedThreads.value.map((t) => t.id));
      selectedThreads.value = selectedThreads.value.filter((id) => !threadIds.has(id));
    }
  },
});

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

async function exportSelected() {
  const selectedData = formattedThreads.value.filter((thread) => selectedThreads.value.includes(thread.id));

  try {
    toast.info('Preparing export, this may take a moment...');

    const exportData = [];

    for (const thread of selectedData) {
      const messages = await getMessagesForThread(thread.id);

      const formattedMessages = messages.map((msg) => {
        const message = msg as unknown as Message;

        const textContent = message.parts
          .filter((part): part is { type: 'text'; text: string } => part.type === 'text')
          .map((part) => part.text)
          .join('\n');

        const baseMessage = {
          id: message._id,
          role: message.role,
          text: textContent,
          createdAt: new Date(message._creationTime).toISOString(),
        };

        if (message.role === 'assistant') {
          const assistantMsg = message as AssistantMessage;
          return {
            ...baseMessage,
            status: assistantMsg.status,
            model: assistantMsg.model,
            ...(assistantMsg.modelParams ? { modelParams: assistantMsg.modelParams } : {}),
          };
        }
        return baseMessage;
      });

      exportData.push({
        id: thread.id,
        title: thread.title,
        date: thread.date,
        createdAt: new Date(thread.createdAt).toISOString(),
        messageCount: formattedMessages.length,
        messages: formattedMessages,
      });
    }

    const dataStr = JSON.stringify(exportData, null, 2);
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);

    const a = document.createElement('a');
    a.href = url;
    a.download = `t4chat-export-${new Date().toISOString().slice(0, 10)}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    toast.success('Export completed successfully');
  } catch (error) {
    console.error('Error during export:', error);
    toast.error('Failed to export conversations');
  }
}
</script>

<template>
  <section class="flex flex-col">
    <h1 class="text-2xl font-bold">History & Sync</h1>
    <p class="text-muted-foreground">Manage your chat history and synchronization preferences.</p>

    <h2 class="mt-6 text-xl font-semibold">Message History</h2>

    <div class="message-history-table mt-4 flex flex-col rounded-lg border">
      <div v-if="!threadsData" class="text-muted-foreground text-sm">Loading conversations...</div>

      <div v-if="!threadsData" class="flex flex-1 items-center justify-center py-12 text-center">
        <p class="text-muted-foreground">Loading conversations...</p>
      </div>
      <div v-else-if="!hasConversations" class="flex flex-1 items-center justify-center py-12 text-center">
        <p class="text-muted-foreground">No conversation history found.</p>
      </div>

      <div v-else class="flex h-full flex-col overflow-hidden">
        <div class="flex flex-shrink-0 items-center justify-between gap-2 border-b py-3 pl-5 pr-3">
          <div class="flex items-center gap-2">
            <Checkbox
              :model-value="selectAll"
              @update:model-value="
                (val: boolean | 'indeterminate') => {
                  if (val !== 'indeterminate') selectAll = val;
                }
              "
              :indeterminate="
                formattedThreads.some((t) => selectedThreads.includes(t.id)) &&
                !formattedThreads.every((t) => selectedThreads.includes(t.id))
              "
              class="h-4 w-4"
            />
            <span class="text-muted-foreground text-sm font-medium">Select all</span>
          </div>

          <div class="flex gap-2">
            <Button variant="outline" size="sm" :disabled="selectedThreads.length === 0" @click="exportSelected">
              Export Selected
            </Button>

            <Dialog v-model:open="showDeleteDialog">
              <DialogTrigger as-child>
                <Button variant="destructive" size="sm" :disabled="selectedThreads.length === 0">
                  Delete Selected
                </Button>
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

        <div class="custom-scrollbar flex h-full min-h-0 flex-1 flex-col gap-1 overflow-y-auto p-1">
          <div
            v-for="thread in formattedThreads"
            :key="thread.id"
            class="hover:bg-accent/50 group relative flex w-full items-center rounded-md p-2 px-4"
            :class="{ 'bg-accent/30': selectedThreads.includes(thread.id) }"
          >
            <div class="flex w-full items-center gap-3">
              <Checkbox
                :model-value="selectedThreads.includes(thread.id)"
                @update:model-value="
                  (checked) => {
                    if (checked) {
                      if (!selectedThreads.includes(thread.id)) {
                        selectedThreads.push(thread.id);
                      }
                    } else {
                      const index = selectedThreads.indexOf(thread.id);
                      if (index !== -1) {
                        selectedThreads.splice(index, 1);
                      }
                    }
                  }
                "
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
    </div>
  </section>
</template>

<style>
.message-history-table {
  max-height: 600px;
}
</style>
