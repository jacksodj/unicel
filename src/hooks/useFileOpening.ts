/**
 * Hook to handle file opening from external sources
 * (Messages, Email, Files app, etc.)
 */

import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { tauriApi } from '../api/tauri';

interface FileOpeningOptions {
  onFileOpen: (path: string) => Promise<void>;
}

export function useFileOpening({ onFileOpen }: FileOpeningOptions) {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    const processed = new Set<string>();
    let cancelled = false;

    const handleImportedFiles = async (paths: string[]) => {
      for (const file of paths) {
        if (!file || !file.endsWith('.usheet')) {
          continue;
        }
        if (processed.has(file)) {
          continue;
        }
        processed.add(file);
        await onFileOpen(file);
      }
    };

    const importPending = async () => {
      try {
        const files = await tauriApi.importIosPendingDocuments();
        if (!cancelled && files?.length) {
          await handleImportedFiles(files);
        }
      } catch (error) {
        console.error('Failed to import pending iOS documents:', error);
      }
    };

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
              processed.add(firstFile);
              await onFileOpen(firstFile);
            }
          }
        });
      } catch (error) {
        console.error('Failed to setup file opening listener:', error);
      }
    };

    setupListener();
    importPending();

    const handleVisibility = () => {
      if (document.visibilityState === 'visible') {
        importPending();
      }
    };

    document.addEventListener('visibilitychange', handleVisibility);

    return () => {
      cancelled = true;
      document.removeEventListener('visibilitychange', handleVisibility);
      // Cleanup listener on unmount
      if (unlisten) {
        unlisten();
      }
    };
  }, [onFileOpen]);
}
