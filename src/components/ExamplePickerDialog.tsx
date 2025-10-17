import { useState, useEffect } from 'react';
import { tauriApi } from '../api/tauri';

interface ExamplePickerDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSelectExample: (filename: string) => void;
}

export default function ExamplePickerDialog({
  isOpen,
  onClose,
  onSelectExample,
}: ExamplePickerDialogProps) {
  const [examples, setExamples] = useState<Array<[string, string]>>([]);
  const [selectedFilename, setSelectedFilename] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen) {
      const loadExamples = async () => {
        try {
          const examplesList = await tauriApi.listExampleWorkbooks();
          setExamples(examplesList);
          if (examplesList.length > 0 && examplesList[0]) {
            setSelectedFilename(examplesList[0][0]);
          }
        } catch (error) {
          console.error('Failed to load examples:', error);
        }
      };
      loadExamples();
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const handleOpen = () => {
    if (selectedFilename) {
      onSelectExample(selectedFilename);
      onClose();
    }
  };

  return (
    <>
      {/* Backdrop */}
      <div className="fixed inset-0 bg-black bg-opacity-50 z-50" onClick={onClose} />

      {/* Dialog */}
      <div className="fixed inset-0 flex items-center justify-center z-50 pointer-events-none">
        <div className="bg-white rounded-lg shadow-xl max-w-md w-full pointer-events-auto">
          {/* Header */}
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-xl font-semibold text-gray-800">Open Example Workbook</h2>
          </div>

          {/* Content */}
          <div className="px-6 py-4">
            <p className="text-sm text-gray-600 mb-4">
              Choose an example workbook to explore Unicel's features:
            </p>
            <div className="space-y-2">
              {examples.map(([filename, title]) => (
                <label
                  key={filename}
                  className="flex items-center p-3 border border-gray-300 rounded hover:bg-gray-50 cursor-pointer"
                >
                  <input
                    type="radio"
                    name="example"
                    value={filename}
                    checked={selectedFilename === filename}
                    onChange={() => setSelectedFilename(filename)}
                    className="mr-3"
                  />
                  <div>
                    <div className="font-medium text-gray-800">{title}</div>
                    <div className="text-xs text-gray-500">{filename}</div>
                  </div>
                </label>
              ))}
            </div>
          </div>

          {/* Footer */}
          <div className="px-6 py-4 border-t border-gray-200 flex justify-end gap-2">
            <button className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded" onClick={onClose}>
              Cancel
            </button>
            <button
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded"
              onClick={handleOpen}
              disabled={!selectedFilename}
            >
              Open
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
