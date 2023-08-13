import daisyui from "daisyui";

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,rs}", "index.html"],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
}

