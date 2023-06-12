/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ['*.html', './app/**/*.rs'],
  },
  theme: {
    extend: {
      gap: {
        '10': '7rem',
        '2': '0.5rem',
      },
      width: {
        '24': '15rem',
        '25': '18rem',
        '26': '21rem',
        '52': '16rem',
      }
    },
  },
  corePlugins: {
    overflowY: true,
  },
  plugins: [require('@tailwindcss/forms')],
};
