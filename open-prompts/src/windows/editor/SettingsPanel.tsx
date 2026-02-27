import { useCallback, useEffect, useState } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Keyboard, Palette, Pin, Settings } from 'lucide-react';
import { api } from '../../services/ipc';
import { useSettingsStore } from '../../stores/settingsStore';

function formatAccelerator(event: KeyboardEvent): string {
  const modifiers: string[] = [];

  if (event.ctrlKey || event.metaKey) modifiers.push('CommandOrControl');
  if (event.shiftKey) modifiers.push('Shift');
  if (event.altKey) modifiers.push('Alt');

  const key = event.key;
  if (['Control', 'Shift', 'Alt', 'Meta'].includes(key)) return '';

  let normalized = key;
  if (key === ' ') normalized = 'Space';
  else if (key === 'Escape') normalized = 'Esc';
  else if (key.length === 1) normalized = key.toUpperCase();

  return [...modifiers, normalized].join('+');
}

export function SettingsPanel() {
  const { settings, save } = useSettingsStore();
  const [isRecording, setIsRecording] = useState(false);
  const [recordedHotkey, setRecordedHotkey] = useState('');

  const startRecording = useCallback(async () => {
    try {
      await api.pauseHotkey();
      setRecordedHotkey('');
      setIsRecording(true);
    } catch (e) {
      console.error('Failed to pause hotkey before recording:', e);
    }
  }, []);

  const cancelRecording = useCallback(async () => {
    setIsRecording(false);
    setRecordedHotkey('');
    try {
      await api.resumeHotkey();
    } catch (e) {
      console.error('Failed to resume hotkey after cancel:', e);
    }
  }, []);

  const saveHotkey = useCallback(async () => {
    if (!settings || !recordedHotkey) return;

    try {
      await api.setHotkey(recordedHotkey);
      await save({
        ...settings,
        general: {
          ...settings.general,
          hotkey: recordedHotkey,
        },
      });
      await api.resumeHotkey();
      setIsRecording(false);
      setRecordedHotkey('');
    } catch (e) {
      console.error('Failed to save hotkey:', e);
      try {
        await api.resumeHotkey();
      } catch (resumeError) {
        console.error('Failed to resume hotkey after save failure:', resumeError);
      }
    }
  }, [recordedHotkey, save, settings]);

  useEffect(() => {
    if (!isRecording) return;

    const handler = (event: KeyboardEvent) => {
      event.preventDefault();
      event.stopPropagation();

      if (event.key === 'Escape') {
        void cancelRecording();
        return;
      }

      const accelerator = formatAccelerator(event);
      if (accelerator) {
        setRecordedHotkey(accelerator);
      }
    };

    window.addEventListener('keydown', handler, true);
    return () => window.removeEventListener('keydown', handler, true);
  }, [cancelRecording, isRecording]);

  const handleThemeChange = useCallback(
    async (theme: 'dark' | 'light' | 'auto') => {
      if (!settings) return;
      document.documentElement.setAttribute('data-theme', theme);
      await save({
        ...settings,
        appearance: {
          ...settings.appearance,
          theme,
        },
      });
    },
    [save, settings],
  );

  const handleAlwaysOnTop = useCallback(
    async (value: boolean) => {
      if (!settings) return;
      try {
        await getCurrentWindow().setAlwaysOnTop(value);
        await save({
          ...settings,
          general: {
            ...settings.general,
            editorAlwaysOnTop: value,
          },
        });
      } catch (e) {
        console.error('Failed to update always-on-top:', e);
      }
    },
    [save, settings],
  );

  if (!settings) return null;

  return (
    <div className="settings-panel">
      <h2 className="settings-title">
        <Settings size={18} />
        Settings
      </h2>

      <div className="settings-section">
        <label className="settings-label">
          <Keyboard size={14} />
          Global Hotkey
        </label>
        {isRecording ? (
          <div className="hotkey-recorder">
            <div className="hotkey-display recording">{recordedHotkey || 'Press key combo (Esc cancels)...'}</div>
            <div className="hotkey-actions">
              <button className="btn-sm btn-primary" onClick={saveHotkey} disabled={!recordedHotkey}>
                Save
              </button>
              <button className="btn-sm btn-ghost" onClick={() => void cancelRecording()}>
                Cancel
              </button>
            </div>
          </div>
        ) : (
          <div className="hotkey-recorder">
            <div className="hotkey-display">{settings.general.hotkey}</div>
            <button className="btn-sm btn-ghost" onClick={() => void startRecording()}>
              Change
            </button>
          </div>
        )}
      </div>

      <div className="settings-section">
        <label className="settings-label">
          <Palette size={14} />
          Theme
        </label>
        <select
          className="settings-select"
          value={settings.appearance.theme}
          onChange={(event) =>
            void handleThemeChange(event.target.value as 'dark' | 'light' | 'auto')
          }
        >
          <option value="dark">Dark</option>
          <option value="light">Light</option>
          <option value="auto">Auto</option>
        </select>
      </div>

      <div className="settings-section">
        <label className="settings-label">
          <Pin size={14} />
          Always on Top
        </label>
        <label className="settings-toggle">
          <input
            type="checkbox"
            checked={settings.general.editorAlwaysOnTop}
            onChange={(event) => void handleAlwaysOnTop(event.target.checked)}
          />
          <span>Keep editor above other windows</span>
        </label>
      </div>
    </div>
  );
}
