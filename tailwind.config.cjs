/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: ["./src/**/*.{html,js,svelte,ts}", "./index.html"],
  theme: { extend: {} },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      "light",
      "dark",
      {
        gptranslate: {
          primary: "#2563eb",
          secondary: "#7c3aed",
          accent: "#fb923c",
          neutral: "#2a323c",
          "base-100": "#1d232a",
          info: "#0ea5e9",
          success: "#10b981",
          warning: "#fbbf24",
          error: "#ef4444",
        },
      },
    ],
  },
};
