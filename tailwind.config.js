/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        colors: {
          'licenciya-blue': '#007BFF',
        },
      },
    },
    plugins: [],
  }