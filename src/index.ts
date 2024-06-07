import { Engine } from "@babylonjs/core/Engines/engine";

import { BaseRuntime } from "./baseRuntime";
import { SceneBuilder } from "./sceneBuilder";

await new Promise(resolve => window.onload = resolve);

const canvas = document.createElement("canvas");
canvas.style.width = "100%";
canvas.style.height = "100%";
canvas.style.display = "block";
document.body.appendChild(canvas);

const engine = new Engine(canvas, false, {
    preserveDrawingBuffer: false,
    stencil: true,
    antialias: true,
    alpha: false,
    premultipliedAlpha: false,
    powerPreference: "high-performance",
    doNotHandleTouchAction: false,
    doNotHandleContextLost: true,
    audioEngine: false,
    disableWebGL2Support: false
}, true);

BaseRuntime.Create({
    canvas,
    engine,
    sceneBuilder: new SceneBuilder()
}).then(runtime => runtime.run());
