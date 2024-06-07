import "@babylonjs/core/Loading/loadingScreen";

import { ArcRotateCamera } from "@babylonjs/core/Cameras/arcRotateCamera";
import type { AbstractEngine } from "@babylonjs/core/Engines/abstractEngine";
import { DirectionalLight } from "@babylonjs/core/Lights/directionalLight";
import { HemisphericLight } from "@babylonjs/core/Lights/hemisphericLight";
import { CascadedShadowGenerator } from "@babylonjs/core/Lights/Shadows/cascadedShadowGenerator";
import { ShadowGenerator } from "@babylonjs/core/Lights/Shadows/shadowGenerator";
import { Color3, Color4 } from "@babylonjs/core/Maths/math.color";
import { Vector3 } from "@babylonjs/core/Maths/math.vector";
import { CreateBox } from "@babylonjs/core/Meshes/Builders/boxBuilder";
import { CreatePlane } from "@babylonjs/core/Meshes/Builders/planeBuilder";
import { Scene } from "@babylonjs/core/scene";
import { Inspector } from "@babylonjs/inspector";

import type { ISceneBuilder } from "./baseRuntime";
import * as wasmBindgen from "./wasm/index";

export class SceneBuilder implements ISceneBuilder {
    public async build(_canvas: HTMLCanvasElement, engine: AbstractEngine): Promise<Scene> {
        const scene = new Scene(engine);
        scene.clearColor = new Color4(0.95, 0.95, 0.95, 1.0);

        const camera = new ArcRotateCamera("arcRotateCamera", 0, 0, 150, new Vector3(0, 10, 0), scene);
        camera.minZ = 1;
        camera.maxZ = 5000;
        camera.setPosition(new Vector3(0, 10, -45));
        camera.attachControl(undefined, false);
        camera.inertia = 0.8;
        camera.speed = 10;

        const hemisphericLight = new HemisphericLight("hemisphericLight", new Vector3(0, 1, 0), scene);
        hemisphericLight.intensity = 0.5;
        hemisphericLight.specular = new Color3(0, 0, 0);
        hemisphericLight.groundColor = new Color3(1, 1, 1);

        const directionalLight = new DirectionalLight("directionalLight", new Vector3(0.5, -1, 1), scene);
        directionalLight.intensity = 0.5;

        const shadowGenerator = new CascadedShadowGenerator(1024, directionalLight);
        shadowGenerator.transparencyShadow = true;
        shadowGenerator.usePercentageCloserFiltering = true;
        shadowGenerator.forceBackFacesOnly = false;
        shadowGenerator.lambda = 0.96;
        shadowGenerator.bias = 0.007;
        shadowGenerator.filteringQuality = ShadowGenerator.QUALITY_MEDIUM;
        shadowGenerator.stabilizeCascades = true;

        const ground = CreatePlane("ground", { size: 50 }, scene);
        ground.rotation = new Vector3(Math.PI / 2, 0, 0);

        shadowGenerator.addShadowCaster(ground);
        ground.receiveShadows = true;

        for (let i = 0; i < 10; ++i) {
            for (let j = 0; j < 10; ++j) {
                const box = CreateBox("box", { size: 2 }, scene);
                box.position.x = j * 3 - 15 + (i % 2) * 1.5;
                box.position.y = 4 + i * 3;

                shadowGenerator.addShadowCaster(box);
                box.receiveShadows = true;
            }
        }

        await wasmBindgen.default();
        wasmBindgen.init();
        await wasmBindgen.initThreadPool(2);

        const now = performance.now();
        wasmBindgen.test();
        console.log("Time taken:", performance.now() - now);

        Inspector.Show(scene, { });
        return scene;
    }
}
