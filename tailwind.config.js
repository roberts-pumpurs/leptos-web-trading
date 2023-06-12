/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ['*.html', './app/**/*.rs'],
  },
  theme: {
    extend: {
      gap: {
        '10': '7rem',
      },
      width: {
        '25': '25rem'
      }
    },
  },
  plugins: [require('@tailwindcss/forms')],
};
