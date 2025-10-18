/**
 * Orientation detection hook
 *
 * Detects and monitors device orientation changes
 */

import { useEffect, useState } from 'react';

export type Orientation = 'portrait' | 'landscape';

export interface OrientationInfo {
  orientation: Orientation;
  isPortrait: boolean;
  isLandscape: boolean;
  angle: number;
}

export function useOrientation(): OrientationInfo {
  const [orientation, setOrientation] = useState<Orientation>(getOrientation());
  const [angle, setAngle] = useState<number>(getAngle());

  useEffect(() => {
    const handleOrientationChange = () => {
      setOrientation(getOrientation());
      setAngle(getAngle());
    };

    // Listen to both resize and orientation change events
    window.addEventListener('resize', handleOrientationChange);
    window.addEventListener('orientationchange', handleOrientationChange);

    // Initial check
    handleOrientationChange();

    return () => {
      window.removeEventListener('resize', handleOrientationChange);
      window.removeEventListener('orientationchange', handleOrientationChange);
    };
  }, []);

  return {
    orientation,
    isPortrait: orientation === 'portrait',
    isLandscape: orientation === 'landscape',
    angle,
  };
}

/**
 * Get current orientation from window dimensions
 */
function getOrientation(): Orientation {
  if (window.matchMedia('(orientation: portrait)').matches) {
    return 'portrait';
  }
  return 'landscape';
}

/**
 * Get screen orientation angle
 */
function getAngle(): number {
  // Modern API
  if (window.screen.orientation) {
    return window.screen.orientation.angle;
  }

  // Fallback to deprecated API
  if ('orientation' in window) {
    return (window as any).orientation || 0;
  }

  return 0;
}
