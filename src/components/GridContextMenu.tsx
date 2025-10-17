import { useEffect, useRef } from 'react';

export interface ContextMenuPosition {
  x: number;
  y: number;
}

export interface ColumnContextMenuProps {
  position: ContextMenuPosition;
  column: string;
  onClose: () => void;
  onInsertBefore: (col: string) => void;
  onInsertAfter: (col: string) => void;
  onDelete: (col: string) => void;
}

export interface RowContextMenuProps {
  position: ContextMenuPosition;
  row: number;
  onClose: () => void;
  onInsertBefore: (row: number) => void;
  onInsertAfter: (row: number) => void;
  onDelete: (row: number) => void;
}

export function ColumnContextMenu({
  position,
  column,
  onClose,
  onInsertBefore,
  onInsertAfter,
  onDelete,
}: ColumnContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    document.addEventListener('keydown', handleEscape);

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
      document.removeEventListener('keydown', handleEscape);
    };
  }, [onClose]);

  const handleAction = (action: () => void) => {
    action();
    onClose();
  };

  return (
    <div
      ref={menuRef}
      className="fixed bg-white border border-gray-300 rounded shadow-lg py-1 z-50 min-w-[180px]"
      style={{
        left: `${position.x}px`,
        top: `${position.y}px`,
      }}
    >
      <button
        className="w-full px-4 py-2 text-left text-sm hover:bg-blue-50 transition-colors"
        onClick={() => handleAction(() => onInsertBefore(column))}
      >
        Insert Column Before
      </button>
      <button
        className="w-full px-4 py-2 text-left text-sm hover:bg-blue-50 transition-colors"
        onClick={() => handleAction(() => onInsertAfter(column))}
      >
        Insert Column After
      </button>
      <div className="border-t border-gray-200 my-1"></div>
      <button
        className="w-full px-4 py-2 text-left text-sm text-red-600 hover:bg-red-50 transition-colors"
        onClick={() => handleAction(() => onDelete(column))}
      >
        Delete Column {column}
      </button>
    </div>
  );
}

export function RowContextMenu({
  position,
  row,
  onClose,
  onInsertBefore,
  onInsertAfter,
  onDelete,
}: RowContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    document.addEventListener('keydown', handleEscape);

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
      document.removeEventListener('keydown', handleEscape);
    };
  }, [onClose]);

  const handleAction = (action: () => void) => {
    action();
    onClose();
  };

  return (
    <div
      ref={menuRef}
      className="fixed bg-white border border-gray-300 rounded shadow-lg py-1 z-50 min-w-[180px]"
      style={{
        left: `${position.x}px`,
        top: `${position.y}px`,
      }}
    >
      <button
        className="w-full px-4 py-2 text-left text-sm hover:bg-blue-50 transition-colors"
        onClick={() => handleAction(() => onInsertBefore(row))}
      >
        Insert Row Above
      </button>
      <button
        className="w-full px-4 py-2 text-left text-sm hover:bg-blue-50 transition-colors"
        onClick={() => handleAction(() => onInsertAfter(row))}
      >
        Insert Row Below
      </button>
      <div className="border-t border-gray-200 my-1"></div>
      <button
        className="w-full px-4 py-2 text-left text-sm text-red-600 hover:bg-red-50 transition-colors"
        onClick={() => handleAction(() => onDelete(row))}
      >
        Delete Row {row}
      </button>
    </div>
  );
}
