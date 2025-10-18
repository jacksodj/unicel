/**
 * iPad split view layout
 *
 * Shows grid on left, cell details on right for tablets
 * Responsive: collapses to single pane on smaller screens
 */

import { useState } from 'react';

export interface CellDetails {
  address: string;
  value: number | string;
  unit: string | null;
  formula: string | null;
  displayValue: string;
}

export interface iPadSplitViewProps {
  gridContent: React.ReactNode;
  selectedCell: CellDetails | null;
  onCellDeselect?: () => void;
}

export function iPadSplitView({ gridContent, selectedCell, onCellDeselect }: iPadSplitViewProps) {
  const [showDetails, setShowDetails] = useState(true);

  return (
    <div className="flex h-full">
      {/* Grid pane - always visible */}
      <div className={`flex-1 overflow-hidden ${selectedCell && showDetails ? 'lg:flex-[2]' : ''}`}>
        {gridContent}
      </div>

      {/* Details pane - only on iPad when cell selected */}
      {selectedCell && (
        <div className="hidden lg:flex lg:flex-col lg:w-80 xl:w-96 border-l bg-gray-50">
          <CellDetailsPanel
            cell={selectedCell}
            onClose={() => {
              setShowDetails(false);
              onCellDeselect?.();
            }}
          />
        </div>
      )}
    </div>
  );
}

function CellDetailsPanel({ cell, onClose }: { cell: CellDetails; onClose: () => void }) {
  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex justify-between items-center p-4 border-b bg-white">
        <h2 className="text-lg font-semibold">Cell {cell.address}</h2>
        <button
          onClick={onClose}
          className="text-gray-500 hover:text-gray-700 p-1"
          aria-label="Close details panel"
        >
          ✕
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {/* Value */}
        <DetailSection label="Value">
          <div className="text-xl font-mono">{cell.displayValue}</div>
        </DetailSection>

        {/* Raw value and unit */}
        {cell.unit && (
          <DetailSection label="Raw Value">
            <div className="font-mono text-gray-700">
              {cell.value} {cell.unit}
            </div>
          </DetailSection>
        )}

        {/* Formula */}
        {cell.formula && (
          <DetailSection label="Formula">
            <div className="font-mono text-sm bg-gray-100 p-3 rounded border">
              {cell.formula}
            </div>
          </DetailSection>
        )}

        {/* Unit information */}
        {cell.unit && (
          <DetailSection label="Unit">
            <div className="space-y-2">
              <div className="text-sm">
                <span className="font-semibold">Type:</span> {getUnitType(cell.unit)}
              </div>
              <div className="text-sm">
                <span className="font-semibold">Dimension:</span> {getUnitDimension(cell.unit)}
              </div>
            </div>
          </DetailSection>
        )}

        {/* Metadata */}
        <DetailSection label="Metadata">
          <div className="text-sm space-y-1">
            <div>
              <span className="font-semibold">Type:</span>{' '}
              {cell.formula ? 'Formula' : 'Constant'}
            </div>
            <div>
              <span className="font-semibold">Address:</span> {cell.address}
            </div>
          </div>
        </DetailSection>
      </div>
    </div>
  );
}

function DetailSection({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="bg-white p-4 rounded-lg border">
      <h3 className="text-sm font-semibold text-gray-600 mb-2">{label}</h3>
      {children}
    </div>
  );
}

// Utility functions - these would be replaced with actual unit system integration
function getUnitType(unit: string): string {
  const types: Record<string, string> = {
    'm': 'Length',
    'ft': 'Length',
    'mi': 'Length',
    'km': 'Length',
    'kg': 'Mass',
    'lb': 'Mass',
    'g': 'Mass',
    's': 'Time',
    'min': 'Time',
    'hr': 'Time',
    'USD': 'Currency',
    'EUR': 'Currency',
    'GBP': 'Currency',
  };

  return types[unit] || 'Unknown';
}

function getUnitDimension(unit: string): string {
  const dimensions: Record<string, string> = {
    'm': 'L',
    'ft': 'L',
    'mi': 'L',
    'km': 'L',
    'kg': 'M',
    'lb': 'M',
    'g': 'M',
    's': 'T',
    'min': 'T',
    'hr': 'T',
    'USD': '[$]',
    'EUR': '[$]',
    'GBP': '[$]',
  };

  return dimensions[unit] || 'Dimensionless';
}

/**
 * Side-by-side metric/imperial comparison view
 */
export function MetricImperialComparisonView({ cell }: { cell: CellDetails }) {
  // This would integrate with the actual conversion system
  const metricValue = cell.displayValue; // Placeholder
  const imperialValue = cell.displayValue; // Placeholder

  return (
    <div className="grid grid-cols-2 gap-4 p-4">
      <div className="bg-blue-50 p-4 rounded-lg border-2 border-blue-200">
        <h3 className="text-sm font-semibold text-blue-700 mb-2">Metric</h3>
        <div className="text-xl font-mono">{metricValue}</div>
      </div>

      <div className="bg-green-50 p-4 rounded-lg border-2 border-green-200">
        <h3 className="text-sm font-semibold text-green-700 mb-2">Imperial</h3>
        <div className="text-xl font-mono">{imperialValue}</div>
      </div>
    </div>
  );
}
