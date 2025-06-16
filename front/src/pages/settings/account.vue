<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { useClerk } from '@clerk/vue';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const clerk = useClerk();
const router = useRouter();
const isDeleting = ref(false);
const showDialog = ref(false);

async function deleteAccount() {
  if (!clerk.value) return;
  
  try {
    isDeleting.value = true;
    // Call Clerk's deleteAccount method
    await clerk.value.deleteAccount();
    // Redirect to home after successful deletion
    router.push('/');
  } catch (error) {
    console.error('Error deleting account:', error);
  } finally {
    isDeleting.value = false;
    showDialog.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col gap-6">
    <div>
      <h1 class="text-2xl font-bold">Account Settings</h1>
      <p class="text-muted-foreground">Manage your account settings and preferences.</p>
    </div>
    
    <div class="border-destructive/20 bg-destructive/5 rounded-md border p-6">
      <h2 class="text-destructive text-lg font-medium">Danger Zone</h2>
      <p class="text-destructive/80 mb-4 mt-1">Permanently delete your account and all associated data.</p>
      
      <Dialog v-model:open="showDialog">
        <DialogTrigger asChild>
          <Button variant="destructive">Delete Account</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete Account?</DialogTitle>
            <DialogDescription>
              This action cannot be undone. This will permanently delete your account and remove all your data from our servers.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" @click="showDialog = false">Cancel</Button>
            <Button variant="destructive" :disabled="isDeleting" @click="deleteAccount">
              {{ isDeleting ? 'Deleting...' : 'Delete Account' }}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  </div>
</template>
