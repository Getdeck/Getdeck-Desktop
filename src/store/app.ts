// Utilities
import { defineStore } from 'pinia'

export const useAppStore = defineStore('appStore', {
  state: () => {
    return {
      connection: {
        clusterName: "",
        kubeconfigPath: "",
        connected: false,
      },
      auth: {
        authenticated: false,
        user: "",
      }
    }
  },
})
