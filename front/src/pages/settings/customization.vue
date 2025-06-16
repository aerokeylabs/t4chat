<script setup lang="ts">
import {
  TagsInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText,
  TagsInputInput,
} from '@/components/ui/tags-input';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import { defineStore } from 'pinia';
import { useLocalStorage } from '@vueuse/core';

// Customization store
const useCustomizationStore = defineStore('customization', () => {
  const userName = useLocalStorage('customization_userName', '');
  const userOccupation = useLocalStorage('customization_userOccupation', '');
  const userTraits = useLocalStorage('customization_userTraits', [] as string[]);
  const hidePersonalInfo = useLocalStorage('customization_hidePersonalInfo', false);
  const mainFont = useLocalStorage('customization_mainFont', 'Inter');
  const codeFont = useLocalStorage('customization_codeFont', 'Fira Code');

  return {
    userName,
    userOccupation,
    userTraits,
    hidePersonalInfo,
    mainFont,
    codeFont,
  };
});

const customization = useCustomizationStore();

const mainFontOptions = ['Inter', 'Roboto', 'Open Sans', 'Lato', 'Montserrat'];
const codeFontOptions = ['Fira Code', 'JetBrains Mono', 'Source Code Pro', 'Hack', 'Consolas'];

const presetTraits = ['friendly', 'witty', 'concise', 'curious', 'empathetic', 'creative', 'patient'];

// No need for addTrait function since TagsInput will handle adding tags
</script>

<template>
  <div class="flex flex-col gap-8">
    <!-- Header section -->
    <div>
      <h1 class="text-2xl font-bold">Customization</h1>
      <p class="text-muted-foreground">Personalize your T4Chat experience.</p>
    </div>

    <!-- Main content -->
    <div class="space-y-6">
      <!-- Personal Information section -->
      <div>
        <h2 class="text-xl font-semibold">Personal Information</h2>
        <div class="mt-4 space-y-4">
          <div class="space-y-2">
            <Label for="username">What should T4 Chat call you?</Label>
            <Input id="username" v-model="customization.userName" placeholder="Enter your name" class="max-w-sm" />
          </div>

          <div class="space-y-2">
            <Label for="occupation">What do you do?</Label>
            <Input
              id="occupation"
              v-model="customization.userOccupation"
              placeholder="Engineer, student, etc."
              class="max-w-sm"
            />
          </div>
        </div>
      </div>

      <!-- Traits section -->
      <div>
        <h2 class="text-xl font-semibold">T4Chat Traits</h2>
        <p class="text-muted-foreground text-sm">What traits should T4Chat have?</p>

        <div class="mt-4 flex flex-col gap-4">
          <div>
            <p class="text-muted-foreground mb-2 text-sm">Add traits:</p>
            <TagsInput v-model="customization.userTraits" class="max-w-md">
              <TagsInputItem v-for="trait in customization.userTraits" :key="trait" :value="trait">
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
                v-for="trait in presetTraits.filter((t) => !customization.userTraits.includes(t))"
                :key="trait"
                variant="outline"
                size="sm"
                @click="() => customization.userTraits.push(trait)"
              >
                {{ trait }}
              </Button>
              <span
                v-if="presetTraits.every((t) => customization.userTraits.includes(t))"
                class="text-muted-foreground text-sm"
              >
                All preset traits added.
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Visual Options section -->
      <div>
        <h2 class="text-xl font-semibold">Visual Options</h2>

        <div class="mt-4 space-y-4">
          <div class="flex items-center justify-between">
            <Label for="hidePersonalInfo" class="cursor-pointer">Hide Personal Information</Label>
            <Switch id="hidePersonalInfo" v-model:checked="customization.hidePersonalInfo" />
          </div>

          <div class="space-y-2">
            <Label for="mainFont">Main Font</Label>
            <Select v-model="customization.mainFont">
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
            <Select v-model="customization.codeFont">
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
