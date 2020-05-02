import init, { run_app } from './pkg/face_generator.js';
async function main() {
    await init('/pkg/face_generator_bg.wasm');
    run_app();
}
main()
