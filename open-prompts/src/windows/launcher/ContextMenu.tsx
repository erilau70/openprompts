import { useEffect, useRef, type CSSProperties } from 'react';
import { Copy, ExternalLink } from 'lucide-react';

interface ContextMenuProps {
  x: number;
  y: number;
  onCopy: () => void;
  onOpenEditor: () => void;
  onClose: () => void;
}

export function ContextMenu({ x, y, onCopy, onOpenEditor, onClose }: ContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  // Close on click outside or escape
  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };

    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.stopPropagation(); // Don't dismiss the whole launcher
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClick);
    document.addEventListener('keydown', handleEsc, true);

    return () => {
      document.removeEventListener('mousedown', handleClick);
      document.removeEventListener('keydown', handleEsc, true);
    };
  }, [onClose]);

  // Adjust position to stay within viewport
  const MENU_WIDTH = 180;
  const MENU_HEIGHT = 88;
  const EDGE_PADDING = 8;
  const left = Math.min(x, Math.max(EDGE_PADDING, window.innerWidth - MENU_WIDTH - EDGE_PADDING));
  const top = Math.min(y, Math.max(EDGE_PADDING, window.innerHeight - MENU_HEIGHT - EDGE_PADDING));

  const style: CSSProperties = {
    position: 'fixed',
    left,
    top,
    zIndex: 1000,
  };

  return (
    <div className="context-menu" ref={menuRef} style={style} onContextMenu={(e) => e.preventDefault()}>
      <button
        className="context-menu-item"
        onClick={() => {
          onCopy();
          onClose();
        }}
      >
        <Copy size={14} />
        Copy to clipboard
      </button>
      <button
        className="context-menu-item"
        onClick={() => {
          onOpenEditor();
          onClose();
        }}
      >
        <ExternalLink size={14} />
        Open in editor
      </button>
    </div>
  );
}
