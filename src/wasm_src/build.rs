fn main() {
    cc::Build::new()
        .warnings(false)
        .archiver("llvm-ar")
        .cpp_link_stdlib(None)
        .cpp(true)
        .flag("-xc++")
        .flag("-matomics")
        .flag("-mbulk-memory")
        .flag("-msimd128")
        .flag("-Wno-c++11-narrowing")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-#pragma-messages")
        .flag("-fno-exceptions")

        // for use simd instructions
        .define("_WIN32", None)
        .define("_MSC_VER", "1401")
        .define("__i386__", None)
        .define("__SSE__", None)
        .define("__SSE2__", None)
        .define("__SSE3__", None)
        .define("__SSSE3__", None)
        .define("__SSE4_1__", None)
        .define("BT_USE_SSE", None)
        // .define("BT_USE_SSE_IN_API", None)
        .define("BT_NO_SIMD_OPERATOR_OVERLOADS", None)
        .define("BT_USE_SIMD_VECTOR3", None)
        .define("__wasm32__", None)
        
        .includes([
            "cpp_wasm_std",
            "cpp_src"
        ])
        .files([
            "cpp_src/lib.cpp",
            "cpp_wasm_std/cxa_guard.cpp",
            "cpp_wasm_std/stdio.cpp",
            "cpp_wasm_std/string_c.cpp",
            "cpp_wasm_std/string.cpp",
            "cpp_wasm_std/windows.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btBroadphaseProxy.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btDbvt.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btDbvtBroadphase.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btDispatcher.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btOverlappingPairCache.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btQuantizedBvh.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btSimpleBroadphase.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btActivatingCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btBoxBoxCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btBoxBoxDetector.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btCollisionWorld.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btCollisionDispatcher.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btCollisionObject.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btCompoundCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btCompoundCompoundCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btConvexConcaveCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btConvexConvexAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btConvexPlaneCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btDefaultCollisionConfiguration.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btEmptyCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btHashedSimplePairCache.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btManifoldResult.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btSimulationIslandManager.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btSphereSphereCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btSphereTriangleCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btUnionFind.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/SphereTriangleDetector.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btBoxShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btBvhTriangleMeshShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btCapsuleShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btCollisionShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btCompoundShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConcaveShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConeShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexHullShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexInternalShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexPointCloudShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexPolyhedron.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexTriangleMeshShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btCylinderShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btHeightfieldTerrainShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btMiniSDF.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btMinkowskiSumShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btMultiSphereShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btOptimizedBvh.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btPolyhedralConvexShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btScaledBvhTriangleMeshShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btSdfCollisionShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btSphereShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btStaticPlaneShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btStridingMeshInterface.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btTriangleCallback.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btTriangleMeshShape.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btContinuousConvexCollision.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btConvexCast.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btGjkConvexCast.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btGjkEpa2.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btGjkEpaPenetrationDepthSolver.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btGjkPairDetector.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btMinkowskiPenetrationDepthSolver.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btPersistentManifold.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btPolyhedralContactClipping.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btRaycastCallback.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btSubSimplexConvexCast.cpp",
            "cpp_src/BulletCollision/NarrowPhaseCollision/btVoronoiSimplexSolver.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btConeTwistConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btContactConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btGeneric6DofConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btGeneric6DofSpring2Constraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btGeneric6DofSpringConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btHingeConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btPoint2PointConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btSliderConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btTypedConstraint.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btSequentialImpulseConstraintSolver.cpp",
            "cpp_src/BulletDynamics/Dynamics/btDiscreteDynamicsWorld.cpp",
            "cpp_src/BulletDynamics/Dynamics/btRigidBody.cpp",
            "cpp_src/LinearMath/btAlignedAllocator.cpp",
            "cpp_src/LinearMath/btConvexHullComputer.cpp",
            "cpp_src/LinearMath/btGeometryUtil.cpp",
            "cpp_src/LinearMath/btQuickprof.cpp",
            "cpp_src/LinearMath/btSerializer.cpp",
            "cpp_src/LinearMath/btThreads.cpp",
            "cpp_src/LinearMath/btVector3.cpp",
        ])
        .opt_level_str("fast")
        .compile("bullet");

    println!("cargo:rerun-if-changed=cpp_wasm_std");
    println!("cargo:rerun-if-changed=cpp_src/lib.h");
}
