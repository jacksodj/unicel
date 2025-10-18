/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Custom colors for unit warnings
        'warning': '#ff9500',
        'warning-light': '#fff4e6',
      },
      spacing: {
        'safe-top': 'env(safe-area-inset-top)',
        'safe-bottom': 'env(safe-area-inset-bottom)',
        'safe-left': 'env(safe-area-inset-left)',
        'safe-right': 'env(safe-area-inset-right)',
      },
      screens: {
        // iPhone SE: 375px
        'xs': '375px',
        // iPhone 13/14: 390px
        'sm': '390px',
        // iPhone 14 Pro Max / iPad Mini portrait: 430px / 744px
        'md': '768px',
        // iPad Air portrait: 820px
        'lg': '1024px',
        // iPad Pro portrait / iPad landscape: 1024px+
        'xl': '1280px',
        // iPad Pro 12.9" landscape
        '2xl': '1366px',
        // Custom breakpoints for specific devices
        'iphone-se': { 'raw': '(max-width: 375px)' },
        'iphone-max': { 'raw': '(min-width: 428px) and (max-width: 932px)' },
        'ipad-mini': { 'raw': '(min-width: 744px) and (max-width: 1133px)' },
        'ipad-air': { 'raw': '(min-width: 820px) and (max-width: 1180px)' },
        'ipad-pro': { 'raw': '(min-width: 1024px)' },
        'landscape': { 'raw': '(orientation: landscape)' },
        'portrait': { 'raw': '(orientation: portrait)' },
      },
    },
  },
  plugins: [],
}
