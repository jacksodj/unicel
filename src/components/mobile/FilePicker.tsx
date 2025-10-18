/**
 * File picker component for iOS
 * Uses Tauri dialog plugin to open .usheet files
 */

import { open } from '@tauri-apps/plugin-dialog';
import { useState } from 'react';

interface FilePickerProps {
  onFileSelected: (path: string) => Promise<void>;
  onError?: (error: string) => void;
}

export function FilePicker({ onFileSelected, onError }: FilePickerProps) {
  const [isSelecting, setIsSelecting] = useState(false);

  const handleOpenFile = async () => {
    try {
      setIsSelecting(true);

      console.log('Opening file picker...');

      // Open file picker dialog
      // On iOS, we need to use proper configuration to open document picker
      // The UTI is defined in Info.plist as com.unicel.usheet
      const selected = await open({
        multiple: false,
        directory: false,
        // On iOS, filters work with UTI from Info.plist
        // The extension filter should trigger UIDocumentPickerViewController
        filters: [
          {
            name: 'Unicel Spreadsheets',
            extensions: ['usheet'],
          },
        ],
        title: 'Select Spreadsheet',
        // Explicitly request document picker behavior
        defaultPath: undefined,
      });

      console.log('File picker result:', selected);

      if (selected) {
        // selected can be string or string[] (for multiple), we specified single file
        const filePath = Array.isArray(selected) ? selected[0] : selected;

        if (filePath) {
          console.log('Selected file:', filePath);
          await onFileSelected(filePath);
        }
      } else {
        console.log('No file selected');
      }
    } catch (error) {
      console.error('File picker error:', error);
      onError?.(error instanceof Error ? error.message : 'Failed to open file picker');
    } finally {
      setIsSelecting(false);
    }
  };

  return (
    <button
      onClick={handleOpenFile}
      disabled={isSelecting}
      className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg disabled:opacity-50"
      style={{ minHeight: '44px', minWidth: '44px' }}
    >
      {isSelecting ? (
        <>
          <svg className="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24">
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
          <span>Opening...</span>
        </>
      ) : (
        <>
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
              d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
            />
          </svg>
          <span>Open Spreadsheet</span>
        </>
      )}
    </button>
  );
}
