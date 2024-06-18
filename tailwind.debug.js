/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        "./src/**/*.{html,rs}",
        "./*.{html,js,css}"
    ],
    safelist: [
        {
            // https://design2tailwind.com/blog/tailwindcss-generate-all-classes/
            pattern: /./, // the "." means "everything"
        },
    ],
    theme: {},
    variants: {},
    plugins: [
        // require('@tailwindcss/typography'),
        // require('@tailwindcss/forms'),
        // require('@tailwindcss/aspect-ratio'),
        // require('@tailwindcss/container-queries'),
        require('daisyui')
    ],
};