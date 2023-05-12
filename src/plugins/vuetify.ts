/**
 * plugins/vuetify.ts
 *
 * Framework documentation: https://vuetifyjs.com`
 */

// Styles
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

// Composables
import { createVuetify } from 'vuetify'

// https://vuetifyjs.com/en/introduction/why-vuetify/#feature-guides
export default createVuetify({
  defaults: {
    global: {
      ripple: false,
    },
    VTooltip: {
      location: "bottom",
      offset: "2"
    },
    VTextField: {
      variant: "outlined",
    }
  },
  theme: {
    themes: {
      light: {
        colors: {
          "on-background": "#303846",
          primary: "#ff1654",
          secondary: "#303846",
          'secondary-2': "#303846"
        },
      },
    },
  },
})
