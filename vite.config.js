import { defineConfig } from 'vite';
import UnoCSS from 'unocss/vite';
import aurelia from '@aurelia/vite-plugin';

export default defineConfig({
  plugins: [
    UnoCSS({
      configFile: './unocss.config.ts',
    }),
    aurelia()],
});
