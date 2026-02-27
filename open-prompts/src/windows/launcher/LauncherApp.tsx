import { useEffect, useRef } from 'react';
import { useLauncherStore } from '../../stores/launcherStore';
import { SearchInput } from './SearchInput';
import { ResultList } from './ResultList';
import '../../styles/launcher.css';

export function LauncherApp() {
  const { results, query, isLoading, setQuery, moveSelection, pasteSelected, dismiss, refresh, openInEditor } =
    useLauncherStore();
  const initialized = useRef(false);

  // Load prompts on mount (empty query = recency sorted)
  useEffect(() => {
    if (!initialized.current) {
      initialized.current = true;
      refresh();
    }
  }, [refresh]);

  // Also refresh when the window becomes visible again (re-triggered by Tauri show)
  useEffect(() => {
    const handleFocus = () => {
      // Reset query and refresh on each launcher activation
      setQuery('');
    };
    window.addEventListener('focus', handleFocus);
    return () => window.removeEventListener('focus', handleFocus);
  }, [setQuery]);

  // Global keyboard handler
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'e') {
        e.preventDefault();
        openInEditor();
        return;
      }

      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          moveSelection(1);
          break;
        case 'ArrowUp':
          e.preventDefault();
          moveSelection(-1);
          break;
        case 'Enter':
          e.preventDefault();
          pasteSelected();
          break;
        case 'Escape':
          e.preventDefault();
          dismiss();
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [moveSelection, pasteSelected, dismiss, openInEditor]);

  const hasResults = results.length > 0;
  const isEmptyQuery = query.trim() === '';
  const showLoadingState = isLoading && !hasResults;
  const showNoPromptsYet = !isLoading && isEmptyQuery && !hasResults;
  const showNoMatches = !isLoading && !isEmptyQuery && !hasResults;

  return (
    <div className="launcher">
      <SearchInput />
      {hasResults ? (
        <ResultList />
      ) : showLoadingState ? (
        <div className="launcher-empty" aria-live="polite">
          <span>Loading prompts…</span>
        </div>
      ) : showNoPromptsYet ? (
        <div className="launcher-empty">
          <span>No prompts yet — press [Ctrl+E] or right-click to open the editor</span>
          <span className="launcher-empty-hint">Press Escape to close launcher</span>
        </div>
      ) : showNoMatches ? (
        <div className="launcher-empty">
          <span>No matches for “{query}”</span>
          <span className="launcher-empty-hint">Try a different keyword</span>
        </div>
      ) : (
        <></>
      )}
      <div className="launcher-footer">
        <div className="launcher-footer-shortcuts">
          <span className="shortcut-hint">
            <kbd>↑↓</kbd> navigate
          </span>
          <span className="shortcut-hint">
            <kbd>↵</kbd> paste
          </span>
          <span className="shortcut-hint">
            <kbd>ctrl</kbd> + <kbd>E</kbd> editor
          </span>
          <span className="shortcut-hint">
            <kbd>esc</kbd> close
          </span>
        </div>
      </div>
    </div>
  );
}