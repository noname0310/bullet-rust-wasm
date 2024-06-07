import "@babylonjs/core/Loading/loadingScreen";

import { ArcRotateCamera } from "@babylonjs/core/Cameras/arcRotateCamera";
import type { AbstractEngine } from "@babylonjs/core/Engines/abstractEngine";
import { DirectionalLight } from "@babylonjs/core/Lights/directionalLight";
import { HemisphericLight } from "@babylonjs/core/Lights/hemisphericLight";
import { ShadowGenerator } from "@babylonjs/core/Lights/Shadows/shadowGenerator";
import { Color3, Color4 } from "@babylonjs/core/Maths/math.color";
import { Matrix, Quaternion, Vector3 } from "@babylonjs/core/Maths/math.vector";
import { CreateBox } from "@babylonjs/core/Meshes/Builders/boxBuilder";
import { CreatePlane } from "@babylonjs/core/Meshes/Builders/planeBuilder";
import { DefaultRenderingPipeline } from "@babylonjs/core/PostProcesses/RenderPipeline/Pipelines/defaultRenderingPipeline";
import { Scene } from "@babylonjs/core/scene";
import { Inspector } from "@babylonjs/inspector";

import type { ISceneBuilder } from "./baseRuntime";
import * as wasmBindgen from "./wasm/index";

