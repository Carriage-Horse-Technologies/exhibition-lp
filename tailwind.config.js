const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,rs}", "./dist/**/*.{html,js}"],
  theme: {
    screens: {
      'xs': '400px',
      ...defaultTheme.screens,
    },
    extend: {
      colors: {
        "dark-primary": "#713FFF",
        "dark-primary-deep": "#5a28ff",
        "dark-primary-pale": "#825aff",
        "dark-secondary": "#ba000d",
        "dark-secondary-deep": "#960000",
        "dark-secondary-pale": "#C81E1E",
        "dark-content-background": "#111827",
        "dark-content-border": "#30363D",
        "dark-background": "#0F0F0F",
        "dark-text": "#F1F1F1",
        "dark-roundbutton-bg-hover": "#808080",
        "dark-button-bg": "#272727",
        "dark-button-border": "#2E323A",
        "dark-button-border-hover": "#808080",
        "dark-button-bg-active": "#808080",

        "light-primary": "#2196f3",
        "light-primary-deep": "#0A8CDC",
        "light-primary-pale": "#32B4FF",
        "light-secondary": "#f50057",
        "light-secondary-deep": "#DC003C",
        "light-secondary-pale": "#FF146E",
        "light-content-background": "#f3f6f4",
        "light-content-border": "#E1E5EA",
        "light-background": "#FFFFFF",
        "light-text": "#0F0F0F",
        "light-roundbutton-bg-hover": "#c0c0c0",
        "light-button-bg": "#DDDDDD",
        // NOTE: ライトテーマのみ
        "light-button-bg-hover": "#c0c0c0",
        "light-button-border": "#c0c0c0",
        "light-button-border-hover": "#808080",
        "light-button-bg-active": "#808080",

        // NOTE: 共通カラー
        "link-text": "#31A0D3",
        "enter-button": "#00ff55",
        "enter-button-deep": "#00A336",
        "picture-frame": "#FFD700",
        "picture-frame-pale": "#FFE556"
      },
      transitionTimingFunction: {
        "character-move": "cubic-bezier(.45,0,.82,.56)"
      },
      animation: {
        "slide-in-left": "slide-in-left 0.3s cubic-bezier(0.250, 0.460, 0.450, 0.940)   both",
        "slide-out-left": "slide-out-left 0.3s cubic-bezier(0.550, 0.085, 0.680, 0.530)   both"
      },
      keyframes: {
        "slide-in-left": {
          "0%": {
            transform: "translateX(-100%)",
            opacity: "0"
          },
          to: {
            transform: "translateX(0)",
            opacity: "1"
          }
        },
        "slide-out-left": {
          "0%": {
            transform: "translateX(0)",
            opacity: "1"
          },
          to: {
            transform: "translateX(-100%)",
            opacity: "0"
          }
        }
      },
      backgroundImage: {
        'content-background-img': "url('https://objectstorage.ap-tokyo-1.oraclecloud.com/n/nr7eduszgfzb/b/image-bucket/o/allo%2Fallo-bg.png')",
        "tile-bg-img": "url(https://online.futabazemi.work/unity/CosmoWall.jpg)",
        "rainbow-load-bg-img": "url(https://objectstorage.ap-tokyo-1.oraclecloud.com/n/nr7eduszgfzb/b/image-bucket/o/allo%2Frainbow-load64.jpg)",
      }
    },
  },
  plugins: [],
  darkMode: "class"
}
