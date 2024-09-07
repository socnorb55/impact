module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs"
  ],
  theme: {
    colors: {
      "cordavon": "#904E55",
      "cosmic-latte": "#FFF8E7",
      "eerie-black": "#252627",
      "wenge": "#564E58",
      "zomp": "#629677",
    }
  },
  plugins: [
    require("@tailwindcss/forms")
  ]
}