export class SceneBuilder implements ISceneBuilder {
    public async build(_canvas: HTMLCanvasElement, engine: AbstractEngine): Promise<Scene> {
        const scene = new Scene(engine);
        scene.clearColor = new Color4(0.95, 0.95, 0.95, 1.0);

        const camera = new ArcRotateCamera("arcRotateCamera", 0, 0, 200, new Vector3(0, 20, 0), scene);
        camera.minZ = 1;
        camera.maxZ = 5000;
        camera.setPosition(new Vector3(40, 90, -200));
        camera.attachControl(undefined, false);
        camera.inertia = 0.8;
        camera.speed = 10;

        const hemisphericLight = new HemisphericLight("hemisphericLight", new Vector3(0, 1, 0), scene);
        hemisphericLight.intensity = 0.5;
        hemisphericLight.specular = new Color3(0, 0, 0);
        hemisphericLight.groundColor = new Color3(1, 1, 1);

        const directionalLight = new DirectionalLight("directionalLight", new Vector3(0.5, -1, 1), scene);
        directionalLight.intensity = 0.5;
        directionalLight.shadowMaxZ = 200;
        directionalLight.shadowMinZ = -200;

        const shadowGenerator = new ShadowGenerator(2048, directionalLight, true);
        shadowGenerator.transparencyShadow = true;
        shadowGenerator.usePercentageCloserFiltering = true;
        shadowGenerator.forceBackFacesOnly = false;
        shadowGenerator.bias = 0.004;
        shadowGenerator.filteringQuality = ShadowGenerator.QUALITY_MEDIUM;

        const wasmInternal = await wasmBindgen.default();
        wasmBindgen.init();
        await wasmBindgen.initThreadPool(2);

        const physicsWorld = wasmBindgen.createPhysicsWorld();

        const xCount = 8;
        const yCount = 100;

        const physicsObjectId = 0;
        const rigidbodyHandles = new Int32Array(xCount * yCount);
        const rigidbodyMatrixBuffer = new Float32Array(16 * xCount * yCount);
        const colorBuffer = new Float32Array(3 * xCount * yCount);
        physicsWorld.createPhysicsObject(physicsObjectId);

        const ground = CreatePlane("ground", { size: 250 }, scene);
        ground.rotationQuaternion = Quaternion.RotationAxis(new Vector3(1, 0, 0), Math.PI / 2);
        shadowGenerator.addShadowCaster(ground);
        ground.receiveShadows = true;
        {
            const info = wasmBindgen.createRigidbodyConstructionInfo();
            info.setMass(0.0);
            info.setMotionType(2); // Static
            info.setShapeType(5); // StaticPlane
            info.setShapeSize(0, 0, -1, 0); // Normal and constant
            info.setFriction(0.5);
            info.setStartTransform(
                ground.position.x,
                ground.position.y,
                ground.position.z,
                ground.rotationQuaternion!.x,
                ground.rotationQuaternion!.y,
                ground.rotationQuaternion!.z,
                ground.rotationQuaternion!.w
            );
            physicsWorld.createRigidbody(physicsObjectId, info);
            info.free();
        }

        const baseBox = CreateBox("box", { size: 2 }, scene);
        baseBox.isEnabled(false);
        shadowGenerator.addShadowCaster(baseBox);
        baseBox.receiveShadows = true;

        const position = new Vector3();
        const rotation = new Quaternion();
        const scale = new Vector3(1, 1, 1);
        const matrix = new Matrix();

        for (let i = 0; i < yCount; ++i) {
            for (let j = 0; j < xCount; ++j) {
                position.set(
                    j * 3 + (i % 2) * 1.5 - xCount * 3 / 2,
                    i * 2 + 2,
                    0
                );
                rotation.set(0, 0, 0, 1);

                const info = wasmBindgen.createRigidbodyConstructionInfo();
                info.setMass(1.0);
                info.setMotionType(0); // Dynamic
                info.setShapeType(0); // Box
                info.setShapeSize(1, 1, 1, 0);
                info.setFriction(1.0);
                info.setStartTransform(
                    position.x,
                    position.y,
                    position.z,
                    0, 0, 0, 1
                );

                const rigidBodyHandle = physicsWorld.createRigidbody(physicsObjectId, info);
                info.free();

                const bufferIndex = i * xCount + j;
                rigidbodyHandles[bufferIndex] = rigidBodyHandle;

                Matrix.ComposeToRef(scale, rotation, position, matrix);
                matrix.copyToArray(rigidbodyMatrixBuffer, bufferIndex * 16);

                colorBuffer[bufferIndex * 3 + 0] = Math.random();
                colorBuffer[bufferIndex * 3 + 1] = Math.random();
                colorBuffer[bufferIndex * 3 + 2] = Math.random();
            }
        }

        baseBox.thinInstanceSetBuffer("matrix", rigidbodyMatrixBuffer, 16, false);
        baseBox.thinInstanceSetBuffer("color", colorBuffer, 3, true);

        scene.onDisposeObservable.addOnce(() => {
            physicsWorld.destroyPhysicsObject(physicsObjectId);
            physicsWorld.free();
        });

        scene.registerBeforeRender(() => {
            // const deltaTime = scene.getEngine().getDeltaTime() / 1000;
            // physicsWorld.stepSimulation(1 / 60, 120, 1 / 120);

            physicsWorld.stepSimulation(1 / 60, 10, 1 / 60);

            const transformPtr = physicsWorld.getTransforms(physicsObjectId);
            const wasmMatrixBuffer = new Float32Array(wasmInternal.memory.buffer, transformPtr, rigidbodyMatrixBuffer.length);
            rigidbodyMatrixBuffer.set(wasmMatrixBuffer);

            baseBox.thinInstanceBufferUpdated("matrix");
        });

        const defaultPipeline = new DefaultRenderingPipeline("default", true, scene);
        defaultPipeline.samples = 4;

        const fpsElement = document.createElement("div");
        fpsElement.style.position = "absolute";
        fpsElement.style.top = "0";
        fpsElement.style.left = "0";
        fpsElement.style.color = "white";
        fpsElement.style.backgroundColor = "rgba(0, 0, 0, 0.5)";
        fpsElement.style.padding = "4px";
        fpsElement.style.fontFamily = "Arial";
        fpsElement.style.fontSize = "12px";
        document.body.appendChild(fpsElement);
        if (fpsElement) {
            scene.onBeforeRenderObservable.add(() => {
                fpsElement.textContent = `${engine.getFps().toFixed()} FPS`;
            });
        }

        Inspector.Show;//(scene, { });
        return scene;
    }
}
