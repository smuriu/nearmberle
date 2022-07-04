import { defineConfig, transform } from 'windicss/helpers'
import typography from 'windicss/plugin/typography'
import colors from 'daisyui/src/colors'

export default defineConfig({
  plugins: [
    typography,
    transform('daisyui')
  ],
  theme: {
    extend: {
      colors
    }
  },
  daisyui: {
    themes: ['coffee'],
  },
})
