import Spreadsheet from './components/Spreadsheet';
import { MobileApp } from './components/mobile/MobileApp';
import { useMobile } from './hooks/useMobile';

function App() {
  const { isTouchDevice } = useMobile();

  // Route to mobile UI for touch devices (iOS, iPad)
  if (isTouchDevice) {
    return <MobileApp />;
  }

  // Desktop UI
  return <Spreadsheet />;
}

export default App;
