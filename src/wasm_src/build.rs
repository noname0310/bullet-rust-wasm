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
        
        .includes([
            "cpp_wasm_std",
            "cpp_src"
        ])
        .files([
            "cpp_src/lib.cpp",
            "cpp_wasm_std/string_c.cpp",
            "cpp_wasm_std/string.cpp",
            "cpp_wasm_std/windows.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btBroadphaseProxy.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btCollisionAlgorithm.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btDbvt.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btDbvtBroadphase.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btDispatcher.cpp",
            "cpp_src/BulletCollision/BroadphaseCollision/btOverlappingPairCache.cpp",
            "cpp_src/BulletCollision/CollisionDispatch/btCollisionObject.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btBoxShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btCapsuleShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btCollisionShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConeShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexHullShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexInternalShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexPointCloudShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btConvexPolyhedron.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btCylinderShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btPolyhedralConvexShape.cpp",
            "cpp_src/BulletCollision/CollisionShapes/btSphereShape.cpp",
            "cpp_src/BulletDynamics/ConstraintSolver/btTypedConstraint.cpp",
            "cpp_src/BulletDynamics/Dynamics/btRigidBody.cpp",
            "cpp_src/LinearMath/btAlignedAllocator.cpp",
            "cpp_src/LinearMath/btConvexHullComputer.cpp",
            "cpp_src/LinearMath/btGeometryUtil.cpp",
            "cpp_src/LinearMath/btQuickprof.cpp",
            "cpp_src/LinearMath/btSerializer.cpp",
            "cpp_src/LinearMath/btThreads.cpp",
            "cpp_src/LinearMath/btVector3.cpp",
        ])
        .compile("bullet");

    println!("cargo:rerun-if-changed=cpp_wasm_std");
    println!("cargo:rerun-if-changed=cpp_src/lib.h");
}
