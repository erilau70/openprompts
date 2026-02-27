import { useState } from 'react';
import { useEditorStore } from '../../stores/editorStore';
import { FolderOpen, Plus, FileText } from 'lucide-react';

export function Sidebar() {
  const {
    prompts,
    folders,
    activePromptId,
    folderFilter,
    selectPrompt,
    createPrompt,
    addFolder,
    setFolderFilter,
  } = useEditorStore();
  const [newFolderName, setNewFolderName] = useState('');
  const [showNewFolder, setShowNewFolder] = useState(false);

  const filteredPrompts = folderFilter ? prompts.filter((p) => p.folder === folderFilter) : prompts;

  const handleAddFolder = async () => {
    if (newFolderName.trim()) {
      await addFolder(newFolderName.trim());
      setNewFolderName('');
      setShowNewFolder(false);
    }
  };

  return (
    <div className="editor-sidebar">
      <div className="sidebar-header">
        <h2>Prompts</h2>
        <div style={{ display: 'flex', gap: 4 }}>
          <button
            className="icon-btn"
            onClick={() => createPrompt(folderFilter || undefined)}
            title="New prompt (Ctrl+N)"
          >
            <Plus size={16} />
          </button>
          <button className="icon-btn" onClick={() => setShowNewFolder(true)} title="New folder">
            <FolderOpen size={16} />
          </button>
        </div>
      </div>

      <div className="sidebar-folders">
        <button
          className={`folder-item ${!folderFilter ? 'active' : ''}`}
          onClick={() => setFolderFilter(undefined)}
        >
          <FileText size={14} />
          All Prompts ({prompts.length})
        </button>
        {folders.map((f) => (
          <button
            key={f}
            className={`folder-item ${folderFilter === f ? 'active' : ''}`}
            onClick={() => setFolderFilter(f)}
          >
            <FolderOpen size={14} />
            {f} ({prompts.filter((p) => p.folder === f).length})
          </button>
        ))}
        {showNewFolder && (
          <div style={{ padding: '4px 8px' }}>
            <input
              autoFocus
              placeholder="Folder name"
              value={newFolderName}
              onChange={(e) => setNewFolderName(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter') handleAddFolder();
                if (e.key === 'Escape') setShowNewFolder(false);
              }}
              onBlur={() => {
                if (!newFolderName.trim()) setShowNewFolder(false);
              }}
              style={{
                width: '100%',
                padding: '4px 8px',
                background: 'var(--bg-surface)',
                border: '1px solid var(--border-primary)',
                borderRadius: 'var(--radius-sm)',
                fontSize: 13,
                color: 'var(--text-primary)',
              }}
            />
          </div>
        )}
      </div>

      <div className="sidebar-prompts">
        {filteredPrompts.map((p) => (
          <button
            key={p.id}
            className={`prompt-item ${activePromptId === p.id ? 'active' : ''}`}
            onClick={() => selectPrompt(p.id)}
          >
            <span className="prompt-item-name">{p.name}</span>
            {p.description && <span className="prompt-item-desc">{p.description}</span>}
          </button>
        ))}
        {filteredPrompts.length === 0 && (
          <div style={{ padding: 16, textAlign: 'center', color: 'var(--text-muted)', fontSize: 13 }}>
            No prompts{folderFilter ? ` in "${folderFilter}"` : ''}
          </div>
        )}
      </div>
    </div>
  );
}