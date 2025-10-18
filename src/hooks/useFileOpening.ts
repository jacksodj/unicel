/**
 * Hook to handle file opening from external sources
 * (Messages, Email, Files app, etc.)
 */

import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

interface FileOpeningOptions {
  onFileOpen: (path: string) => Promise<void>;
}

export function useFileOpening({ onFileOpen }: FileOpeningOptions) {
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    // Listen for file open events from the system
    const setupListener = async () => {
      try {
        unlisten = await listen<string[]>('tauri://file-drop', async (event) => {
          // Handle dropped or opened files
          const files = event.payload;

          if (files && files.length > 0) {
            const firstFile = files[0];

            // Only handle .usheet files
            if (firstFile?.endsWith('.usheet')) {
              await onFileOpen(firstFile);
            }
          }
        });
      } catch (error) {
        console.error('Failed to setup file opening listener:', error);
      }
    };

    setupListener();

    return () => {
      // Cleanup listener on unmount
      if (unlisten) {
        unlisten();
      }
    };
  }, [onFileOpen]);
}
