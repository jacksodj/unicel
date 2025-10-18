/**
 * Example spreadsheet picker for iOS
 * Shows list of bundled example .usheet files
 */

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ExamplePickerProps {
  onExampleSelected: (path: string) => Promise<void>;
  onClose: () => void;
}

interface ExampleWorkbook {
  filename: string;
  displayName: string;
}

export function ExamplePicker({ onExampleSelected, onClose }: ExamplePickerProps) {
  const [examples, setExamples] = useState<ExampleWorkbook[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadExamples();
  }, []);

  const loadExamples = async () => {
    try {
      console.log('Loading example list...');
      const exampleList = await invoke<[string, string][]>('list_example_workbooks');
      console.log('Example list received:', exampleList);
      setExamples(
        exampleList.map(([filename, displayName]) => ({
          filename,
          displayName,
        }))
      );
    } catch (err) {
      console.error('Failed to load examples:', err);
      setError(err instanceof Error ? err.message : 'Failed to load examples');
    }
  };

  const handleSelectExample = async (filename: string) => {
    try {
      setIsLoading(true);
      setError(null);

      console.log('Getting path for example:', filename);

      // Get the full path to the example file
      const path = await invoke<string>('get_example_workbook_path', { filename });

      console.log('Example path received:', path);

      // Load the example
      await onExampleSelected(path);

      console.log('Example loaded successfully');
    } catch (err) {
      console.error('Failed to load example:', err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      console.error('Error details:', errorMessage);
      setError(errorMessage);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-end justify-center z-50">
      <div
        className="bg-white rounded-t-2xl w-full max-h-[80vh] overflow-y-auto"
        style={{
          paddingBottom: 'calc(env(safe-area-inset-bottom) + 1rem)',
        }}
      >
        {/* Header */}
        <div className="sticky top-0 bg-white border-b px-4 py-4 flex items-center justify-between">
          <h2 className="text-xl font-semibold">Example Spreadsheets</h2>
          <button
            onClick={onClose}
            className="w-10 h-10 flex items-center justify-center text-gray-500 hover:text-gray-700"
            aria-label="Close"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        {/* Error message */}
        {error && (
          <div className="mx-4 mt-4 p-3 bg-red-50 border border-red-200 rounded-lg">
            <p className="text-red-800 text-sm">{error}</p>
          </div>
        )}

        {/* Example list */}
        <div className="p-4 space-y-2">
          {examples.map(({ filename, displayName }) => (
            <button
              key={filename}
              onClick={() => handleSelectExample(filename)}
              disabled={isLoading}
              className="w-full text-left px-4 py-4 bg-gray-50 hover:bg-gray-100 rounded-lg disabled:opacity-50 transition-colors"
              style={{ minHeight: '56px' }}
            >
              <div className="flex items-center gap-3">
                <div className="flex-shrink-0 w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-blue-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                </div>
                <div className="flex-1">
                  <p className="font-medium text-gray-900">{displayName}</p>
                  <p className="text-sm text-gray-500">{filename}</p>
                </div>
                {isLoading ? (
                  <svg
                    className="w-5 h-5 animate-spin text-blue-500"
                    fill="none"
                    viewBox="0 0 24 24"
                  >
                    <circle
                      className="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      strokeWidth="4"
                    />
                    <path
                      className="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    />
                  </svg>
                ) : (
                  <svg
                    className="w-5 h-5 text-gray-400"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 5l7 7-7 7"
                    />
                  </svg>
                )}
              </div>
            </button>
          ))}
        </div>

        {examples.length === 0 && !error && (
          <div className="p-8 text-center text-gray-500">
            <p>No example spreadsheets available</p>
          </div>
        )}
      </div>
    </div>
  );
}
