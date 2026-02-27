import { create } from 'zustand';
import type { PromptMetadata, Prompt } from '../types';
import { api } from '../services/ipc';

let autoSaveTimer: ReturnType<typeof setTimeout> | null = null;

function clearAutoSaveTimer() {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
    autoSaveTimer = null;
  }
}

function scheduleAutoSave(saveFn: () => Promise<void>) {
  clearAutoSaveTimer();
  autoSaveTimer = setTimeout(() => {
    void saveFn();
    autoSaveTimer = null;
  }, 500);
}

interface EditorState {
  prompts: PromptMetadata[];
  folders: string[];
  activePromptId: string | undefined;
  activePrompt: Prompt | undefined;
  dirty: boolean;
  saveStatus: 'idle' | 'saving' | 'saved' | 'error';
  folderFilter: string | undefined;

  loadInitial: () => Promise<void>;
  selectPrompt: (id: string) => Promise<void>;
  updateActive: (fields: Partial<Prompt>) => void;
  saveActive: () => Promise<void>;
  deleteActive: () => Promise<void>;
  createPrompt: (folder?: string) => void;
  addFolder: (name: string) => Promise<void>;
  renameFolder: (oldName: string, newName: string) => Promise<void>;
  deleteFolder: (name: string) => Promise<void>;
  setFolderFilter: (folder: string | undefined) => void;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  prompts: [],
  folders: [],
  activePromptId: undefined,
  activePrompt: undefined,
  dirty: false,
  saveStatus: 'idle',
  folderFilter: undefined,

  loadInitial: async () => {
    try {
      const index = await api.getIndex();
      set({ prompts: index.prompts, folders: index.folders });
    } catch (e) {
      console.error('Failed to load index:', e);
    }
  },

  selectPrompt: async (id: string) => {
    try {
      clearAutoSaveTimer();
      if (get().dirty) {
        await get().saveActive();
      }
      const prompt = await api.getPrompt(id);
      set({ activePromptId: id, activePrompt: prompt, dirty: false, saveStatus: 'idle' });
    } catch (e) {
      console.error('Failed to load prompt:', e);
    }
  },

  updateActive: (fields: Partial<Prompt>) => {
    const { activePrompt } = get();
    if (!activePrompt) return;
    set({
      activePrompt: { ...activePrompt, ...fields },
      dirty: true,
      saveStatus: 'idle',
    });
    scheduleAutoSave(() => get().saveActive());
  },

  saveActive: async () => {
    clearAutoSaveTimer();
    const { activePrompt } = get();
    if (!activePrompt) return;

    set({ saveStatus: 'saving' });
    try {
      const saved = await api.savePrompt(activePrompt);
      const index = await api.getIndex();
      set({
        prompts: index.prompts,
        folders: index.folders,
        activePromptId: saved.id,
        dirty: false,
        saveStatus: 'saved',
      });
      // Reset status after 2s
      setTimeout(() => {
        if (get().saveStatus === 'saved') set({ saveStatus: 'idle' });
      }, 2000);
    } catch (e) {
      console.error('Failed to save prompt:', e);
      set({ saveStatus: 'error' });
    }
  },

  deleteActive: async () => {
    const { activePromptId } = get();
    if (!activePromptId) return;
    try {
      await api.deletePrompt(activePromptId);
      const index = await api.getIndex();
      set({
        prompts: index.prompts,
        folders: index.folders,
        activePromptId: undefined,
        activePrompt: undefined,
        dirty: false,
        saveStatus: 'idle',
      });
    } catch (e) {
      console.error('Failed to delete prompt:', e);
    }
  },

  createPrompt: (folder?: string) => {
    const now = new Date().toISOString();
    const newPrompt: Prompt = {
      id: '',
      name: 'New Prompt',
      folder: folder || '',
      description: '',
      filename: '',
      useCount: 0,
      lastUsed: null,
      created: now,
      updated: now,
      content: '',
    };
    set({ activePrompt: newPrompt, activePromptId: undefined, dirty: true, saveStatus: 'idle' });
  },

  addFolder: async (name: string) => {
    try {
      const folders = await api.addFolder(name);
      set({ folders });
    } catch (e) {
      console.error('Failed to add folder:', e);
    }
  },

  renameFolder: async (oldName: string, newName: string) => {
    try {
      const folders = await api.renameFolder(oldName, newName);
      set({ folders });
    } catch (e) {
      console.error('Failed to rename folder:', e);
    }
  },

  deleteFolder: async (name: string) => {
    try {
      const folders = await api.deleteFolder(name);
      const index = await api.getIndex();
      set({ folders, prompts: index.prompts });
    } catch (e) {
      console.error('Failed to delete folder:', e);
    }
  },

  setFolderFilter: (folder: string | undefined) => {
    set({ folderFilter: folder });
  },
}));