/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["./src/**/*.rs"],
    extract: {
      rs: (content) => {
        const candidates = content.match(/\."?([\w:\/_-]+)"?/g)?.map((str) => str.substring(1).replace(/"/g, '')) ?? [];
        return candidates;
      }
    }
  },
  theme: {
    extend: {},
  },
  plugins: [],
}

