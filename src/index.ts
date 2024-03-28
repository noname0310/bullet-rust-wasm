import * as wasmBindgen from "./wasm/index";

await wasmBindgen.default();
wasmBindgen.init();
await wasmBindgen.initThreadPool(2);
wasmBindgen.test();
