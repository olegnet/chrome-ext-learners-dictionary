/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        "./**/*.{html,js,css}",
        "./src/**/*.{rs,html,css}",
        "./pkg/**/*.{html,js,css}"
    ],
    theme: {},
    variants: {},
    plugins: [
        require('daisyui')
    ],
};