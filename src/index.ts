import * as wasm from "./wasm/index";

const wasmInstance = await wasm.default();
wasmInstance.init();

console.log(wasmInstance.testFunction(2));
console.log(wasmInstance.allocationTest(10));
