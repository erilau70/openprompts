import { invoke } from '@tauri-apps/api/core';
import {
  disable as disableAutoStart,
  enable as enableAutoStart,
  isEnabled as isAutoStartEnabled,
} from '@tauri-apps/plugin-autostart';
import type { Prompt, PromptMetadata, PromptIndex, AppSettings } from '../types';

export const api = {
  // Data
  getIndex: () => invoke<PromptIndex>('get_index'),
  getFolders: () => invoke<string[]>('get_folders'),
  getPrompt: (id: string) => invoke<Prompt>('get_prompt', { id }),
  savePrompt: (prompt: Prompt) => invoke<PromptMetadata>('save_prompt', { prompt }),
  deletePrompt: (id: string) => invoke<void>('delete_prompt', { id }),
  addFolder: (name: string) => invoke<string[]>('add_folder', { name }),
  renameFolder: (oldName: string, newName: string) =>
    invoke<string[]>('rename_folder', { old: oldName, new: newName }),
  deleteFolder: (name: string) => invoke<string[]>('delete_folder', { name }),
  searchPrompts: (query: string) => invoke<PromptMetadata[]>('search_prompts', { query }),
  recordUsage: (id: string) => invoke<void>('record_usage', { id }),

  // Window
  pasteAndDismiss: (text: string) => invoke<void>('paste_and_dismiss', { text }),
  dismissWindow: () => invoke<void>('dismiss_window'),
  copyToClipboard: (text: string) => invoke<void>('copy_to_clipboard', { text }),
  openEditorWindow: () => invoke<void>('open_editor_window'),
  closeEditorWindow: () => invoke<void>('close_editor_window'),
  quitApp: () => invoke<void>('quit_app'),

  // Hotkey
  getCurrentHotkey: () => invoke<string>('get_current_hotkey'),
  setHotkey: (hotkey?: string) => invoke<string>('set_hotkey', { hotkey }),
  pauseHotkey: () => invoke<void>('pause_hotkey'),
  resumeHotkey: () => invoke<void>('resume_hotkey'),

  // Settings
  getSettings: () => invoke<AppSettings>('get_settings'),
  saveSettings: (settings: AppSettings) => invoke<AppSettings>('save_settings', { settings }),
  getAutoLaunchEnabled: () => isAutoStartEnabled(),
  setAutoLaunchEnabled: async (enabled: boolean) => {
    if (enabled) {
      await enableAutoStart();
      return;
    }
    await disableAutoStart();
  },
};