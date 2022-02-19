import { createApp } from 'vue'
import App from './App.vue'
import './index.css'

// we have to reference the wasm url here, otherwise wasm doesn't get bundled for some reason.
// If we reference it only from worker, it's missing in production build.
// FIXME report this as a bug in Vite
import wasm_path from '../crate/pkg/webzebra_bg.wasm?url'
// log, because bundler would eliminate the import if we didn't use it
console.log('wasm path: ' + wasm_path)


createApp(App).mount('#app')
