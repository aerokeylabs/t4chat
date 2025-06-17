<script setup lang="ts">
import { reactive, watch } from 'vue';
import { useQuery, useMutation } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import Name from '@/components/Name.vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import {
  TagsInput,
  TagsInputInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText,
} from '@/components/ui/tags-input';

const { data: settings } = useQuery(api.settings.getSettings);
const updateSettings = useMutation(api.settings.updateSettings);

const customizationState = reactive({
  userName: '',
  userOccupation: '',
  userTraits: [] as string[],
  hidePersonalInfo: false,
  mainFont: 'Inter',
  codeFont: 'Fira Code',
  isInitialized: false
});

watch(
  () => settings.value,
  (newSettings) => {
    if (newSettings) {
      customizationState.userName = newSettings.userName || '';
      customizationState.userOccupation = newSettings.userOccupation || '';
      customizationState.userTraits = newSettings.userTraits || [];
      customizationState.hidePersonalInfo = newSettings.hidePersonalInfo || false;
      customizationState.mainFont = newSettings.mainFont || 'Inter';
      customizationState.codeFont = newSettings.codeFont || 'Fira Code';
      customizationState.isInitialized = true;
    }
  },
  { immediate: true }
);

const saveSettings = async () => {
  if (!customizationState.isInitialized) return;
  
  const settingsToUpdate = {
    userName: customizationState.userName,
    userOccupation: customizationState.userOccupation,
    userTraits: [...customizationState.userTraits],
    hidePersonalInfo: customizationState.hidePersonalInfo,
    mainFont: customizationState.mainFont,
    codeFont: customizationState.codeFont,
  };
  
  try {
    await updateSettings({ settings: settingsToUpdate });
  } catch (error) {
    console.error('Failed to update settings:', error);
  }
};

watch(() => customizationState.userName, saveSettings);
watch(() => customizationState.userOccupation, saveSettings);
watch(() => [...customizationState.userTraits], saveSettings, { deep: true });
watch(() => customizationState.hidePersonalInfo, saveSettings, { immediate: true });
watch(() => customizationState.mainFont, saveSettings);
watch(() => customizationState.codeFont, saveSettings);

const mainFontOptions = ['Inter', 'Roboto', 'Open Sans', 'Lato', 'Montserrat'];
const codeFontOptions = ['Fira Code', 'JetBrains Mono', 'Source Code Pro', 'Hack', 'Consolas'];
const presetTraits = ['friendly', 'witty', 'concise', 'curious', 'empathetic', 'creative', 'patient'];

function addTrait(trait: string) {
  if (!customizationState.userTraits.includes(trait)) {
    customizationState.userTraits = [...customizationState.userTraits, trait];
  }
}
</script>

<template>
  <div class="flex flex-col gap-8">
    <div>
      <h1 class="text-2xl font-bold">Customization</h1>
      <p class="text-muted-foreground">Personalize your <Name /> experience.</p>
    </div>

    <div class="space-y-6">
      <div>
        <h2 class="text-xl font-semibold">Personal Information</h2>
        <div class="mt-4 space-y-4">
          <div class="space-y-2">
            <Label for="username">What should <Name /> call you?</Label>
            <Input id="username" v-model="customizationState.userName" placeholder="Enter your name" class="max-w-sm" />
          </div>

          <div class="space-y-2">
            <Label for="occupation">What do you do?</Label>
            <Input
              id="occupation"
              v-model="customizationState.userOccupation"
              placeholder="Engineer, student, etc."
              class="max-w-sm"
            />
          </div>
        </div>
      </div>

      <!-- Traits section -->
      <div>
        <h2 class="text-xl font-semibold"><Name /> Traits</h2>
        <p class="text-muted-foreground text-sm">What traits should <Name /> have?</p>

        <div class="mt-4 flex flex-col gap-4">
          <div>
            <p class="text-muted-foreground mb-2 text-sm">Add traits:</p>
            <TagsInput v-model="customizationState.userTraits" class="max-w-md">
              <TagsInputItem v-for="trait in customizationState.userTraits" :key="trait" :value="trait">
                <TagsInputItemText />
                <TagsInputItemDelete />
              </TagsInputItem>
              <TagsInputInput placeholder="Type and press Enter" />
            </TagsInput>
          </div>

          <div class="mt-4">
            <p class="text-muted-foreground mb-2 text-sm">Preset traits:</p>
            <div class="flex flex-wrap gap-2">
              <Button
                v-for="trait in presetTraits.filter((t) => !customizationState.userTraits.includes(t))"
                :key="trait"
                variant="outline"
                size="sm"
                @click="() => addTrait(trait)"
              >
                {{ trait }}
              </Button>
              <span
                v-if="presetTraits.every((t) => customizationState.userTraits.includes(t))"
                class="text-muted-foreground text-sm"
              >
                All preset traits added.
              </span>
            </div>
          </div>
        </div>
      </div>

      <div>
        <h2 class="text-xl font-semibold">Visual Options</h2>

        <div class="mt-4 space-y-4">
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <Label for="hidePersonalInfo" class="cursor-pointer">Hide Personal Information</Label>
              </div>
              <div class="flex items-center gap-2">
                <Switch 
                  id="hidePersonalInfo" 
                  v-model:modelValue="customizationState.hidePersonalInfo"
                />
              </div>
            </div>
            <p class="text-xs text-muted-foreground">
              Toggle this switch to hide/show personal information in the chat.
            </p>
          </div>

          <div class="space-y-2">
            <Label for="mainFont">Main Font</Label>
            <Select v-model="customizationState.mainFont">
              <SelectTrigger class="w-[200px]">
                <SelectValue placeholder="Select a font" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem v-for="font in mainFontOptions" :key="font" :value="font">
                  {{ font }}
                </SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div class="space-y-2">
            <Label for="codeFont">Code Font</Label>
            <Select v-model="customizationState.codeFont">
              <SelectTrigger class="w-[200px]">
                <SelectValue placeholder="Select a font" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem v-for="font in codeFontOptions" :key="font" :value="font">
                  {{ font }}
                </SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
