<script setup lang="ts">
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
import { useSettings } from '@/composables/settings';

const settings = useSettings();

const mainFontOptions = ['Inter', 'Roboto', 'Open Sans', 'Lato', 'Montserrat'];
const codeFontOptions = ['Fira Code', 'JetBrains Mono', 'Source Code Pro', 'Hack', 'Consolas'];
const presetTraits = ['friendly', 'witty', 'concise', 'curious', 'empathetic', 'creative', 'patient'];

function addTrait(trait: string) {
  if (settings.customization == null) return;
  if (!settings.customization.userTraits.includes(trait)) {
    settings.customization.userTraits = [...settings.customization.userTraits, trait];
  }
}
</script>

<template>
  <div class="flex flex-col gap-8">
    <div>
      <h1 class="text-2xl font-bold">Customization</h1>
      <p class="text-muted-foreground">Personalize your <Name /> experience.</p>
    </div>

    <div class="space-y-6" v-if="settings.customization != null">
      <div>
        <h2 class="text-xl font-semibold">Personal Information</h2>
        <div class="mt-4 space-y-4">
          <div class="space-y-2">
            <Label for="username">What should <Name /> call you?</Label>
            <Input
              id="username"
              v-model="settings.customization.userName"
              placeholder="Enter your name"
              class="max-w-sm"
            />
          </div>

          <div class="space-y-2">
            <Label for="occupation">What do you do?</Label>
            <Input
              id="occupation"
              v-model="settings.customization.userOccupation"
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
            <TagsInput v-model="settings.customization.userTraits" class="max-w-md">
              <TagsInputItem v-for="trait in settings.customization.userTraits" :key="trait" :value="trait">
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
                v-for="trait in presetTraits.filter((t) => !settings.customization?.userTraits.includes(t))"
                :key="trait"
                variant="outline"
                size="sm"
                @click="() => addTrait(trait)"
              >
                {{ trait }}
              </Button>
              <span
                v-if="presetTraits.every((t) => settings.customization?.userTraits.includes(t))"
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
                <Switch id="hidePersonalInfo" v-model:modelValue="settings.customization.hidePersonalInfo" />
              </div>
            </div>
            <p class="text-muted-foreground text-xs">
              Toggle this switch to hide/show personal information in the chat.
            </p>
          </div>

          <div class="space-y-2">
            <Label for="mainFont">Main Font</Label>
            <Select v-model="settings.customization.mainFont">
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
            <Select v-model="settings.customization.codeFont">
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

    <div v-else>
      <p class="text-muted-foreground">Loading customization settings...</p>
    </div>
  </div>
</template>
