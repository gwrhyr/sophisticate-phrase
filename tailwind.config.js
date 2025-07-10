/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './templates/**/*.html',
  ],
  theme: {
    extend: {
      colors: {
        primary: 'oklch(50% 0.05 270)',
        secondary: 'oklch(70% 0.05 270)',
        'dark-text': 'oklch(20% 0.05 270)',
        'light-text': 'oklch(90% 0.05 270)',
        'acrylic-light': 'oklch(95% 0.02 270 / 0.6)',
        'acrylic-dark': 'oklch(98% 0.01 270 / 0.8)', // Whiter for header/footer
        'background-dark': 'oklch(20% 0.05 270)', // Darker background
        'pattern-light': 'oklch(25% 0.05 270 / 0.1)', // For background pattern
      },
    },
  },
  plugins: [],
}