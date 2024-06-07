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

        const camera = new ArcRotateCamera("arcRotateCamera", 0, 0, 500, new Vector3(0, -50, 0), scene);
        camera.minZ = 1;
        camera.maxZ = 5000;
        camera.setPosition(new Vector3(300, 500, -500));
        camera.attachControl(undefined, false);
        camera.inertia = 0.8;
        camera.speed = 10;

        const hemisphericLight = new HemisphericLight("hemisphericLight", new Vector3(0, 1, 0), scene);
        hemisphericLight.intensity = 0.5;
        hemisphericLight.specular = new Color3(0, 0, 0);
        hemisphericLight.groundColor = new Color3(1, 1, 1);

        const directionalLight = new DirectionalLight("directionalLight", new Vector3(0.5, -1, 1), scene);
        directionalLight.intensity = 0.5;
        directionalLight.shadowMaxZ = 300;
        directionalLight.shadowMinZ = -300;

        const shadowGenerator = new ShadowGenerator(2048, directionalLight, true);
        shadowGenerator.transparencyShadow = true;
        shadowGenerator.usePercentageCloserFiltering = true;
        shadowGenerator.forceBackFacesOnly = false;
        shadowGenerator.bias = 0.004;
        shadowGenerator.filteringQuality = ShadowGenerator.QUALITY_MEDIUM;

        const wasmInternal = await wasmBindgen.default();
        wasmBindgen.init();
        await wasmBindgen.initThreadPool(navigator.hardwareConcurrency);

        const xCount = 16;
        const yCount = 50;
        const worldMatrixBufferSize = 16 * xCount * yCount;
        const worldColorBufferSize = 3 * xCount * yCount;
        const worldCount = Math.max(Math.min(32, Math.floor(navigator.hardwareConcurrency * 0.75)), 1);
        const physicsObjectId = 0;

        const worldXMargin = 120;
        const worldZMargin = 90;
        const worldZWidth = Math.ceil(worldCount / 4);

        const multiPhysicsWorld = wasmBindgen.createMultiPhysicsWorld();

        const ground = CreatePlane("ground", { size: 600 }, scene);
        ground.rotationQuaternion = Quaternion.RotationAxis(new Vector3(1, 0, 0), Math.PI / 2);
        shadowGenerator.addShadowCaster(ground);
        ground.receiveShadows = true;
        ground.scaling.y = (worldZWidth * worldZMargin + worldZMargin) / 600;

        const baseBox = CreateBox("box", { size: 2 }, scene);
        // baseBox.isEnabled(false);
        baseBox.scaling.set(1, 1, 1);
        shadowGenerator.addShadowCaster(baseBox);
        baseBox.receiveShadows = true;

        const rigidbodyMatrixBuffer = new Float32Array(worldMatrixBufferSize * worldCount);
        const colorBuffer = new Float32Array(worldColorBufferSize * worldCount);

        for (let worldIndex = 0; worldIndex < worldCount; ++worldIndex) {
            const physicsWorld = wasmBindgen.createPhysicsWorld();

            physicsWorld.createPhysicsObject(physicsObjectId);

            // Ground
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

            const position = new Vector3();
            const rotation = new Quaternion();
            const scale = new Vector3(1, 1, 1);
            const matrix = new Matrix();

            const worldXOffset = worldIndex % 4;
            const worldZOffset = Math.floor(worldIndex / 4);

            for (let i = 0; i < yCount; ++i) {
                for (let j = 0; j < xCount; ++j) {

                    position.set(
                        j * 3 + (i % 2) * 1.5 - xCount * 3 / 2 + (worldXOffset * worldXMargin - worldXMargin * 1.5),
                        i * 2 + 2,
                        worldZOffset * worldZMargin - (worldZWidth === 0 ? 0 : worldZWidth - 1) * worldZMargin / 2
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
                    rigidBodyHandle;
                    info.free();

                    const bufferIndex = i * xCount + j;

                    Matrix.ComposeToRef(scale, rotation, position, matrix);
                    matrix.copyToArray(rigidbodyMatrixBuffer, bufferIndex * 16 + worldIndex * worldMatrixBufferSize);

                    colorBuffer[bufferIndex * 3 + 0 + worldIndex * worldColorBufferSize] = Math.random();
                    colorBuffer[bufferIndex * 3 + 1 + worldIndex * worldColorBufferSize] = Math.random();
                    colorBuffer[bufferIndex * 3 + 2 + worldIndex * worldColorBufferSize] = Math.random();
                }
            }

            scene.onDisposeObservable.addOnce(() => {
                physicsWorld.destroyPhysicsObject(physicsObjectId);
                physicsWorld.free();
            });

            multiPhysicsWorld.addPhysicsWorld(physicsWorld);
        }

        baseBox.thinInstanceSetBuffer("matrix", rigidbodyMatrixBuffer, 16, false);
        baseBox.thinInstanceSetBuffer("color", colorBuffer, 3, true);

        scene.onDisposeObservable.addOnce(() => {
            multiPhysicsWorld.free();
        });

        scene.registerBeforeRender(() => {
            // const deltaTime = scene.getEngine().getDeltaTime() / 1000;
            // physicsWorld.stepSimulation(1 / 60, 120, 1 / 120);

            multiPhysicsWorld.stepSimulation(1 / 60, 10, 1 / 60);

            for (let worldIndex = 0; worldIndex < worldCount; ++worldIndex) {
                const transformPtr = multiPhysicsWorld.getTransforms(worldIndex, physicsObjectId);
                const wasmMatrixBuffer = new Float32Array(wasmInternal.memory.buffer, transformPtr + 4 * 16, worldMatrixBufferSize); // + 4 * 16 to skip the plane rigidbody
                rigidbodyMatrixBuffer.set(wasmMatrixBuffer, worldIndex * worldMatrixBufferSize);
            }

            baseBox.thinInstanceBufferUpdated("matrix");
        });

        const defaultPipeline = new DefaultRenderingPipeline("default", true, scene);
        defaultPipeline.samples = 4;

        const statDiv = document.createElement("div");
        statDiv.style.position = "absolute";
        statDiv.style.top = "0";
        statDiv.style.left = "0";
        statDiv.style.color = "white";
        statDiv.style.backgroundColor = "rgba(0, 0, 0, 0.5)";
        statDiv.style.padding = "4px";
        statDiv.style.fontFamily = "Arial";
        statDiv.style.fontSize = "12px";
        document.body.appendChild(statDiv);

        const fpsElement = document.createElement("div");
        statDiv.appendChild(fpsElement);
        scene.onBeforeRenderObservable.add(() => {
            fpsElement.textContent = `${engine.getFps().toFixed()} FPS`;
        });

        const rigidbodyCountElement = document.createElement("div");
        statDiv.appendChild(rigidbodyCountElement);
        rigidbodyCountElement.textContent = `Rigidbody Count: ${xCount * yCount * worldCount}`;

        const rigidbodyCountPerThreadElement = document.createElement("div");
        statDiv.appendChild(rigidbodyCountPerThreadElement);
        rigidbodyCountPerThreadElement.textContent = `Rigidbody Count Per World: ${xCount * yCount}`;

        const threadCountElement = document.createElement("div");
        statDiv.appendChild(threadCountElement);
        threadCountElement.textContent = `Thread Count: ${navigator.hardwareConcurrency}`;

        Inspector.Show;//(scene, { });
        return scene;
    }
}
