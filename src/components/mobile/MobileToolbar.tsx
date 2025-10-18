/**
 * Simplified toolbar for mobile iOS app
 *
 * Features:
 * - File name display
 * - Sheet selector dropdown
 * - Display toggle (Metric/Imperial)
 * - Safe area inset handling for notch
 */

import { useState } from 'react';
import { haptics } from '../../utils/haptics';

interface MobileToolbarProps {
  fileName?: string;
  sheetNames: string[];
  currentSheetIndex: number;
  onSheetChange: (index: number) => void;
  onToggleDisplay: () => void;
  displayPreference: 'Metric' | 'Imperial';
  isLandscape?: boolean;
}

export function MobileToolbar({
  fileName = 'Untitled',
  sheetNames,
  currentSheetIndex,
  onSheetChange,
  onToggleDisplay,
  displayPreference,
  isLandscape = false,
}: MobileToolbarProps) {
  const [showSheetSelector, setShowSheetSelector] = useState(false);

  const handleSheetSelect = (index: number) => {
    onSheetChange(index);
    setShowSheetSelector(false);
    haptics.medium();
  };

  const handleDisplayToggle = () => {
    onToggleDisplay();
    haptics.light();
  };

  return (
    <>
      <div
        className={`flex items-center justify-between bg-white border-b shadow-sm relative z-20 ${
          isLandscape ? 'px-3 py-1.5' : 'px-4 py-2'
        }`}
        style={{
          paddingTop: isLandscape
            ? 'calc(env(safe-area-inset-top) + 0.25rem)'
            : 'calc(env(safe-area-inset-top) + 0.5rem)',
          paddingLeft: 'calc(env(safe-area-inset-left) + 1rem)',
          paddingRight: 'calc(env(safe-area-inset-right) + 1rem)',
        }}
      >
        {/* Left: File name and sheet selector */}
        <div className="flex-1 min-w-0 flex items-center gap-2">
          <h1 className={`font-semibold text-gray-900 truncate ${
            isLandscape ? 'text-sm' : 'text-base'
          }`}>
            {fileName}
          </h1>

          {/* Separator */}
          {isLandscape && <span className="text-gray-300">|</span>}

          {/* Sheet selector */}
          <button
            onClick={() => setShowSheetSelector(!showSheetSelector)}
            className={`flex items-center text-gray-600 ${
              isLandscape ? 'text-xs' : 'text-xs mt-0.5'
            }`}
          >
            <span className="truncate max-w-[120px]">{sheetNames[currentSheetIndex]}</span>
            <svg
              className={`w-4 h-4 ml-1 transition-transform ${showSheetSelector ? 'rotate-180' : ''}`}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
            </svg>
          </button>
        </div>

        {/* Center: App name (only in landscape on tablets) */}
        {isLandscape && (
          <div className="hidden md:block text-xs text-gray-400 px-4">Unicel</div>
        )}

        {/* Right: Display toggle */}
        <button
          onClick={handleDisplayToggle}
          className={`font-medium bg-blue-500 text-white rounded-lg active:bg-blue-600 transition-colors ${
            isLandscape ? 'px-2 py-1 text-xs' : 'px-3 py-1.5 text-sm'
          }`}
          style={{ minWidth: 44, minHeight: isLandscape ? 36 : 44 }}
          aria-label={`Toggle display preference (currently ${displayPreference})`}
        >
          {displayPreference}
        </button>
      </div>

      {/* Sheet selector dropdown */}
      {showSheetSelector && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 bg-black bg-opacity-25 z-30"
            onClick={() => setShowSheetSelector(false)}
          />

          {/* Dropdown menu */}
          <div
            className="absolute left-4 right-4 top-20 bg-white rounded-lg shadow-2xl border border-gray-200 z-40 max-h-64 overflow-y-auto"
            style={{
              left: 'calc(env(safe-area-inset-left) + 1rem)',
              right: 'calc(env(safe-area-inset-right) + 1rem)',
            }}
          >
            {sheetNames.map((name, index) => (
              <button
                key={index}
                onClick={() => handleSheetSelect(index)}
                className={`w-full px-4 py-3 text-left border-b border-gray-100 last:border-b-0 active:bg-gray-100 transition-colors ${
                  index === currentSheetIndex ? 'bg-blue-50 text-blue-600 font-semibold' : 'text-gray-900'
                }`}
                style={{ minHeight: 44 }}
              >
                <div className="flex items-center justify-between">
                  <span>{name}</span>
                  {index === currentSheetIndex && (
                    <svg className="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path
                        fillRule="evenodd"
                        d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                        clipRule="evenodd"
                      />
                    </svg>
                  )}
                </div>
              </button>
            ))}
          </div>
        </>
      )}
    </>
  );
}
