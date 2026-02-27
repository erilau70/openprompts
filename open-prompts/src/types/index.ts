export interface PromptMetadata {
  id: string;
  name: string;
  folder: string;
  description: string;
  filename: string;
  useCount: number;
  lastUsed: string | null;
  created: string;
  updated: string;
  icon?: string;
  color?: string;
}

export interface Prompt extends PromptMetadata {
  content: string;
}

export interface FolderMeta {
  name: string;
  icon?: string;
  color?: string;
}

export interface PromptIndex {
  prompts: PromptMetadata[];
  folders: string[];
  folderMeta?: Record<string, FolderMeta>;
  seeded: boolean;
}

export interface GeneralSettings {
  autoLaunch: boolean;
  hotkey: string;
  editorAlwaysOnTop: boolean;
  welcomeScreenDismissed: boolean;
}

export interface AppearanceSettings {
  theme: 'dark' | 'light' | 'auto';
  accentColor: string;
}

export interface AppSettings {
  general: GeneralSettings;
  appearance: AppearanceSettings;
}