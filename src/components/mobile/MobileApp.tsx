/**
 * Main mobile application component for iOS
 *
 * This is the root component for the iOS read-only viewer.
 * Handles file opening, display, and touch interactions.
 */

import { useState } from 'react';
import { MobileGrid } from './MobileGrid';
import { MobileToolbar } from './MobileToolbar';
import { MobileStatusBar } from './MobileStatusBar';

interface MobileAppProps {
  // Props will be defined during implementation
}

export function MobileApp(_props: MobileAppProps) {
  const [workbookPath, _setWorkbookPath] = useState<string | null>(null);
  const [currentSheet, _setCurrentSheet] = useState<string>('Sheet1');
  const [displayPreference, setDisplayPreference] = useState<'Metric' | 'Imperial'>('Metric');

  const handleOpenFile = async () => {
    // TODO: Implement iOS document picker
    console.log('Opening file picker...');
  };

  const handleToggleDisplay = () => {
    setDisplayPreference(prev => prev === 'Metric' ? 'Imperial' : 'Metric');
  };

  if (!workbookPath) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-50">
        <div className="text-center">
          <h1 className="text-2xl font-bold mb-4">Unicel</h1>
          <p className="text-gray-600 mb-6">Unit-aware spreadsheet viewer</p>
          <button
            onClick={handleOpenFile}
            className="px-6 py-3 bg-blue-500 text-white rounded-lg"
          >
            Open Spreadsheet
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen">
      <MobileToolbar
        onOpenFile={handleOpenFile}
        onToggleDisplay={handleToggleDisplay}
        displayPreference={displayPreference}
      />

      <MobileGrid
        workbookPath={workbookPath}
        sheetName={currentSheet}
        displayPreference={displayPreference}
      />

      <MobileStatusBar
        workbookPath={workbookPath}
        currentSheet={currentSheet}
      />
    </div>
  );
}
