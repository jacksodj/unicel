/**
 * iOS Haptic Feedback Utility
 *
 * Provides typed haptic feedback for touch interactions.
 * Uses the Vibration API with patterns optimized for iOS.
 *
 * Usage:
 * import { haptics } from '@/utils/haptics';
 * haptics.light(); // On cell tap
 * haptics.medium(); // On sheet change
 * haptics.heavy(); // On error
 * haptics.success(); // On successful action
 */

export type HapticIntensity = 'light' | 'medium' | 'heavy';

export interface HapticsAPI {
  light: () => void;
  medium: () => void;
  heavy: () => void;
  success: () => void;
  warning: () => void;
  error: () => void;
  selection: () => void;
  impact: (intensity: HapticIntensity) => void;
  isSupported: () => boolean;
}

/**
 * Trigger haptic feedback with specified pattern
 */
function vibrate(pattern: number | number[]): void {
  if ('vibrate' in navigator) {
    navigator.vibrate(pattern);
  }
}

/**
 * Check if haptic feedback is supported
 */
function isSupported(): boolean {
  return 'vibrate' in navigator;
}

/**
 * Light haptic feedback (10ms)
 * Use for: Cell selection, button taps, minor interactions
 */
function light(): void {
  vibrate(10);
}

/**
 * Medium haptic feedback (20ms)
 * Use for: Sheet changes, mode toggles, moderate actions
 */
function medium(): void {
  vibrate(20);
}

/**
 * Heavy haptic feedback (30ms)
 * Use for: Errors, warnings, important alerts
 */
function heavy(): void {
  vibrate(30);
}

/**
 * Success pattern (double tap: 15ms, 10ms gap, 15ms)
 * Use for: Successful file open, successful save
 */
function success(): void {
  vibrate([15, 10, 15]);
}

/**
 * Warning pattern (medium followed by light: 20ms, 15ms gap, 10ms)
 * Use for: Unit mismatch warnings, non-critical issues
 */
function warning(): void {
  vibrate([20, 15, 10]);
}

/**
 * Error pattern (heavy triple tap: 30ms, 10ms gap, 30ms, 10ms gap, 30ms)
 * Use for: File load errors, critical errors
 */
function error(): void {
  vibrate([30, 10, 30, 10, 30]);
}

/**
 * Selection pattern (very light: 5ms)
 * Use for: Continuous selection feedback (e.g., drag selection)
 */
function selection(): void {
  vibrate(5);
}

/**
 * Generic impact feedback with specified intensity
 */
function impact(intensity: HapticIntensity): void {
  switch (intensity) {
    case 'light':
      light();
      break;
    case 'medium':
      medium();
      break;
    case 'heavy':
      heavy();
      break;
  }
}

/**
 * Haptics API
 */
export const haptics: HapticsAPI = {
  light,
  medium,
  heavy,
  success,
  warning,
  error,
  selection,
  impact,
  isSupported,
};
