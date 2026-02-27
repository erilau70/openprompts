import { useEffect, useCallback, useState } from 'react';
import { useEditorStore } from '../../stores/editorStore';
import { useSettingsStore } from '../../stores/settingsStore';
import { Sidebar } from './Sidebar';
import { PromptForm } from './PromptForm';
import { SettingsPanel } from './SettingsPanel';
import { Settings } from 'lucide-react';
import '../../styles/editor.css';

export function EditorApp() {
  const { activePrompt, loadInitial, saveActive, createPrompt } = useEditorStore();
  const { settings, load } = useSettingsStore();
  const [showSettings, setShowSettings] = useState(false);

  useEffect(() => {
    loadInitial();
  }, [loadInitial]);

  useEffect(() => {
    if (!settings) {
      void load();
      return;
    }
    document.documentElement.setAttribute('data-theme', settings.appearance.theme);
  }, [load, settings]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === 's') {
        e.preventDefault();
        saveActive();
      }
      if (e.ctrlKey && e.key === 'n') {
        e.preventDefault();
        createPrompt();
      }
    },
    [saveActive, createPrompt],
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  return (
    <div className="editor-app">
      <Sidebar />
      <div className="editor-main">
        <div className="editor-app-toolbar">
          <div />
          <button
            className={`toolbar-btn icon-btn ${showSettings ? 'active' : ''}`}
            onClick={() => setShowSettings((prev) => !prev)}
            title={showSettings ? 'Close settings' : 'Open settings'}
            aria-label={showSettings ? 'Close settings' : 'Open settings'}
          >
            <Settings size={14} />
          </button>
        </div>
        {showSettings ? (
          <SettingsPanel />
        ) : activePrompt ? (
          <PromptForm />
        ) : (
          <div className="editor-empty">Select a prompt or press Ctrl+N to create one</div>
        )}
      </div>
    </div>
  );
}