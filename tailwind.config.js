/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        "./src/**/*.{html,rs}",
        "./*.{html,js,css}"
    ],
    theme: {},
    variants: {},
    plugins: [
        require('daisyui')
    ],
};