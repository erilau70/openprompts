import { getCurrentWindow } from '@tauri-apps/api/window';
import { EditorApp } from './windows/editor/EditorApp';
import { LauncherApp } from './windows/launcher/LauncherApp';

function App() {
  const windowLabel = getCurrentWindow().label;

  if (windowLabel === 'editor') {
    return <EditorApp />;
  }

  return <LauncherApp />;
}

export default App;
