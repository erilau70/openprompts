import { create } from 'zustand';
import type { AppSettings } from '../types';
import { api } from '../services/ipc';

interface SettingsState {
  settings: AppSettings | undefined;
  loading: boolean;
  load: () => Promise<void>;
  save: (next: AppSettings) => Promise<void>;
  loadSettings: () => Promise<void>;
  saveSettings: (next: AppSettings) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: undefined,
  loading: false,

  load: async () => {
    set({ loading: true });
    try {
      const settings = await api.getSettings();
      set({ settings, loading: false });
    } catch (e) {
      console.error('Failed to load settings:', e);
      set({ loading: false });
    }
  },

  save: async (next: AppSettings) => {
    try {
      const saved = await api.saveSettings(next);
      set({ settings: saved });
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  },

  loadSettings: async () => {
    await useSettingsStore.getState().load();
  },

  saveSettings: async (next: AppSettings) => {
    await useSettingsStore.getState().save(next);
  },
}));