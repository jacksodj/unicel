import { useEffect, useRef } from 'react';

export interface DeleteConfirmDialogProps {
  type: 'column' | 'row';
  identifier: string | number;
  onConfirm: () => void;
  onCancel: () => void;
}

export function DeleteConfirmDialog({
  type,
  identifier,
  onConfirm,
  onCancel,
}: DeleteConfirmDialogProps) {
  const confirmButtonRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    // Focus the confirm button when dialog opens
    confirmButtonRef.current?.focus();

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onCancel();
      } else if (event.key === 'Enter') {
        onConfirm();
      }
    };

    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [onConfirm, onCancel]);

  return (
    <div className="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
        <h2 className="text-lg font-semibold mb-3 text-gray-900">
          Delete {type === 'column' ? 'Column' : 'Row'} {identifier}?
        </h2>
        <p className="text-gray-600 mb-6">
          This will permanently delete {type === 'column' ? 'column' : 'row'} {identifier} and all
          its data. This action cannot be undone.
        </p>
        <div className="flex justify-end gap-3">
          <button
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded hover:bg-gray-200 transition-colors"
            onClick={onCancel}
          >
            Cancel
          </button>
          <button
            ref={confirmButtonRef}
            className="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
            onClick={onConfirm}
          >
            Delete {type === 'column' ? 'Column' : 'Row'}
          </button>
        </div>
      </div>
    </div>
  );
}
