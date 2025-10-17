/**
 * Platform detection hook for iOS/mobile devices
 *
 * Usage:
 * const { isMobile, isTablet, isTouchDevice, isIOS } = useMobile();
 */

import { useEffect, useState } from 'react';

export interface MobilePlatform {
  isMobile: boolean;
  isTablet: boolean;
  isTouchDevice: boolean;
  isIOS: boolean;
}

export function useMobile(): MobilePlatform {
  const [isMobile, setIsMobile] = useState(false);
  const [isTablet, setIsTablet] = useState(false);

  useEffect(() => {
    const checkPlatform = () => {
      const userAgent = navigator.userAgent;

      // Detect phone vs tablet
      setIsMobile(/iPhone|iPod/.test(userAgent));
      setIsTablet(/iPad/.test(userAgent));
    };

    checkPlatform();

    // Re-check on window resize (for responsive testing)
    window.addEventListener('resize', checkPlatform);
    return () => window.removeEventListener('resize', checkPlatform);
  }, []);

  return {
    isMobile,
    isTablet,
    isTouchDevice: isMobile || isTablet,
    isIOS: isMobile || isTablet,
  };
}
