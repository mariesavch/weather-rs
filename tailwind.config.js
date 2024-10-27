
module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
    ],
  theme: {
    extend: {
      colors: {
        red: 'var(--red)',
        pink: 'var(--pink)',
        mauve: 'var(--pink)',
        peach: 'var(--peach)',
        yellow: 'var(--yellow)',
        green: 'var(--green)',
        teal: 'var(--teal)',
        blue: 'var(--blue)',
        text: 'var(--text)',
        subtext1: 'var(--subtext1)',
        subtext0: 'var(--subtext0)',
        overlay2: 'var(--overlay2)',
        overlay1: 'var(--overlay1)',
        overlay0: 'var(--overlay0)',
        surface2: 'var(--surface2)',
        surface1: 'var(--surface1)',
        surface0: 'var(--surface0)',
        base: 'var(--base)',
        mantle: 'var(--mantle)',
        crust: 'var(--crust)',
      },
      fontFamily: {
        sans: "Cartograph CF",
        mono: "Cartograph CF",
      },
    },
  },
  future: {
    hoverOnlyWhenSupported: true,
  },
}
