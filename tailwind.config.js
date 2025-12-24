/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // macOS-inspired color palette
        'macos': {
          'sidebar': 'rgba(246, 246, 246, 0.8)',
          'sidebar-dark': 'rgba(30, 30, 30, 0.8)',
          'bg': '#ffffff',
          'bg-dark': '#1e1e1e',
          'text': '#1d1d1f',
          'text-dark': '#f5f5f7',
          'accent': '#007aff',
          'accent-hover': '#0066d6',
          'success': '#34c759',
          'warning': '#ff9500',
          'danger': '#ff3b30',
          'border': '#d2d2d7',
          'border-dark': '#424245',
        },
      },
      fontFamily: {
        'system': ['-apple-system', 'BlinkMacSystemFont', 'SF Pro Text', 'Helvetica Neue', 'sans-serif'],
      },
      backdropBlur: {
        'macos': '20px',
      },
      borderRadius: {
        'macos': '10px',
        'macos-lg': '14px',
      },
      boxShadow: {
        'macos': '0 0 0 1px rgba(0, 0, 0, 0.1), 0 2px 8px rgba(0, 0, 0, 0.1)',
        'macos-dark': '0 0 0 1px rgba(255, 255, 255, 0.1), 0 2px 8px rgba(0, 0, 0, 0.3)',
      },
    },
  },
  plugins: [],
}
