<script setup lang="ts">
import Slider from '@/components/Slider.vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { type ThemeMode, useTheme } from '@/composables/theme';
import { computed } from 'vue';

const MIN_CHROMA = 0;
const MAX_CHROMA = 8;
const CHROMA_STEP = 0.1;
const CHROMA_FAST_STEP = 0.5;

const MIN_HUE = 0;
const MAX_HUE = 360;
const HUE_STEP = 1;
const HUE_FAST_STEP = 10;

const theme = useTheme();

const dark = computed(() => theme.mode === 'dark');
const system = computed(() => theme.mode === 'system');
const light = computed(() => theme.mode === 'light');

function setMode(mode: ThemeMode) {
  theme.mode = mode;
}
</script>

<template>
  <div class="theme-settings">
    <div class="theme-buttons">
      <Button variant="ghost" :active="dark" @click="setMode('dark')">Dark</Button>
      <Button variant="ghost" :active="system" @click="setMode('system')">System</Button>
      <Button variant="ghost" :active="light" @click="setMode('light')">Light</Button>
    </div>

    <div class="theme-slider">
      <Label>Set hue</Label>
      <div class="flex items-center gap-2">
        <Input
          class="bg-secondary w-24"
          type="number"
          placeholder="Set hue"
          auto-focus
          v-model="theme.hue"
          @keydown.enter="$emit('close')"
          :min="MIN_HUE"
          :max="MAX_HUE"
          :step="HUE_STEP"
          :fast-step="HUE_FAST_STEP"
        />
        <Slider v-model="theme.hue" class="hue" :min="MIN_HUE" :max="MAX_HUE" :step="HUE_STEP" />
      </div>
    </div>

    <div class="theme-slider">
      <Label>Set chroma</Label>
      <div class="flex items-center gap-2">
        <Input
          class="bg-secondary w-24"
          type="number"
          placeholder="Chroma"
          auto-focus
          @update:modelValue="theme.chroma = parseFloat($event.toString()) || 0"
          :modelValue="theme.chroma.toFixed(2)"
          @keydown.enter="$emit('close')"
          :min="MIN_CHROMA"
          :max="MAX_CHROMA"
          :step="CHROMA_STEP"
          :fast-step="CHROMA_FAST_STEP"
        />
        <Slider v-model="theme.chroma" class="chroma" :min="MIN_CHROMA" :max="MAX_CHROMA" :step="CHROMA_STEP" />
      </div>
    </div>
  </div>
</template>

<style>
.theme-settings {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;

  > .theme-buttons {
    display: flex;
    flex-direction: row;
    padding: var(--spacing);
    gap: var(--spacing);

    > * {
      flex: 1;
    }
  }

  > .theme-slider {
    display: flex;
    flex-direction: column;
    padding: calc(var(--spacing) * 2);
    gap: calc(var(--spacing) * 2);
  }
}

.hue {
  --h-l: 50;

  background: linear-gradient(
    to right,
    lch(var(--h-l) 140 0),
    lch(var(--h-l) 140 30),
    lch(var(--h-l) 140 60),
    lch(var(--h-l) 140 90),
    lch(var(--h-l) 140 120),
    lch(var(--h-l) 140 150),
    lch(var(--h-l) 140 180),
    lch(var(--h-l) 140 210),
    lch(var(--h-l) 140 240),
    lch(var(--h-l) 140 270),
    lch(var(--h-l) 140 300),
    lch(var(--h-l) 140 330),
    lch(var(--h-l) 140 360)
  );

  .light & {
    --h-l: 80;
  }
}

.chroma {
  --c-l: 50;

  background: linear-gradient(
    to right,
    lch(var(--c-l) 0 var(--h)),
    lch(var(--c-l) 20 var(--h)),
    lch(var(--c-l) 40 var(--h)),
    lch(var(--c-l) 60 var(--h)),
    lch(var(--c-l) 80 var(--h)),
    lch(var(--c-l) 100 var(--h))
  );

  .light & {
    --c-l: 80;
  }
}
</style>
