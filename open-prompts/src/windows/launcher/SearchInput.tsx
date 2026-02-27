import { useRef, useEffect } from 'react';
import { useLauncherStore } from '../../stores/launcherStore';

export function SearchInput() {
  const { query, setQuery, isLoading } = useLauncherStore();
  const inputRef = useRef<HTMLInputElement>(null);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  // Auto-focus on mount and when window gains focus
  useEffect(() => {
    inputRef.current?.focus();
    const handleFocus = () => inputRef.current?.focus();
    window.addEventListener('focus', handleFocus);
    return () => window.removeEventListener('focus', handleFocus);
  }, []);

  useEffect(() => {
    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, []);

  const handleChange = (value: string) => {
    // Debounce search by 150ms
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => {
      setQuery(value);
    }, 150);
  };

  return (
    <div className="launcher-search">
      <input
        ref={inputRef}
        className="search-input"
        type="text"
        placeholder={isLoading ? 'Searching...' : 'Search prompts...'}
        defaultValue={query}
        onChange={e => handleChange(e.target.value)}
        autoComplete="off"
        spellCheck={false}
      />
    </div>
  );
}