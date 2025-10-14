import { useState, useEffect } from 'react';
import { tauriApi, UnitPreferences } from '../api/tauri';

interface UnitPreferencesDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSave?: () => void;
}

export default function UnitPreferencesDialog({
  isOpen,
  onClose,
  onSave,
}: UnitPreferencesDialogProps) {
  const [preferences, setPreferences] = useState<UnitPreferences | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [activeTab, setActiveTab] = useState<'metric' | 'imperial' | 'digital' | 'rates' | 'currency'>('metric');
  const [unitsInUse, setUnitsInUse] = useState<string[]>([]);

  useEffect(() => {
    if (isOpen) {
      loadPreferences();
    }
  }, [isOpen]);

  const loadPreferences = async () => {
    try {
      setLoading(true);
      // Load preferences and units separately with individual error handling
      const prefs = await tauriApi.getUnitPreferences();
      setPreferences(prefs);

      // Try to load units in use, but don't fail if it errors
      try {
        const units = await tauriApi.getUnitsInUse();
        setUnitsInUse(units);
      } catch (unitsError) {
        console.warn('Failed to load units in use:', unitsError);
        setUnitsInUse([]);
      }
    } catch (error) {
      console.error('Failed to load preferences:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    console.log('[UnitPreferencesDialog] handleSave called');
    console.log('[UnitPreferencesDialog] preferences:', preferences);

    if (!preferences) {
      console.log('[UnitPreferencesDialog] No preferences to save');
      return;
    }

    try {
      setSaving(true);
      console.log('[UnitPreferencesDialog] Calling updateUnitPreferences with:', preferences);
      await tauriApi.updateUnitPreferences(preferences);
      console.log('[UnitPreferencesDialog] Save successful');
      onSave?.();
      onClose();
    } catch (error) {
      console.error('[UnitPreferencesDialog] Failed to save preferences:', error);
    } finally {
      setSaving(false);
    }
  };

  if (!isOpen) return null;

  if (loading || !preferences) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
        <div className="bg-white rounded-lg p-8">
          <div className="text-center">Loading preferences...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-4xl max-h-[90vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="bg-blue-600 text-white px-6 py-4">
          <h2 className="text-xl font-semibold">Unit Preferences</h2>
          <p className="text-sm text-blue-100 mt-1">
            Configure how units are displayed in Metric and Imperial modes
          </p>
        </div>

        {/* Tabs */}
        <div className="border-b border-gray-200 flex">
          {[
            { id: 'metric' as const, label: 'Metric System', icon: '🌍' },
            { id: 'imperial' as const, label: 'Imperial System', icon: '🇺🇸' },
            { id: 'digital' as const, label: 'Digital Storage', icon: '💾' },
            { id: 'rates' as const, label: 'Time Rates', icon: '⏱️' },
            { id: 'currency' as const, label: 'Currency', icon: '💰' },
          ].map((tab) => (
            <button
              key={tab.id}
              className={`flex-1 px-6 py-3 text-sm font-medium flex items-center justify-center gap-2 ${
                activeTab === tab.id
                  ? 'border-b-2 border-blue-600 text-blue-600 bg-blue-50'
                  : 'text-gray-600 hover:bg-gray-50'
              }`}
              onClick={() => setActiveTab(tab.id)}
            >
              <span>{tab.icon}</span>
              <span>{tab.label}</span>
            </button>
          ))}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {/* Units in Use Section */}
          {unitsInUse.length > 0 && (
            <div className="mb-6 bg-gray-50 border border-gray-200 rounded-lg p-4">
              <h3 className="text-sm font-semibold text-gray-700 mb-2">
                Units Currently in Use
              </h3>
              <div className="flex flex-wrap gap-2">
                {unitsInUse.map((unit) => (
                  <span
                    key={unit}
                    className="px-2 py-1 bg-white border border-gray-300 rounded text-sm text-gray-700"
                  >
                    {unit}
                  </span>
                ))}
              </div>
              <p className="text-xs text-gray-600 mt-2">
                These are the units detected in your current sheet. Configure how they display in different modes below.
              </p>
            </div>
          )}

          {activeTab === 'metric' && (
            <div className="space-y-6">
              {/* Metric System Choice */}
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">
                  Metric System
                </label>
                <div className="space-y-2">
                  <label className="flex items-center gap-3 p-3 border border-gray-300 rounded hover:bg-gray-50 cursor-pointer">
                    <input
                      type="radio"
                      name="metricSystem"
                      checked={preferences.metric_system === 'MKS'}
                      onChange={() =>
                        setPreferences({
                          ...preferences,
                          metric_system: 'MKS',
                          metric_length: 'm',
                          metric_mass: 'kg',
                        })
                      }
                      className="w-4 h-4"
                    />
                    <div>
                      <div className="font-medium">MKS (Meter-Kilogram-Second)</div>
                      <div className="text-sm text-gray-600">
                        Standard SI units: meters, kilograms, seconds
                      </div>
                    </div>
                  </label>
                  <label className="flex items-center gap-3 p-3 border border-gray-300 rounded hover:bg-gray-50 cursor-pointer">
                    <input
                      type="radio"
                      name="metricSystem"
                      checked={preferences.metric_system === 'CGS'}
                      onChange={() =>
                        setPreferences({
                          ...preferences,
                          metric_system: 'CGS',
                          metric_length: 'cm',
                          metric_mass: 'g',
                        })
                      }
                      className="w-4 h-4"
                    />
                    <div>
                      <div className="font-medium">CGS (Centimeter-Gram-Second)</div>
                      <div className="text-sm text-gray-600">
                        Alternative metric: centimeters, grams, seconds
                      </div>
                    </div>
                  </label>
                </div>
              </div>

              {/* Metric Units */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Length Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.metric_length}
                    onChange={(e) =>
                      setPreferences({ ...preferences, metric_length: e.target.value })
                    }
                  >
                    <option value="mm">Millimeter (mm)</option>
                    <option value="cm">Centimeter (cm)</option>
                    <option value="m">Meter (m)</option>
                    <option value="km">Kilometer (km)</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Mass Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.metric_mass}
                    onChange={(e) =>
                      setPreferences({ ...preferences, metric_mass: e.target.value })
                    }
                  >
                    <option value="mg">Milligram (mg)</option>
                    <option value="g">Gram (g)</option>
                    <option value="kg">Kilogram (kg)</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Time Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.metric_time}
                    onChange={(e) =>
                      setPreferences({ ...preferences, metric_time: e.target.value })
                    }
                  >
                    <option value="s">Second (s)</option>
                    <option value="min">Minute (min)</option>
                    <option value="hr">Hour (hr)</option>
                    <option value="day">Day (day)</option>
                    <option value="month">Month (month)</option>
                    <option value="year">Year (year)</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Temperature Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.metric_temperature}
                    onChange={(e) =>
                      setPreferences({ ...preferences, metric_temperature: e.target.value })
                    }
                  >
                    <option value="C">Celsius (°C)</option>
                    <option value="K">Kelvin (K)</option>
                  </select>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'imperial' && (
            <div className="space-y-6">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Length Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.imperial_length}
                    onChange={(e) =>
                      setPreferences({ ...preferences, imperial_length: e.target.value })
                    }
                  >
                    <option value="in">Inch (in)</option>
                    <option value="ft">Foot (ft)</option>
                    <option value="yd">Yard (yd)</option>
                    <option value="mi">Mile (mi)</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Mass Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.imperial_mass}
                    onChange={(e) =>
                      setPreferences({ ...preferences, imperial_mass: e.target.value })
                    }
                  >
                    <option value="oz">Ounce (oz)</option>
                    <option value="lb">Pound (lb)</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Time Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.imperial_time}
                    onChange={(e) =>
                      setPreferences({ ...preferences, imperial_time: e.target.value })
                    }
                  >
                    <option value="s">Second (s)</option>
                    <option value="min">Minute (min)</option>
                    <option value="hr">Hour (hr)</option>
                    <option value="day">Day (day)</option>
                    <option value="month">Month (month)</option>
                    <option value="year">Year (year)</option>
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Temperature Unit
                  </label>
                  <select
                    className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    value={preferences.imperial_temperature}
                    onChange={(e) =>
                      setPreferences({ ...preferences, imperial_temperature: e.target.value })
                    }
                  >
                    <option value="F">Fahrenheit (°F)</option>
                  </select>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'digital' && (
            <div className="space-y-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Default Digital Storage Unit
                </label>
                <p className="text-sm text-gray-600 mb-4">
                  Choose the default unit for displaying digital storage and token values
                </p>
                <select
                  className="w-full max-w-md px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  value={preferences.digital_storage_unit}
                  onChange={(e) =>
                    setPreferences({ ...preferences, digital_storage_unit: e.target.value })
                  }
                >
                  <option value="B">Byte (B)</option>
                  <option value="KB">Kilobyte (KB)</option>
                  <option value="MB">Megabyte (MB)</option>
                  <option value="GB">Gigabyte (GB)</option>
                  <option value="TB">Terabyte (TB)</option>
                  <option value="PB">Petabyte (PB)</option>
                </select>
              </div>

              <div className="bg-blue-50 border border-blue-200 rounded p-4">
                <div className="flex gap-2">
                  <span className="text-blue-600">ℹ️</span>
                  <div className="text-sm text-blue-900">
                    <p className="font-medium mb-1">Note:</p>
                    <p>
                      This setting applies to all digital storage and token units (Tok, MTok, etc.) when no specific unit is entered.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'rates' && (
            <div className="space-y-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Time Unit for Rates
                </label>
                <p className="text-sm text-gray-600 mb-4">
                  When displaying rate units (e.g., $/hr, mi/hr), convert the time denominator to this unit
                </p>
                <select
                  className="w-full max-w-md px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  value={preferences.time_rate_unit}
                  onChange={(e) =>
                    setPreferences({ ...preferences, time_rate_unit: e.target.value })
                  }
                >
                  <option value="s">Second (s)</option>
                  <option value="min">Minute (min)</option>
                  <option value="hr">Hour (hr)</option>
                  <option value="day">Day (day)</option>
                  <option value="month">Month (month)</option>
                  <option value="year">Year (year)</option>
                </select>
              </div>

              <div className="bg-blue-50 border border-blue-200 rounded p-4">
                <div className="flex gap-2">
                  <span className="text-blue-600">ℹ️</span>
                  <div className="text-sm text-blue-900">
                    <p className="font-medium mb-1">Example:</p>
                    <p>
                      If you enter "100 $/hr" and set this to "month", the cell will display as "$/month" in Metric or Imperial mode with the value automatically converted to the monthly rate.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'currency' && (
            <div className="space-y-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Preferred Currency
                </label>
                <select
                  className="w-full max-w-md px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  value={preferences.currency}
                  onChange={(e) =>
                    setPreferences({ ...preferences, currency: e.target.value })
                  }
                >
                  {Object.keys(preferences.currency_rates).map((curr) => (
                    <option key={curr} value={curr}>
                      {curr}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-3">
                  Currency Exchange Rates (relative to USD)
                </label>
                <div className="space-y-3 max-w-2xl">
                  {Object.entries(preferences.currency_rates)
                    .sort(([a], [b]) => a.localeCompare(b))
                    .map(([currency, rate]) => (
                      <div key={currency} className="flex items-center gap-4">
                        <div className="w-20 font-medium text-gray-700">{currency}</div>
                        <div className="flex-1 flex items-center gap-2">
                          <span className="text-gray-600">=</span>
                          <input
                            type="number"
                            step="0.01"
                            className="flex-1 px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                            value={rate}
                            onChange={(e) => {
                              const newRate = parseFloat(e.target.value);
                              if (!isNaN(newRate)) {
                                setPreferences({
                                  ...preferences,
                                  currency_rates: {
                                    ...preferences.currency_rates,
                                    [currency]: newRate,
                                  },
                                });
                              }
                            }}
                            disabled={currency === 'USD'}
                          />
                          <span className="text-gray-600 w-12">USD</span>
                        </div>
                      </div>
                    ))}
                </div>
              </div>

              <div className="bg-yellow-50 border border-yellow-200 rounded p-4">
                <div className="flex gap-2">
                  <span className="text-yellow-600">⚠️</span>
                  <div className="text-sm text-yellow-900">
                    <p className="font-medium mb-1">Important:</p>
                    <p>
                      Exchange rates should be updated regularly for accurate conversions. Rates are relative to 1 USD.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="border-t border-gray-200 px-6 py-4 flex justify-end gap-3">
          <button
            className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded"
            onClick={onClose}
            disabled={saving}
          >
            Cancel
          </button>
          <button
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:bg-gray-400"
            onClick={handleSave}
            disabled={saving}
          >
            {saving ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </div>
    </div>
  );
}
