import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: '0.0.0.0',
    proxy: {
      '/graphql': {
        target: 'http://api.mini-oss.top:30002',
        rewrite: (path) => {
          return path.replace('/graphql', '/');
        },
      },
    },
  },
});
