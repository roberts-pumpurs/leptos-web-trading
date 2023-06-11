/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ['*.html', './app/**/*.rs'],
  },
  theme: {
    extend: {},
  },
  corePlugins: {
    overflowY: true,
  },
  plugins: [require('@tailwindcss/forms')],
};
