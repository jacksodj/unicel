/**
 * Performance optimization utilities for mobile
 *
 * Provides debouncing, throttling, and memoization helpers
 * for achieving 60fps rendering on mobile devices
 */

/**
 * Debounce function - delays execution until after wait milliseconds have elapsed
 * since the last time it was invoked
 *
 * @param func Function to debounce
 * @param wait Wait time in milliseconds
 * @returns Debounced function
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;

  return function executedFunction(...args: Parameters<T>) {
    const later = () => {
      timeout = null;
      func(...args);
    };

    if (timeout !== null) {
      clearTimeout(timeout);
    }
    timeout = setTimeout(later, wait);
  };
}

/**
 * Throttle function - ensures function is called at most once per specified time period
 *
 * @param func Function to throttle
 * @param limit Time limit in milliseconds
 * @returns Throttled function
 */
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean;
  let lastResult: ReturnType<T>;

  return function executedFunction(...args: Parameters<T>) {
    if (!inThrottle) {
      inThrottle = true;
      lastResult = func(...args);

      setTimeout(() => {
        inThrottle = false;
      }, limit);
    }

    return lastResult;
  };
}

/**
 * Request animation frame wrapper for smooth 60fps animations
 *
 * @param callback Function to execute on next frame
 * @returns Cancel function
 */
export function requestFrame(callback: () => void): () => void {
  const handle = requestAnimationFrame(callback);
  return () => cancelAnimationFrame(handle);
}

/**
 * Batch multiple DOM updates into a single animation frame
 *
 * @param updates Array of update functions
 */
export function batchUpdates(updates: Array<() => void>): void {
  requestAnimationFrame(() => {
    updates.forEach((update) => update());
  });
}

/**
 * Measure rendering performance
 *
 * @param label Performance marker label
 * @returns Timing measurement object
 */
export function measurePerformance(label: string) {
  const start = performance.now();

  return {
    end: () => {
      const duration = performance.now() - start;
      console.log(`[Performance] ${label}: ${duration.toFixed(2)}ms`);
      return duration;
    },
  };
}

/**
 * Check if running on iOS
 */
export function isIOS(): boolean {
  return /iPad|iPhone|iPod/.test(navigator.userAgent);
}

/**
 * Check if running on Safari
 */
export function isSafari(): boolean {
  return /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
}

/**
 * Optimize scroll performance with passive event listeners
 *
 * @param element Element to attach listener to
 * @param handler Scroll handler
 * @returns Cleanup function
 */
export function optimizeScroll(
  element: HTMLElement,
  handler: (event: Event) => void
): () => void {
  element.addEventListener('scroll', handler, { passive: true });

  return () => {
    element.removeEventListener('scroll', handler);
  };
}

/**
 * Lazy load component with intersection observer
 *
 * @param element Element to observe
 * @param callback Callback when element is visible
 * @param options Intersection observer options
 * @returns Cleanup function
 */
export function lazyLoad(
  element: HTMLElement,
  callback: () => void,
  options: IntersectionObserverInit = {}
): () => void {
  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        callback();
        observer.unobserve(entry.target);
      }
    });
  }, options);

  observer.observe(element);

  return () => {
    observer.disconnect();
  };
}

/**
 * Memory-efficient cache with size limit
 */
export class LRUCache<K, V> {
  private max: number;
  private cache: Map<K, V>;

  constructor(max: number = 100) {
    this.max = max;
    this.cache = new Map();
  }

  get(key: K): V | undefined {
    const item = this.cache.get(key);
    if (item !== undefined) {
      // Refresh the item (move to end)
      this.cache.delete(key);
      this.cache.set(key, item);
    }
    return item;
  }

  set(key: K, value: V): void {
    // Delete and re-insert to move to end
    if (this.cache.has(key)) {
      this.cache.delete(key);
    }
    this.cache.set(key, value);

    // Evict oldest item if over capacity
    if (this.cache.size > this.max) {
      const firstKey = this.cache.keys().next().value;
      if (firstKey !== undefined) {
        this.cache.delete(firstKey);
      }
    }
  }

  has(key: K): boolean {
    return this.cache.has(key);
  }

  clear(): void {
    this.cache.clear();
  }

  get size(): number {
    return this.cache.size;
  }
}
