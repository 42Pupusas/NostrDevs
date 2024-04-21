/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.{html,rs}', 'index.html'],

    plugins: [
        require('@tailwindcss/typography'),
        require('@tailwindcss/forms'),
    ],
    theme: {
        fontFamily: {
            'source': ['SourceCodePro', 'sans-serif'],
        },
    },
};
