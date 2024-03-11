import * as wasm from "./wasm/index";

const wasmInstance = await wasm.default();
wasmInstance.init();
