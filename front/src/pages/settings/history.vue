<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
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
import { useLocalStorage } from '@vueuse/core';
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

// Mock conversation data for demo purposes
// In a real app, this would come from an API or local storage
const useHistoryStore = defineStore('history', () => {
  const conversations = useLocalStorage('history_conversations', [
    { id: '1', title: 'Chat about project ideas', date: '2023-06-10', messages: 12 },
    { id: '2', title: 'Code review assistance', date: '2023-06-12', messages: 8 },
    { id: '3', title: 'Help with JavaScript bugs', date: '2023-06-14', messages: 15 },
  ]);

  function deleteConversation(id: string) {
    const index = conversations.value.findIndex((conv) => conv.id === id);
    if (index !== -1) {
      conversations.value.splice(index, 1);
    }
  }

  function deleteAllConversations() {
    conversations.value = [];
  }

  return {
    conversations,
    deleteConversation,
    deleteAllConversations,
  };
});

const historyStore = useHistoryStore();
const selectedConversations = ref<string[]>([]);
const showDeleteDialog = ref(false);
const showDeleteAllDialog = ref(false);

const hasSelectedConversations = computed(() => selectedConversations.value.length > 0);

function toggleSelectAll(value: boolean) {
  if (value) {
    selectedConversations.value = historyStore.conversations.map((conv) => conv.id);
  } else {
    selectedConversations.value = [];
  }
}

function toggleSelection(id: string) {
  const index = selectedConversations.value.indexOf(id);
  if (index === -1) {
    selectedConversations.value.push(id);
  } else {
    selectedConversations.value.splice(index, 1);
  }
}

function deleteSelected() {
  selectedConversations.value.forEach((id) => {
    historyStore.deleteConversation(id);
  });
  selectedConversations.value = [];
  showDeleteDialog.value = false;
}

function exportSelected() {
  const selectedData = historyStore.conversations.filter((conv) => selectedConversations.value.includes(conv.id));

  // Create a JSON blob and trigger download
  const dataStr = JSON.stringify(selectedData, null, 2);
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

    <div class="rounded-lg border p-6">
      <div class="mb-6 flex items-center justify-between">
        <h2 class="text-xl font-semibold">Message History</h2>

        <div class="flex gap-2" v-if="historyStore.conversations.length > 0">
          <Button variant="outline" size="sm" :disabled="!hasSelectedConversations" @click="exportSelected">
            Export Selected
          </Button>

          <Dialog v-model:open="showDeleteDialog">
            <DialogTrigger asChild>
              <Button variant="destructive" size="sm" :disabled="!hasSelectedConversations"> Delete Selected </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Delete Selected Conversations?</DialogTitle>
                <DialogDescription>
                  This will permanently delete {{ selectedConversations.length }} selected conversation(s). This action
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
            <DialogTrigger asChild>
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
                <Button
                  variant="destructive"
                  @click="
                    () => {
                      historyStore.deleteAllConversations();
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
      </div>

      <div v-if="historyStore.conversations.length === 0" class="py-12 text-center">
        <p class="text-muted-foreground">No conversation history found.</p>
      </div>

      <div v-else>
        <div class="mb-2 grid grid-cols-[25px_1fr_150px_100px] gap-4 border-b py-2 font-medium">
          <div class="flex items-center">
            <Checkbox
              :checked="
                selectedConversations.length === historyStore.conversations.length &&
                historyStore.conversations.length > 0
              "
              @update:checked="toggleSelectAll"
            />
          </div>
          <span>Title</span>
          <span>Date</span>
          <span>Messages</span>
        </div>

        <Card v-for="conversation in historyStore.conversations" :key="conversation.id" class="mb-2">
          <CardContent class="grid grid-cols-[25px_1fr_150px_100px] gap-4 py-3">
            <div class="flex items-center">
              <Checkbox
                :checked="selectedConversations.includes(conversation.id)"
                @update:checked="() => toggleSelection(conversation.id)"
              />
            </div>
            <span class="truncate">{{ conversation.title }}</span>
            <span>{{ conversation.date }}</span>
            <span>{{ conversation.messages }}</span>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>
