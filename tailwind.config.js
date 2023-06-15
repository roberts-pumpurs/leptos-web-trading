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
      },
      colors: {
        'lapse': '#f3f3f3',
      },
      screens: {
        'vsm': '360px',
      }
    },
  },
  corePlugins: {
    overflowY: true,
  },
  plugins: [require('@tailwindcss/forms')],
};
