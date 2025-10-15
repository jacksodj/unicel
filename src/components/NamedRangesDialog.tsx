import { useState, useEffect } from 'react';
import { tauriApi, NamedRangeInfo } from '../api/tauri';

interface NamedRangesDialogProps {
  isOpen: boolean;
  onClose: () => void;
  currentSheetIndex: number;
}

export default function NamedRangesDialog({
  isOpen,
  onClose,
  currentSheetIndex,
}: NamedRangesDialogProps) {
  const [namedRanges, setNamedRanges] = useState<NamedRangeInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [newName, setNewName] = useState('');
  const [newAddress, setNewAddress] = useState('');
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen) {
      loadNamedRanges();
    }
  }, [isOpen]);

  const loadNamedRanges = async () => {
    try {
      setLoading(true);
      setError(null);
      const ranges = await tauriApi.listNamedRanges();
      setNamedRanges(ranges);
    } catch (err) {
      console.error('Failed to load named ranges:', err);
      setError('Failed to load named ranges');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async () => {
    if (!newName.trim() || !newAddress.trim()) {
      setError('Please enter both name and cell address');
      return;
    }

    // Validate name format (must start with lowercase or underscore)
    if (!/^[a-z_][a-z0-9_]*$/i.test(newName.trim())) {
      setError('Name must start with lowercase letter or underscore, and contain only letters, numbers, and underscores');
      return;
    }

    // Validate address format (e.g., A1, B12, AA100)
    if (!/^[A-Z]+[0-9]+$/i.test(newAddress.trim())) {
      setError('Invalid cell address format (e.g., A1, B12)');
      return;
    }

    try {
      setCreating(true);
      setError(null);
      await tauriApi.createNamedRange(
        newName.trim(),
        currentSheetIndex,
        newAddress.trim().toUpperCase()
      );
      setNewName('');
      setNewAddress('');
      await loadNamedRanges();
    } catch (err: any) {
      console.error('Failed to create named range:', err);
      setError(err.toString());
    } finally {
      setCreating(false);
    }
  };

  const handleDelete = async (name: string) => {
    if (!confirm(`Delete named range "${name}"?`)) {
      return;
    }

    try {
      setError(null);
      await tauriApi.deleteNamedRange(name);
      await loadNamedRanges();
    } catch (err: any) {
      console.error('Failed to delete named range:', err);
      setError(err.toString());
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !creating) {
      handleCreate();
    }
  };

  if (!isOpen) return null;

  if (loading) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
        <div className="bg-white rounded-lg p-8">
          <div className="text-center">Loading named ranges...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="bg-blue-600 text-white px-6 py-4">
          <h2 className="text-xl font-semibold">Named Ranges</h2>
          <p className="text-sm text-blue-100 mt-1">
            Create names for cells to use in formulas (e.g., =revenue * tax_rate)
          </p>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {/* Info box about inline syntax */}
          <div className="mb-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
            <div className="flex gap-2">
              <span className="text-blue-600">💡</span>
              <div className="text-sm text-blue-900">
                <p className="font-medium mb-1">Tip: Inline Label Syntax</p>
                <p className="mb-2">
                  You can also create named ranges directly when entering cell values:
                </p>
                <ul className="list-disc list-inside space-y-1">
                  <li>
                    <code className="bg-white px-1 py-0.5 rounded">tax_rate: 0.15</code> - creates a named range with a value
                  </li>
                  <li>
                    <code className="bg-white px-1 py-0.5 rounded">total:= A1+A2</code> - creates a named range with a formula
                  </li>
                </ul>
              </div>
            </div>
          </div>

          {/* Error message */}
          {error && (
            <div className="mb-4 bg-red-50 border border-red-200 rounded-lg p-3 text-sm text-red-800">
              {error}
            </div>
          )}

          {/* Create new named range */}
          <div className="mb-6 border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-semibold text-gray-700 mb-3">Create New Named Range</h3>
            <div className="grid grid-cols-2 gap-3">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Name
                </label>
                <input
                  type="text"
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="e.g., revenue, tax_rate"
                  value={newName}
                  onChange={(e) => setNewName(e.target.value)}
                  onKeyPress={handleKeyPress}
                />
                <p className="text-xs text-gray-500 mt-1">
                  Must start with lowercase letter or underscore
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Cell Address
                </label>
                <input
                  type="text"
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="e.g., A1, B12"
                  value={newAddress}
                  onChange={(e) => setNewAddress(e.target.value)}
                  onKeyPress={handleKeyPress}
                />
              </div>
            </div>
            <button
              className="mt-3 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:bg-gray-400"
              onClick={handleCreate}
              disabled={creating}
            >
              {creating ? 'Creating...' : 'Create Named Range'}
            </button>
          </div>

          {/* List of named ranges */}
          <div>
            <h3 className="text-sm font-semibold text-gray-700 mb-3">
              Existing Named Ranges ({namedRanges.length})
            </h3>
            {namedRanges.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <p>No named ranges defined yet.</p>
                <p className="text-sm mt-1">Create one above or use inline label syntax.</p>
              </div>
            ) : (
              <div className="border border-gray-200 rounded-lg overflow-hidden">
                <table className="w-full">
                  <thead className="bg-gray-50">
                    <tr>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-700 uppercase">
                        Name
                      </th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-700 uppercase">
                        Cell
                      </th>
                      <th className="px-4 py-2 text-left text-xs font-medium text-gray-700 uppercase">
                        Sheet
                      </th>
                      <th className="px-4 py-2 text-right text-xs font-medium text-gray-700 uppercase">
                        Actions
                      </th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200">
                    {namedRanges.map((range) => (
                      <tr key={range.name} className="hover:bg-gray-50">
                        <td className="px-4 py-3 text-sm font-medium text-gray-900">
                          {range.name}
                        </td>
                        <td className="px-4 py-3 text-sm text-gray-700 font-mono">
                          {range.cell_address}
                        </td>
                        <td className="px-4 py-3 text-sm text-gray-600">
                          Sheet {range.sheet_index + 1}
                        </td>
                        <td className="px-4 py-3 text-right">
                          <button
                            className="text-red-600 hover:text-red-800 text-sm"
                            onClick={() => handleDelete(range.name)}
                          >
                            Delete
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="border-t border-gray-200 px-6 py-4 flex justify-end">
          <button
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            onClick={onClose}
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
