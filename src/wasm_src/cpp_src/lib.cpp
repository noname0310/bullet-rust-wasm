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
#include "LinearMath/btThreads.h"
#include "BulletCollision/BroadphaseCollision/btBroadphaseInterface.h"
#include "BulletCollision/BroadphaseCollision/btDbvt.h"
#include "BulletCollision/BroadphaseCollision/btOverlappingPairCallback.h"
#include "BulletCollision/BroadphaseCollision/btCollisionAlgorithm.h"
#include "BulletCollision/BroadphaseCollision/btDispatcher.h"
#include "BulletCollision/BroadphaseCollision/btOverlappingPairCache.h"
#include "BulletCollision/BroadphaseCollision/btDbvtBroadphase.h"
#include "BulletCollision/CollisionDispatch/btActivatingCollisionAlgorithm.h"
#include "BulletCollision/NarrowPhaseCollision/btGjkPairDetector.h"
#include "BulletCollision/CollisionDispatch/btConvexConvexAlgorithm.h"
#include "BulletCollision/CollisionDispatch/btEmptyCollisionAlgorithm.h"
#include "BulletCollision/CollisionDispatch/btConvexConcaveCollisionAlgorithm.h"
#include "BulletCollision/CollisionShapes/btMiniSDF.h"

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

extern "C" void* bt_create_dbvtbroadphase() {
    btDbvtBroadphase* broadphase = new btDbvtBroadphase();
    return broadphase;
}

extern "C" void bt_delete_dbvtbroadphase(void* broadphase) {
    delete static_cast<btDbvtBroadphase*>(broadphase);
}

extern "C" void* bt_create_rigidbody() {
    btRigidBody::btRigidBodyConstructionInfo info(0, 0, 0);
    btRigidBody* body = new btRigidBody(info);
    return body;
}

extern "C" void bt_delete_rigidbody(void* body) {
    delete static_cast<btRigidBody*>(body);
}

extern "C" void bt_link_test() {
    btSphereShape shape1(1);
    btSphereShape shape2(1);
    btGjkPairDetector detector(&shape1, &shape2, 0, 0);

    btRigidBody body1(0, 0, 0);

    class Dispatcher : public btDispatcher {
        btCollisionAlgorithm* findAlgorithm(const btCollisionObjectWrapper* body0Wrap, const btCollisionObjectWrapper* body1Wrap, btPersistentManifold* sharedManifold, ebtDispatcherQueryType queryType) {
            return nullptr;
        }
        btPersistentManifold* getNewManifold(const btCollisionObject* b0, const btCollisionObject* b1) {
            return nullptr;
        }
        void releaseManifold(btPersistentManifold* manifold) {}
        void clearManifold(btPersistentManifold* manifold) {}
        bool needsCollision(const btCollisionObject* body0, const btCollisionObject* body1) {
            return false;
        }
        bool needsResponse(const btCollisionObject* body0, const btCollisionObject* body1) {
            return false;
        }
        void dispatchAllCollisionPairs(btOverlappingPairCache* pairCache, const btDispatcherInfo& dispatchInfo, btDispatcher* dispatcher) {}
        int getNumManifolds() const {
            return 0;
        }
        btPersistentManifold* getManifoldByIndexInternal(int index) {
            return nullptr;
        }

        btPersistentManifold** getInternalManifoldPointer() {
            return nullptr;
        }

        btPoolAllocator* getInternalManifoldPool() {
            return nullptr;
        }

        const btPoolAllocator* getInternalManifoldPool() const {
            return nullptr;
        }

        void* allocateCollisionAlgorithm(int size) {
            return nullptr;
        }

        void freeCollisionAlgorithm(void* algo) {}
    };

    btPersistentManifold manifold;
    
    Dispatcher dispatcher1;
    btCollisionAlgorithmConstructionInfo info1(&dispatcher1, 0);
    btCollisionObjectWrapper wrapper1(nullptr, &shape1, &body1, btTransform::getIdentity(), 0, 0);
    btCollisionObjectWrapper wrapper2(nullptr, &shape2, &body1, btTransform::getIdentity(), 0, 0);
    btConvexConvexAlgorithm algorithm(&manifold, info1, nullptr, nullptr, nullptr, 0, 0);
    
    Dispatcher dispatcher2;
    btCollisionAlgorithmConstructionInfo info2(&dispatcher2, 0);
    btEmptyAlgorithm empty_algorithm(info2);

    Dispatcher dispatcher3;
    btCollisionAlgorithmConstructionInfo info3(&dispatcher3, 0);
    btConvexConcaveCollisionAlgorithm concave_algorithm(info3, &wrapper1, &wrapper2, false);
    btMiniSDF sdf;
    double s;
    btVector3 v;
    sdf.interpolate(0, s, btVector3(0, 0, 0), &v);
}

//
