/**
 * Simplified toolbar for mobile iOS app
 *
 * Features:
 * - File operations (open)
 * - Display toggle (Metric/Imperial)
 * - Sheet selector
 * - Safe area inset handling
 */

interface MobileToolbarProps {
  onOpenFile: () => void;
  onToggleDisplay: () => void;
  displayPreference: 'Metric' | 'Imperial';
}

export function MobileToolbar({
  onOpenFile,
  onToggleDisplay,
  displayPreference,
}: MobileToolbarProps) {
  return (
    <div
      className="flex items-center justify-between px-4 py-2 bg-white border-b shadow-sm"
      style={{
        paddingTop: 'calc(env(safe-area-inset-top) + 0.5rem)',
        paddingLeft: 'calc(env(safe-area-inset-left) + 1rem)',
        paddingRight: 'calc(env(safe-area-inset-right) + 1rem)',
      }}
    >
      {/* Left: File operations */}
      <button
        onClick={onOpenFile}
        className="p-2 text-blue-500 hover:bg-gray-100 rounded"
        aria-label="Open file"
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
            d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
          />
        </svg>
      </button>

      {/* Center: App title */}
      <h1 className="text-lg font-semibold">Unicel</h1>

      {/* Right: Display toggle */}
      <button
        onClick={onToggleDisplay}
        className="px-3 py-1 text-sm bg-gray-100 rounded hover:bg-gray-200"
      >
        {displayPreference}
      </button>
    </div>
  );
}
