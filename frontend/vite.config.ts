import { defineConfig, UserConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  const opt: UserConfig = {
    plugins: [react()],
  };
  if (mode === 'staging') {
    opt.build = {
      sourcemap: true,
    };
  }
  return opt;
});
