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
    },
  },
  plugins: [],
}
