import { create } from 'zustand';
import type { PromptMetadata } from '../types';
import { api } from '../services/ipc';

interface LauncherState {
  query: string;
  results: PromptMetadata[];
  selectedIndex: number;
  isLoading: boolean;

  setQuery: (q: string) => Promise<void>;
  moveSelection: (delta: number) => void;
  pasteSelected: () => Promise<void>;
  copySelected: () => Promise<void>;
  dismiss: () => Promise<void>;
  openInEditor: () => Promise<void>;
  refresh: () => Promise<void>;
}

export const useLauncherStore = create<LauncherState>((set, get) => ({
  query: '',
  results: [],
  selectedIndex: 0,
  isLoading: false,

  setQuery: async (q: string) => {
    set({ query: q, isLoading: true });
    try {
      const results = await api.searchPrompts(q);
      set({ results, selectedIndex: 0, isLoading: false });
    } catch (e) {
      console.error('Search failed:', e);
      set({ isLoading: false });
    }
  },

  moveSelection: (delta: number) => {
    const { results, selectedIndex } = get();
    if (results.length === 0) return;
    const next = Math.max(0, Math.min(results.length - 1, selectedIndex + delta));
    set({ selectedIndex: next });
  },

  pasteSelected: async () => {
    const { results, selectedIndex } = get();
    const selected = results[selectedIndex];
    if (!selected) return;

    try {
      const prompt = await api.getPrompt(selected.id);
      await api.recordUsage(selected.id);
      await api.pasteAndDismiss(prompt.content);
      // Reset state for next invocation
      set({ query: '', results: [], selectedIndex: 0 });
    } catch (e) {
      console.error('Paste failed:', e);
    }
  },

  copySelected: async () => {
    const { results, selectedIndex } = get();
    const selected = results[selectedIndex];
    if (!selected) return;

    try {
      const prompt = await api.getPrompt(selected.id);
      await api.copyToClipboard(prompt.content);
    } catch (e) {
      console.error('Copy failed:', e);
    }
  },

  dismiss: async () => {
    set({ query: '', results: [], selectedIndex: 0 });
    await api.dismissWindow();
  },

  openInEditor: async () => {
    await api.openEditorWindow();
  },

  refresh: async () => {
    const { query } = get();
    set({ isLoading: true });
    try {
      const results = await api.searchPrompts(query);
      set({ results, selectedIndex: 0, isLoading: false });
    } catch (e) {
      console.error('Refresh failed:', e);
      set({ isLoading: false });
    }
  },
}));