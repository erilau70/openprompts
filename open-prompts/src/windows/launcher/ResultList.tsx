import { useEffect, useRef, useState } from 'react';
import { FileText } from 'lucide-react';
import { useLauncherStore } from '../../stores/launcherStore';
import { ContextMenu } from './ContextMenu';

export function ResultList() {
  const { results, selectedIndex, pasteSelected } = useLauncherStore();
  const listRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; index: number } | null>(null);

  // Scroll selected item into view
  useEffect(() => {
    if (!listRef.current) return;
    const selectedEl = listRef.current.children[selectedIndex] as HTMLElement | undefined;
    selectedEl?.scrollIntoView({ block: 'nearest' });
  }, [selectedIndex]);

  return (
    <div className="launcher-results" ref={listRef}>
      {results.map((prompt, index) => (
        <button
          key={prompt.id}
          className={`result-item ${index === selectedIndex ? 'selected' : ''}`}
          onClick={() => {
            useLauncherStore.setState({ selectedIndex: index });
            pasteSelected();
          }}
          onContextMenu={(e) => {
            e.preventDefault();
            useLauncherStore.setState({ selectedIndex: index });
            setContextMenu({ x: e.clientX, y: e.clientY, index });
          }}
          onMouseEnter={() => {
            useLauncherStore.setState({ selectedIndex: index });
          }}
        >
          <div className="result-icon">
            <FileText size={16} />
          </div>
          <div className="result-text">
            <div className="result-name">{prompt.name}</div>
            {prompt.description && <div className="result-desc">{prompt.description}</div>}
          </div>
          <div className="result-meta">
            {prompt.folder && <span className="result-folder-badge">{prompt.folder}</span>}
          </div>
        </button>
      ))}
      {contextMenu && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          onCopy={() => {
            void useLauncherStore.getState().copySelected();
          }}
          onOpenEditor={() => {
            void useLauncherStore.getState().openInEditor();
          }}
          onClose={() => {
            setContextMenu(null);
          }}
        />
      )}
    </div>
  );
}