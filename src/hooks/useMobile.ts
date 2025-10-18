/**
 * Platform detection hook for iOS/mobile devices
 *
 * Enhanced to detect:
 * - Device type (iPhone vs iPad vs Desktop)
 * - Screen size and orientation
 * - Tauri iOS context
 * - Touch capability
 *
 * Usage:
 * const {
 *   isMobile,
 *   isTablet,
 *   isTouchDevice,
 *   isIOS,
 *   isTauriIOS,
 *   screenSize,
 *   orientation,
 *   isLandscape,
 *   isPortrait
 * } = useMobile();
 */

import { useEffect, useState } from 'react';

export type ScreenSize = 'small' | 'medium' | 'large';
export type Orientation = 'portrait' | 'landscape';

export interface MobilePlatform {
  isMobile: boolean;
  isTablet: boolean;
  isTouchDevice: boolean;
  isIOS: boolean;
  isTauriIOS: boolean;
  screenSize: ScreenSize;
  orientation: Orientation;
  isLandscape: boolean;
  isPortrait: boolean;
  screenWidth: number;
  screenHeight: number;
}

export function useMobile(): MobilePlatform {
  const [isMobile, setIsMobile] = useState(false);
  const [isTablet, setIsTablet] = useState(false);
  const [isTauriIOS, setIsTauriIOS] = useState(false);
  const [screenSize, setScreenSize] = useState<ScreenSize>('medium');
  const [orientation, setOrientation] = useState<Orientation>('portrait');
  const [screenWidth, setScreenWidth] = useState(window.innerWidth);
  const [screenHeight, setScreenHeight] = useState(window.innerHeight);

  useEffect(() => {
    const checkPlatform = () => {
      const userAgent = navigator.userAgent;

      // Detect device type
      const isPhone = /iPhone|iPod/.test(userAgent);
      const isTabletDevice = /iPad/.test(userAgent);

      setIsMobile(isPhone);
      setIsTablet(isTabletDevice);

      // Detect if running in Tauri iOS context
      // Check for window.__TAURI__ or window.__TAURI_INTERNALS__
      const isTauri = '__TAURI__' in window || '__TAURI_INTERNALS__' in window;
      setIsTauriIOS(isTauri && (isPhone || isTabletDevice));

      // Update screen dimensions
      const width = window.innerWidth;
      const height = window.innerHeight;
      setScreenWidth(width);
      setScreenHeight(height);

      // Determine screen size
      // Small: iPhone SE, iPhone 12/13/14 mini (< 400px)
      // Medium: iPhone 12/13/14/15 (390-430px)
      // Large: iPad (> 700px)
      if (width < 400) {
        setScreenSize('small');
      } else if (width < 700) {
        setScreenSize('medium');
      } else {
        setScreenSize('large');
      }

      // Determine orientation
      const newOrientation = width > height ? 'landscape' : 'portrait';
      setOrientation(newOrientation);
    };

    checkPlatform();

    // Re-check on window resize and orientation change
    window.addEventListener('resize', checkPlatform);
    window.addEventListener('orientationchange', checkPlatform);

    return () => {
      window.removeEventListener('resize', checkPlatform);
      window.removeEventListener('orientationchange', checkPlatform);
    };
  }, []);

  return {
    isMobile,
    isTablet,
    isTouchDevice: isMobile || isTablet,
    isIOS: isMobile || isTablet,
    isTauriIOS,
    screenSize,
    orientation,
    isLandscape: orientation === 'landscape',
    isPortrait: orientation === 'portrait',
    screenWidth,
    screenHeight,
  };
}
