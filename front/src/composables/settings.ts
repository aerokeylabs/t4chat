import { useMutation, useQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import { syncRef, useDebounceFn } from '@vueuse/core';
import { acceptHMRUpdate, defineStore } from 'pinia';
import { computed, customRef, ref, toRaw, watch } from 'vue';

export type CustomizationSettings = {
  userName: string;
  userOccupation: string;
  userTraits: string[];
  hidePersonalInfo: boolean;
  mainFont: string;
  codeFont: string;

  _updateFromApi?: boolean;
};

function setFonts(mainFont: string, codeFont: string) {
  if (mainFont === 'system') document.documentElement.style.setProperty('--font-sans', 'system-ui, sans-serif');
  else document.documentElement.style.setProperty('--font-sans', `"${mainFont}", sans-serif`);

  if (codeFont === 'system') document.documentElement.style.setProperty('--font-mono', 'monospace');
  else document.documentElement.style.setProperty('--font-mono', `"${codeFont}", monospace`);

  console.debug('Fonts set:', { mainFont, codeFont });
}

export const useSettings = defineStore('settings', () => {
  const settingsMutation = useMutation(api.settings.updateSettings);

  const { data, error } = useQuery(api.settings.getSettings);

  watch(error, (err) => {
    if (err) {
      console.error('Error fetching settings:', err);
    }
  });

  function updateCustomization(settings?: Partial<CustomizationSettings> | null) {
    if (settings == null) return;

    delete settings._updateFromApi;

    return settingsMutation({ settings });
  }

  const debouncedUpdateCustomization = useDebounceFn(updateCustomization, 500);

  const innerCustomization = customRef<CustomizationSettings | null>((track, trigger) => {
    let value: CustomizationSettings | null = null;

    return {
      get() {
        track();
        return value;
      },
      set(newValue) {
        if (newValue?._updateFromApi === true) console.debug('Skipping committing update from API:', newValue);
        else console.debug('committing new value:', newValue);
        if (newValue != null && newValue._updateFromApi !== true) debouncedUpdateCustomization(toRaw(newValue));

        value = newValue;

        if (value != null) value._updateFromApi = false;

        trigger();
      },
    };
  });

  watch(
    data,
    (data) => {
      if (data == null) return null;
      console.debug('data updated:', data);

      const { userName, userOccupation, userTraits, hidePersonalInfo, mainFont, codeFont } = data;

      innerCustomization.value = {
        userName,
        userOccupation,
        userTraits,
        hidePersonalInfo,
        mainFont,
        codeFont,
        _updateFromApi: true,
      };
    },
    { immediate: true, deep: true },
  );

  const hidePersonalInfo = computed(() => innerCustomization.value?.hidePersonalInfo ?? true);

  const customization = ref<CustomizationSettings | null>(null);

  syncRef(customization, innerCustomization, { deep: true });

  watch(
    customization,
    () => setFonts(customization.value?.mainFont ?? 'Inter', customization.value?.codeFont ?? 'Fira Code'),
    { deep: true, immediate: true },
  );

  return {
    customization,
    hidePersonalInfo,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettings, import.meta.hot));
}
