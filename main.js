import init, { run_app } from './pkg/my_little_habitat.js';
async function main() {
   await init('/pkg/my_little_habitat_bg.wasm');
   run_app();
}
main()