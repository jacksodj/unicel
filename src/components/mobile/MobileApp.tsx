/**
 * Main mobile application component for iOS
 *
 * This is the root component for the iOS read-only viewer.
 * Handles file opening, display, and touch interactions.
 * Supports both iPhone and iPad layouts with responsive design.
 */

import { useState, useCallback, useEffect } from 'react';
import { MobileGrid } from './MobileGrid';
import { MobileToolbar } from './MobileToolbar';
import { MobileStatusBar } from './MobileStatusBar';
import { FilePicker } from './FilePicker';
import { ExamplePicker } from './ExamplePicker';
import { ErrorBoundary } from './ErrorBoundary';
import { LoadingSpinner } from './LoadingSpinner';
import { useMobile } from '../../hooks/useMobile';
import { useFileOpening } from '../../hooks/useFileOpening';
import { tauriApi, convertCellData } from '../../api/tauri';
import { haptics } from '../../utils/haptics';
import { Cell } from '../../types/workbook';

interface MobileAppProps {
  // Props will be defined during implementation
}

export function MobileApp(_props: MobileAppProps) {
  const [workbookPath, setWorkbookPath] = useState<string | null>(null);
  const [workbookName, setWorkbookName] = useState<string | null>(null);
  const [currentSheet, setCurrentSheet] = useState<string>('Sheet1');
  const [displayPreference, setDisplayPreference] = useState<'Metric' | 'Imperial'>('Metric');
  const [selectedCellAddress, setSelectedCellAddress] = useState<string | null>(null);
  const [selectedCellData, setSelectedCellData] = useState<Cell | null>(null);
  const [cells, setCells] = useState<Map<string, Cell>>(new Map());
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sheetNames, setSheetNames] = useState<string[]>([]);
  const [showExamplePicker, setShowExamplePicker] = useState(false);

  // Platform detection
  const { isMobile, isTablet } = useMobile();

  // Handle file selection
  const handleFileSelected = useCallback(async (path: string) => {
    try {
      setIsLoading(true);
      setError(null);

      // Load workbook via Tauri
      await tauriApi.loadWorkbook(path);

      // Get workbook info
      const info = await tauriApi.getWorkbookInfo();

      // Extract filename from path
      const filename = path.split('/').pop()?.replace('.usheet', '') || 'Untitled';

      setWorkbookPath(path);
      setWorkbookName(filename);
      setSheetNames(info.sheet_names);
      setCurrentSheet(info.sheet_names[info.active_sheet_index] || 'Sheet1');

      // Success haptic
      haptics.success();
    } catch (err) {
      console.error('Failed to load workbook:', err);
      setError(err instanceof Error ? err.message : 'Failed to load file');
      haptics.error();
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Handle display toggle
  const handleToggleDisplay = useCallback(async () => {
    try {
      const newPreference = displayPreference === 'Metric' ? 'Imperial' : 'Metric';
      setDisplayPreference(newPreference);

      // Update backend preference
      await tauriApi.setDisplayMode(newPreference);

      // Light haptic feedback
      haptics.light();
    } catch (err) {
      console.error('Failed to toggle display:', err);
      haptics.error();
    }
  }, [displayPreference]);

  // Handle sheet change
  const handleSheetChange = useCallback(async (sheetName: string) => {
    try {
      const sheetIndex = sheetNames.indexOf(sheetName);
      if (sheetIndex === -1) return;

      await tauriApi.setActiveSheet(sheetIndex);
      setCurrentSheet(sheetName);
      haptics.medium();
    } catch (err) {
      console.error('Failed to change sheet:', err);
      haptics.error();
    }
  }, [sheetNames]);

  // Handle file open error
  const handleFileError = useCallback((errorMsg: string) => {
    setError(errorMsg);
    haptics.error();
  }, []);

  // Reset error and try again
  const handleRetry = useCallback(() => {
    setError(null);
    setWorkbookPath(null);
    setWorkbookName(null);
  }, []);

  // Handle file opening from external sources (Messages, Email, etc.)
  useFileOpening({
    onFileOpen: handleFileSelected,
  });

  // Load cells when sheet changes
  useEffect(() => {
    const loadCells = async () => {
      if (!workbookPath) return;

      try {
        const cellsData = await tauriApi.getSheetCells();
        const cellsMap = new Map<string, Cell>();

        cellsData.forEach(([address, cellData]) => {
          cellsMap.set(address, convertCellData(cellData));
        });

        setCells(cellsMap);
      } catch (err) {
        console.error('Failed to load cells:', err);
      }
    };

    loadCells();
  }, [workbookPath, currentSheet]);

  // Update selected cell data when selection or cells change
  useEffect(() => {
    if (selectedCellAddress && cells.has(selectedCellAddress)) {
      setSelectedCellData(cells.get(selectedCellAddress) || null);
    } else {
      setSelectedCellData(null);
    }
  }, [selectedCellAddress, cells]);

  // Loading state
  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-50">
        <LoadingSpinner message="Loading spreadsheet..." size="large" fullScreen />
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-50 p-4">
        <div className="max-w-md w-full bg-white rounded-lg shadow-lg p-6">
          <div className="flex items-center justify-center w-16 h-16 bg-red-100 rounded-full mx-auto mb-4">
            <svg
              className="w-8 h-8 text-red-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </div>
          <h2 className="text-xl font-bold text-center mb-2">Failed to load file</h2>
          <p className="text-gray-600 text-center mb-4">{error}</p>
          <button
            onClick={handleRetry}
            className="w-full py-3 px-4 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
            style={{ minHeight: '44px' }}
          >
            Try Another File
          </button>
        </div>
      </div>
    );
  }

  // No file loaded - show file picker
  if (!workbookPath) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-50 p-4">
        <div className="text-center max-w-md w-full">
          <div className="mb-8">
            <h1 className="text-4xl md:text-5xl font-bold mb-2 text-gray-900">Unicel</h1>
            <p className="text-gray-600 text-base md:text-lg">
              Unit-aware spreadsheet viewer
            </p>
          </div>

          <div className="mb-8">
            <svg
              className="w-24 h-24 mx-auto text-gray-300"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              />
            </svg>
          </div>

          <div className="space-y-3">
            <FilePicker onFileSelected={handleFileSelected} onError={handleFileError} />

            <button
              onClick={() => setShowExamplePicker(true)}
              className="flex items-center justify-center gap-2 w-full px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200"
              style={{ minHeight: '44px' }}
            >
              <svg
                className="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                />
              </svg>
              <span>Open Example</span>
            </button>
          </div>

          <p className="mt-6 text-xs text-gray-500">
            {isMobile ? 'iPhone' : isTablet ? 'iPad' : 'Mobile'} • Read-only viewer
          </p>
        </div>

        {/* Example picker modal */}
        {showExamplePicker && (
          <ExamplePicker
            onExampleSelected={async (path) => {
              await handleFileSelected(path);
              setShowExamplePicker(false);
            }}
            onClose={() => setShowExamplePicker(false)}
          />
        )}
      </div>
    );
  }

  // File loaded - show spreadsheet
  return (
    <ErrorBoundary>
      <div className="flex flex-col h-screen bg-white">
        <MobileToolbar
          fileName={workbookName || 'Untitled'}
          sheetNames={sheetNames}
          currentSheetIndex={sheetNames.indexOf(currentSheet)}
          onSheetChange={(index) => {
            const sheetName = sheetNames[index];
            if (sheetName) {
              handleSheetChange(sheetName);
            }
          }}
          onToggleDisplay={handleToggleDisplay}
          displayPreference={displayPreference}
        />

        <MobileGrid
          workbookPath={workbookPath}
          sheetName={currentSheet}
          displayPreference={displayPreference}
          selectedCell={selectedCellAddress}
          onCellSelect={(address: string) => {
            setSelectedCellAddress(address);
            haptics.light();
          }}
        />

        <MobileStatusBar
          selectedCell={selectedCellAddress}
          selectedCellData={selectedCellData}
          displayPreference={displayPreference}
        />
      </div>
    </ErrorBoundary>
  );
}
