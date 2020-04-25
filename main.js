import init, { run_app } from './pkg/webgl.js';
async function main() {
   await init('/pkg/webgl_bg.wasm');
   run_app();
}
main()