/**
 * Accessibility preferences hook
 *
 * Detects iOS accessibility settings:
 * - VoiceOver (screen reader)
 * - Dynamic Type (text scaling)
 * - Reduce Motion
 * - High Contrast
 */

import { useEffect, useState } from 'react';

export interface AccessibilityPreferences {
  prefersReducedMotion: boolean;
  prefersHighContrast: boolean;
  prefersColorScheme: 'light' | 'dark';
  fontSize: 'small' | 'medium' | 'large' | 'x-large';
  isVoiceOverEnabled: boolean;
}

export function useAccessibility(): AccessibilityPreferences {
  const [preferences, setPreferences] = useState<AccessibilityPreferences>(getPreferences());

  useEffect(() => {
    // Create media query listeners
    const reducedMotionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    const highContrastQuery = window.matchMedia('(prefers-contrast: high)');
    const darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');

    const updatePreferences = () => {
      setPreferences(getPreferences());
    };

    // Listen for changes
    reducedMotionQuery.addEventListener('change', updatePreferences);
    highContrastQuery.addEventListener('change', updatePreferences);
    darkModeQuery.addEventListener('change', updatePreferences);

    // Check for VoiceOver (iOS specific check via focus events)
    const checkVoiceOver = () => {
      // VoiceOver detection is challenging; we use heuristics
      // This will be refined with real device testing
      const isVoiceOver = document.body.classList.contains('voiceover-enabled');
      if (isVoiceOver) {
        updatePreferences();
      }
    };

    window.addEventListener('focus', checkVoiceOver, true);

    return () => {
      reducedMotionQuery.removeEventListener('change', updatePreferences);
      highContrastQuery.removeEventListener('change', updatePreferences);
      darkModeQuery.removeEventListener('change', updatePreferences);
      window.removeEventListener('focus', checkVoiceOver, true);
    };
  }, []);

  return preferences;
}

/**
 * Get current accessibility preferences
 */
function getPreferences(): AccessibilityPreferences {
  const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  const highContrast = window.matchMedia('(prefers-contrast: high)').matches;
  const darkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;

  // Detect font size preference from root font size
  const rootFontSize = parseFloat(getComputedStyle(document.documentElement).fontSize);
  let fontSize: 'small' | 'medium' | 'large' | 'x-large' = 'medium';

  if (rootFontSize >= 20) {
    fontSize = 'x-large';
  } else if (rootFontSize >= 18) {
    fontSize = 'large';
  } else if (rootFontSize < 14) {
    fontSize = 'small';
  }

  // VoiceOver detection (heuristic - requires real device testing)
  const isVoiceOver = document.body.classList.contains('voiceover-enabled') ||
                      // Check if any element has aria-live or role attributes (indicates screen reader usage)
                      document.querySelectorAll('[aria-live], [role]').length > 5;

  return {
    prefersReducedMotion: reducedMotion,
    prefersHighContrast: highContrast,
    prefersColorScheme: darkMode ? 'dark' : 'light',
    fontSize,
    isVoiceOverEnabled: isVoiceOver,
  };
}

/**
 * Get animation duration based on reduced motion preference
 */
export function getAnimationDuration(preferences: AccessibilityPreferences, normalDuration: number): number {
  return preferences.prefersReducedMotion ? 0 : normalDuration;
}

/**
 * Get font size multiplier based on Dynamic Type preference
 */
export function getFontSizeMultiplier(preferences: AccessibilityPreferences): number {
  switch (preferences.fontSize) {
    case 'small': return 0.875;
    case 'medium': return 1.0;
    case 'large': return 1.125;
    case 'x-large': return 1.25;
  }
}
