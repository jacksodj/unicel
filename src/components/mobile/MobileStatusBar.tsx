/**
 * Status bar for mobile iOS app
 *
 * Features:
 * - Current sheet name
 * - Cell selection info
 * - Safe area inset handling (bottom home indicator)
 */

interface MobileStatusBarProps {
  workbookPath: string;
  currentSheet: string;
  selectedCell?: string | null;
}

export function MobileStatusBar({
  workbookPath,
  currentSheet,
  selectedCell,
}: MobileStatusBarProps) {
  // Extract filename from path
  const filename = workbookPath ? workbookPath.split('/').pop() : '';

  return (
    <div
      className="flex items-center justify-between px-4 py-2 bg-gray-50 border-t text-sm text-gray-600"
      style={{
        paddingBottom: 'calc(env(safe-area-inset-bottom) + 0.5rem)',
        paddingLeft: 'calc(env(safe-area-inset-left) + 1rem)',
        paddingRight: 'calc(env(safe-area-inset-right) + 1rem)',
      }}
    >
      {/* Left: File and sheet info */}
      <div className="flex items-center space-x-2">
        <span className="font-medium">{filename}</span>
        <span className="text-gray-400">•</span>
        <span>{currentSheet}</span>
      </div>

      {/* Right: Selected cell */}
      {selectedCell && (
        <div className="text-blue-500 font-medium">
          {selectedCell}
        </div>
      )}
    </div>
  );
}
