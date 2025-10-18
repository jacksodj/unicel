/**
 * Device detection utilities for iOS
 *
 * Detects specific iOS devices and screen sizes for responsive layouts
 */

export type DeviceType = 'iphone-se' | 'iphone-standard' | 'iphone-max' | 'ipad-mini' | 'ipad-air' | 'ipad-pro';

export interface DeviceInfo {
  type: DeviceType;
  screenWidth: number;
  screenHeight: number;
  isSmallPhone: boolean;
  isLargePhone: boolean;
  isTablet: boolean;
  isLargeTablet: boolean;
}

/**
 * Detect device type based on screen dimensions
 */
export function detectDevice(): DeviceInfo {
  const width = window.screen.width;
  const height = window.screen.height;
  const minDimension = Math.min(width, height);
  const maxDimension = Math.max(width, height);

  // iPhone SE: 375x667 (4.7" effective)
  if (minDimension <= 375 && maxDimension <= 667) {
    return {
      type: 'iphone-se',
      screenWidth: width,
      screenHeight: height,
      isSmallPhone: true,
      isLargePhone: false,
      isTablet: false,
      isLargeTablet: false,
    };
  }

  // iPhone 14 Pro Max: 430x932 (6.7")
  if (minDimension >= 414 && maxDimension >= 896) {
    return {
      type: 'iphone-max',
      screenWidth: width,
      screenHeight: height,
      isSmallPhone: false,
      isLargePhone: true,
      isTablet: false,
      isLargeTablet: false,
    };
  }

  // iPhone 13/14: 390x844 (6.1" standard)
  if (minDimension >= 375 && minDimension < 414) {
    return {
      type: 'iphone-standard',
      screenWidth: width,
      screenHeight: height,
      isSmallPhone: false,
      isLargePhone: false,
      isTablet: false,
      isLargeTablet: false,
    };
  }

  // iPad Mini: 744x1133 (8.3")
  if (minDimension >= 744 && minDimension < 820) {
    return {
      type: 'ipad-mini',
      screenWidth: width,
      screenHeight: height,
      isSmallPhone: false,
      isLargePhone: false,
      isTablet: true,
      isLargeTablet: false,
    };
  }

  // iPad Pro 12.9": 1024x1366
  if (minDimension >= 1024) {
    return {
      type: 'ipad-pro',
      screenWidth: width,
      screenHeight: height,
      isSmallPhone: false,
      isLargePhone: false,
      isTablet: true,
      isLargeTablet: true,
    };
  }

  // iPad Air: 820x1180 (10.9")
  return {
    type: 'ipad-air',
    screenWidth: width,
    screenHeight: height,
    isSmallPhone: false,
    isLargePhone: false,
    isTablet: true,
    isLargeTablet: false,
  };
}

/**
 * Get responsive breakpoint for current device
 */
export function getBreakpoint(): 'sm' | 'md' | 'lg' | 'xl' {
  const width = window.innerWidth;

  if (width < 640) return 'sm';  // Phone portrait
  if (width < 768) return 'md';  // Phone landscape / small tablet
  if (width < 1024) return 'lg'; // iPad portrait
  return 'xl';                   // iPad landscape / iPad Pro
}

/**
 * Calculate optimal grid cell size for device
 */
export function getGridCellSize(deviceInfo: DeviceInfo): { width: number; height: number } {
  if (deviceInfo.isSmallPhone) {
    return { width: 80, height: 32 }; // Compact for small screens
  }

  if (deviceInfo.isLargePhone || !deviceInfo.isTablet) {
    return { width: 100, height: 36 }; // Standard phone size
  }

  if (deviceInfo.isLargeTablet) {
    return { width: 120, height: 40 }; // Spacious for iPad Pro
  }

  return { width: 110, height: 38 }; // Standard iPad size
}

/**
 * Calculate visible cells for device and orientation
 */
export function getVisibleCells(deviceInfo: DeviceInfo, isLandscape: boolean): { columns: number; rows: number } {
  const cellSize = getGridCellSize(deviceInfo);
  const width = isLandscape ? deviceInfo.screenHeight : deviceInfo.screenWidth;
  const height = isLandscape ? deviceInfo.screenWidth : deviceInfo.screenHeight;

  // Account for toolbar (60px) and status bar (40px) and safe areas (44px top + 34px bottom)
  const availableHeight = height - 60 - 40 - 44 - 34;
  const availableWidth = width - 40; // Safe area margins

  return {
    columns: Math.floor(availableWidth / cellSize.width),
    rows: Math.floor(availableHeight / cellSize.height),
  };
}
