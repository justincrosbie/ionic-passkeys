import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'ionic.passkeys',
  appName: 'ionic-passkeys',
  webDir: 'dist',
  server: {
    androidScheme: 'https'
  }
};

export default config;
