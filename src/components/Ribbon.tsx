import { useState } from 'react';

interface RibbonProps {
  displayMode: 'AsEntered' | 'Metric' | 'Imperial';
  onDisplayModeChange: (mode: 'AsEntered' | 'Metric' | 'Imperial') => void;
  onNew: () => void;
  onOpen: () => void;
  onSave: () => void;
  onSaveAs: () => void;
  onOpenPreferences: () => void;
  isDirty?: boolean;
}

export default function Ribbon({
  displayMode,
  onDisplayModeChange,
  onNew,
  onOpen,
  onSave,
  onSaveAs,
  onOpenPreferences,
  isDirty = false,
}: RibbonProps) {
  const [showFileMenu, setShowFileMenu] = useState(false);

  const displayModeOptions = [
    { value: 'AsEntered' as const, label: 'As Entered', icon: '✏️' },
    { value: 'Metric' as const, label: 'Metric', icon: '🌍' },
    { value: 'Imperial' as const, label: 'Imperial', icon: '🇺🇸' },
  ];

  return (
    <div className="bg-gray-100 border-b border-gray-300">
      {/* Main ribbon */}
      <div className="flex items-center px-4 py-2 gap-6">
        {/* File menu */}
        <div className="relative">
          <button
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded font-semibold text-sm flex items-center gap-2"
            onClick={() => setShowFileMenu(!showFileMenu)}
          >
            File
            <span className="text-xs">▼</span>
          </button>

          {showFileMenu && (
            <>
              {/* Backdrop to close menu */}
              <div
                className="fixed inset-0 z-10"
                onClick={() => setShowFileMenu(false)}
              />
              {/* Menu */}
              <div className="absolute top-full left-0 mt-1 bg-white border border-gray-300 rounded shadow-lg z-20 min-w-[200px]">
                <button
                  className="w-full px-4 py-2 text-left hover:bg-gray-100 text-sm flex items-center gap-2"
                  onClick={() => {
                    onNew();
                    setShowFileMenu(false);
                  }}
                >
                  <span className="text-lg">📄</span>
                  New
                  <span className="ml-auto text-xs text-gray-500">Ctrl+N</span>
                </button>
                <button
                  className="w-full px-4 py-2 text-left hover:bg-gray-100 text-sm flex items-center gap-2"
                  onClick={() => {
                    onOpen();
                    setShowFileMenu(false);
                  }}
                >
                  <span className="text-lg">📂</span>
                  Open
                  <span className="ml-auto text-xs text-gray-500">Ctrl+O</span>
                </button>
                <div className="border-t border-gray-200" />
                <button
                  className="w-full px-4 py-2 text-left hover:bg-gray-100 text-sm flex items-center gap-2"
                  onClick={() => {
                    onSave();
                    setShowFileMenu(false);
                  }}
                >
                  <span className="text-lg">💾</span>
                  Save
                  <span className="ml-auto text-xs text-gray-500">Ctrl+S</span>
                  {isDirty && <span className="ml-1 text-orange-500">●</span>}
                </button>
                <button
                  className="w-full px-4 py-2 text-left hover:bg-gray-100 text-sm flex items-center gap-2"
                  onClick={() => {
                    onSaveAs();
                    setShowFileMenu(false);
                  }}
                >
                  <span className="text-lg">💾</span>
                  Save As...
                  <span className="ml-auto text-xs text-gray-500">Ctrl+Shift+S</span>
                </button>
              </div>
            </>
          )}
        </div>

        {/* Divider */}
        <div className="h-8 w-px bg-gray-300" />

        {/* Display mode toggle */}
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-gray-700">Display:</span>
          <div className="flex bg-white border border-gray-300 rounded overflow-hidden">
            {displayModeOptions.map((option) => (
              <button
                key={option.value}
                className={`
                  px-3 py-1 text-sm flex items-center gap-1 border-r border-gray-300 last:border-r-0
                  ${displayMode === option.value
                    ? 'bg-blue-500 text-white'
                    : 'hover:bg-gray-100 text-gray-700'
                  }
                `}
                onClick={() => onDisplayModeChange(option.value)}
                title={option.label}
              >
                <span>{option.icon}</span>
                <span className="hidden sm:inline">{option.label}</span>
              </button>
            ))}
          </div>
        </div>

        {/* Divider */}
        <div className="h-8 w-px bg-gray-300" />

        {/* Quick actions */}
        <div className="flex items-center gap-2">
          <button
            className="px-3 py-1 bg-white border border-gray-300 hover:bg-gray-50 rounded text-sm"
            title="Undo (Ctrl+Z)"
          >
            ↶ Undo
          </button>
          <button
            className="px-3 py-1 bg-white border border-gray-300 hover:bg-gray-50 rounded text-sm"
            title="Redo (Ctrl+Y)"
          >
            ↷ Redo
          </button>
        </div>

        {/* Divider */}
        <div className="h-8 w-px bg-gray-300" />

        {/* Settings */}
        <button
          className="px-3 py-1 bg-white border border-gray-300 hover:bg-gray-50 rounded text-sm flex items-center gap-2"
          onClick={onOpenPreferences}
          title="Unit Preferences"
        >
          ⚙️ <span className="hidden md:inline">Unit Settings</span>
        </button>
      </div>

      {/* Secondary toolbar (optional) */}
      <div className="bg-gray-50 px-4 py-1 flex items-center gap-4 text-xs text-gray-600">
        <span>Ready</span>
        {isDirty && (
          <span className="flex items-center gap-1 text-orange-600">
            <span>●</span>
            <span>Unsaved changes</span>
          </span>
        )}
      </div>
    </div>
  );
}
