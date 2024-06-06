import * as wasmBindgen from "./wasm/index";

await wasmBindgen.default();
wasmBindgen.init();
await wasmBindgen.initThreadPool(2);

const now = performance.now();
wasmBindgen.test();
console.log("Time taken:", performance.now() - now);
