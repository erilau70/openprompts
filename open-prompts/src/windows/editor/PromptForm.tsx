import { useEditorStore } from '../../stores/editorStore';
import { MarkdownEditor } from './MarkdownEditor';
import { Trash2 } from 'lucide-react';

export function PromptForm() {
  const { activePrompt, folders, dirty, saveStatus, updateActive, saveActive, deleteActive } =
    useEditorStore();

  if (!activePrompt) return null;

  const statusText =
    saveStatus === 'saving'
      ? 'Saving...'
      : saveStatus === 'saved'
        ? 'Saved'
        : saveStatus === 'error'
          ? 'Save failed'
          : dirty
            ? '● Unsaved changes'
            : '';

  return (
    <>
      <div className="editor-toolbar">
        <div className="editor-toolbar-left">
          <span className={`save-status ${saveStatus} ${dirty && saveStatus === 'idle' ? 'dirty' : ''}`}>
            {statusText}
          </span>
        </div>
        <div className="editor-toolbar-right">
          <button className="toolbar-btn" onClick={saveActive}>
            Save
          </button>
          <button className="toolbar-btn danger" onClick={deleteActive} title="Delete prompt">
            <Trash2 size={14} />
          </button>
        </div>
      </div>

      <div className="editor-meta">
        <div className="meta-field" style={{ flex: 2 }}>
          <label>Name</label>
          <input
            value={activePrompt.name}
            onChange={(e) => updateActive({ name: e.target.value })}
            placeholder="Prompt name"
          />
        </div>
        <div className="meta-field" style={{ flex: 2 }}>
          <label>Description</label>
          <input
            value={activePrompt.description}
            onChange={(e) => updateActive({ description: e.target.value })}
            placeholder="Short description"
          />
        </div>
        <div className="meta-field" style={{ flex: 1 }}>
          <label>Folder</label>
          <select value={activePrompt.folder} onChange={(e) => updateActive({ folder: e.target.value })}>
            <option value="">Uncategorized</option>
            {folders.map((f) => (
              <option key={f} value={f}>
                {f}
              </option>
            ))}
          </select>
        </div>
      </div>

      <div className="editor-content">
        <MarkdownEditor key={activePrompt.id || 'new'} content={activePrompt.content} onChange={(content) => updateActive({ content })} />
      </div>
    </>
  );
}