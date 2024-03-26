// for use simd instructions

#define _WIN32
#define _MSC_VER 1401
#define __i386__
#define __SSE__
#define __SSE2__
#define __SSE3__
#define __SSSE3__
#define __SSE4_1__

#define BT_USE_SSE
#define BT_USE_SSE_IN_API
#define BT_NO_SIMD_OPERATOR_OVERLOADS
#define BT_USE_SIMD_VECTOR3

//

// for manual global memory initialization

extern "C" {
    extern void __wasm_call_ctors();
}

//

// override allocation functions

#include <stdlib.h>
#include <new>

void* operator new(size_t size) {
    return bw_malloc(size);
}

void* operator new[](size_t size) {
    return bw_malloc(size);
}

void operator delete(void* ptr) noexcept {
    bw_free(ptr);
}

void operator delete(void* ptr, size_t size) noexcept {
    bw_free(ptr);
}

void operator delete[](void* ptr) noexcept {
    bw_free(ptr);
}

void operator delete[](void* ptr, size_t size) noexcept {
    bw_free(ptr);
}

//

// for use SIMD instructions

#include <smmintrin.h>

//

#include "LinearMath/btMinMax.h"
#include "LinearMath/btScalar.h"
#include "LinearMath/btVector3.h"
#include "LinearMath/btQuadWord.h"
#include "LinearMath/btQuaternion.h"
#include "LinearMath/btMatrix3x3.h"
#include "LinearMath/btTransform.h"
#include "LinearMath/btAlignedAllocator.h"
#include "LinearMath/btAlignedObjectArray.h"
#include "BulletCollision/BroadphaseCollision/btBroadphaseProxy.h"
#include "LinearMath/btMotionState.h"
#include "LinearMath/btHashMap.h"
#include "LinearMath/btSerializer.h"
#include "BulletCollision/CollisionDispatch/btCollisionObject.h"
#include "BulletCollision/CollisionShapes/btCollisionShape.h"
#include "BulletCollision/CollisionShapes/btCollisionMargin.h"
#include "LinearMath/btConvexHullComputer.h"
#include "LinearMath/btGeometryUtil.h"
#include "LinearMath/btGrahamScan2dConvexHull.h"
#include "BulletCollision/CollisionShapes/btConvexPolyhedron.h"
#include "LinearMath/btAabbUtil2.h"
#include "BulletCollision/CollisionShapes/btConvexInternalShape.h"
#include "BulletCollision/CollisionShapes/btPolyhedralConvexShape.h"
#include "BulletCollision/CollisionShapes/btBoxShape.h"
#include "BulletCollision/CollisionShapes/btTriangleShape.h"
#include "BulletCollision/CollisionShapes/btSphereShape.h"
#include "BulletCollision/CollisionShapes/btCylinderShape.h"
#include "BulletCollision/CollisionShapes/btConeShape.h"
#include "BulletCollision/CollisionShapes/btCapsuleShape.h"
#include "BulletCollision/CollisionShapes/btConvexHullShape.h"
#include "BulletCollision/CollisionShapes/btConvexPointCloudShape.h"
#include "BulletCollision/CollisionShapes/btConvexShape.h"
#include "LinearMath/btTransformUtil.h"
#include "BulletDynamics/ConstraintSolver/btSolverBody.h"
#include "BulletDynamics/ConstraintSolver/btJacobianEntry.h"
#include "BulletDynamics/ConstraintSolver/btSolverConstraint.h"
#include "BulletDynamics/ConstraintSolver/btTypedConstraint.h"
#include "BulletDynamics/Dynamics/btRigidBody.h"

// test extern functions

extern "C" int bt_get_version() {
    return btGetVersion();
}

extern "C" float bt_sin(float x) {
    return btSin(x);
}

extern "C" int* bt_alloc_int() {
    int* boxed = new int(3);
    return boxed;
}

extern "C" int* bt_nonallocnew_test() {
    void* ptr = malloc(sizeof(int));
    int* boxed = new(ptr) int(3);
    return boxed;
}

extern "C" void bt_free_int(int* ptr) {
    delete ptr;
}

extern "C" void bt_transform_test() {
    btTransform t1;
    btTransform t2;
    btTransform t3 = t1 * t2;
}

extern "C" int bt_vector3_test() {
    btVector3 v1(1, 2, 3);
    btVector3 v2(4, 5, 6);
    btVector3 v3 = v1 + v2;
    return v3.x() + v3.y() + v3.z();
}

//
