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

#include "btBulletDynamicsMinimal.h"

// extern functions

enum class bwRigidBodyShapeType : uint8_t {
    BOX = 0,
    SPHERE = 1,
    CAPSULE = 2,
    // CYLINDER = 3,
    // CONE = 4,
    STATIC_PLANE = 5,
    // MESH = 6,
    // CONVEX_HULL = 7,
    // COMPOUND = 8
};

enum class bwRigidBodyMotionType : uint8_t {
    DYNAMIC = 0,
    KINEMATIC = 1,
    STATIC = 2
};

struct bwRigidBodyConstructionInfo final {
    // for shape
    bwRigidBodyShapeType m_shapeType = bwRigidBodyShapeType::BOX;
    btVector4 m_shapeSize = btVector4(1.0f, 1.0f, 1.0f, 0.0f);

    // for motion state
    bwRigidBodyMotionType m_motionType = bwRigidBodyMotionType::DYNAMIC;
    btTransform m_startTransform = btTransform::getIdentity();
    
    // for rigid body
    btScalar m_mass = 1.0f;
    btScalar m_linearDamping = 0.0f;
    btScalar m_angularDamping = 0.0f;
    btScalar m_friction = 0.5f;
    btScalar m_restitution = 0.0f;
    bool m_additionalDamping = false;
    uint16_t m_collisionGroup = 0x0001;
    uint16_t m_collisionMask = 0xFFFF;
    btScalar m_linearSleepingThreshold = 0.8f;
    btScalar m_angularSleepingThreshold = 1.0f;
    bool m_disableDeactivation = false;
};

extern "C" void* bt_create_rigidbody_construction_info() {
    bwRigidBodyConstructionInfo* info = new bwRigidBodyConstructionInfo();
    return info;
}

extern "C" void bt_destroy_rigidbody_construction_info(void* info) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    delete i;
}

extern "C" void bt_rigidbody_construction_info_set_shape_type(void* info, uint8_t shapeType) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_shapeType = static_cast<bwRigidBodyShapeType>(shapeType);
}

extern "C" void bt_rigidbody_construction_info_set_shape_size(void* info, float* size_buffer) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_shapeSize = btVector4(size_buffer[0], size_buffer[1], size_buffer[2], size_buffer[3]);
}

extern "C" void bt_rigidbody_construction_info_set_motion_type(void* info, uint8_t motionType) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_motionType = static_cast<bwRigidBodyMotionType>(motionType);
}

extern "C" void bt_rigidbody_construction_info_set_start_transform(void* info, float* position_buffer, float* rotation_buffer) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_startTransform = btTransform(
        btQuaternion(rotation_buffer[0], rotation_buffer[1], rotation_buffer[2], rotation_buffer[3]),
        btVector3(position_buffer[0], position_buffer[1], position_buffer[2])
    );
}

extern "C" void bt_rigidbody_construction_info_set_mass(void* info, float mass) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_mass = mass;
}

extern "C" void bt_rigidbody_construction_info_set_damping(void* info, float linearDamping, float angularDamping) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_linearDamping = linearDamping;
    i->m_angularDamping = angularDamping;
}

extern "C" void bt_rigidbody_construction_info_set_friction(void* info, float friction) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_friction = friction;
}

extern "C" void bt_rigidbody_construction_info_set_restitution(void* info, float restitution) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_restitution = restitution;
}

extern "C" void bt_rigidbody_construction_info_set_additional_damping(void* info, uint8_t additionalDamping) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_additionalDamping = additionalDamping;
}

extern "C" void bt_rigidbody_construction_info_set_collision_group_mask(void* info, uint16_t collisionGroup, uint16_t collisionMask) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_collisionGroup = collisionGroup;
    i->m_collisionMask = collisionMask;
}

extern "C" void bt_rigidbody_construction_info_set_sleeping_threshold(void* info, float linearSleepingThreshold, float angularSleepingThreshold) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_linearSleepingThreshold = linearSleepingThreshold;
    i->m_angularSleepingThreshold = angularSleepingThreshold;
}

extern "C" void bt_rigidbody_construction_info_set_disable_deactivation(void* info, uint8_t disableDeactivation) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    i->m_disableDeactivation = disableDeactivation;
}

class bwRigidBody final {
private:
    btCollisionShape* m_shape;
    btDefaultMotionState* m_motionState;
    btRigidBody* m_body;
    uint16_t m_collisionGroup;
    uint16_t m_collisionMask;

public:
    bwRigidBody(bwRigidBodyConstructionInfo* info) {
        switch (info->m_shapeType) {
            case bwRigidBodyShapeType::BOX:
                m_shape = new btBoxShape(info->m_shapeSize);
                break;
            case bwRigidBodyShapeType::SPHERE:
                m_shape = new btSphereShape(info->m_shapeSize.x());
                break;
            case bwRigidBodyShapeType::CAPSULE:
                m_shape = new btCapsuleShape(info->m_shapeSize.x(), info->m_shapeSize.y());
                break;
            case bwRigidBodyShapeType::STATIC_PLANE:
                m_shape = new btStaticPlaneShape(
                    btVector3(info->m_shapeSize.x(), info->m_shapeSize.y(), info->m_shapeSize.z()),
                    info->m_shapeSize.w()
                );
                break;
            default:
                m_shape = nullptr;
                break;
        }

        switch (info->m_motionType) {
            case bwRigidBodyMotionType::DYNAMIC:
                m_motionState = new btDefaultMotionState(info->m_startTransform);
                break;
            case bwRigidBodyMotionType::KINEMATIC:
                m_motionState = new btDefaultMotionState(info->m_startTransform);
                break;
            case bwRigidBodyMotionType::STATIC:
                m_motionState = new btDefaultMotionState(info->m_startTransform);
                break;
            default:
                m_motionState = nullptr;
                break;
        }

        btVector3 localInertia(0.0f, 0.0f, 0.0f);
        if (info->m_mass != 0.0f) {
            m_shape->calculateLocalInertia(info->m_mass, localInertia);
        }

        btRigidBody::btRigidBodyConstructionInfo rbInfo(info->m_mass, m_motionState, m_shape, localInertia);
        rbInfo.m_linearDamping = info->m_linearDamping;
        rbInfo.m_angularDamping = info->m_angularDamping;
        rbInfo.m_friction = info->m_friction;
        rbInfo.m_restitution = info->m_restitution;
        rbInfo.m_additionalDamping = info->m_additionalDamping;

        m_body = new btRigidBody(rbInfo);
        m_body->setSleepingThresholds(info->m_linearSleepingThreshold, info->m_angularSleepingThreshold);
        if (info->m_disableDeactivation) {
            m_body->setActivationState(DISABLE_DEACTIVATION);
        }
        
        m_collisionGroup = info->m_collisionGroup;
        m_collisionMask = info->m_collisionMask;
    }

    bwRigidBody(bwRigidBody const&) = delete;
    bwRigidBody& operator=(bwRigidBody const&) = delete;

    ~bwRigidBody() {
        delete m_body;
        delete m_motionState;
        delete m_shape;
    }

    btRigidBody* getBody() {
        return m_body;
    }

    const btRigidBody* getBody() const {
        return m_body;
    }

    uint16_t getCollisionGroup() const {
        return m_collisionGroup;
    }

    uint16_t getCollisionMask() const {
        return m_collisionMask;
    }

    void getTransform(btScalar* transform_buffer) const {
        btTransform& transform = m_body->getWorldTransform();
        transform.getOpenGLMatrix(transform_buffer);
    }

    void setTransform(btScalar* transform_buffer) {
        btTransform transform;
        transform.setFromOpenGLMatrix(transform_buffer);
        m_body->setWorldTransform(transform);
    }
};

extern "C" void* bt_create_rigidbody(void* info) {
    bwRigidBodyConstructionInfo* i = static_cast<bwRigidBodyConstructionInfo*>(info);
    bwRigidBody* body = new bwRigidBody(i);
    return body;
}

extern "C" void bt_destroy_rigidbody(void* body) {
    bwRigidBody* b = static_cast<bwRigidBody*>(body);
    delete b;
}

extern "C" void bt_rigidbody_get_transform(void* body, float* transform_buffer) {
    bwRigidBody* b = static_cast<bwRigidBody*>(body);
    b->getTransform(transform_buffer);
}

extern "C" void bt_rigidbody_set_transform(void* body, float* transform_buffer) {
    bwRigidBody* b = static_cast<bwRigidBody*>(body);
    b->setTransform(transform_buffer);
}

enum class bwConstraintType : uint8_t {
    // POINT2POINT = 0,
    // HINGE = 1,
    // CONE_TWIST = 2,
    GENERIC_6DOF = 3,
    // SLIDER = 4,
    GENERIC_6DOF_SPRING = 5,
    // UNIVERSAL = 6,
    // HINGE2 = 7,
    // GEAR = 8,
    // FIXED = 9
};

struct bwConstraintConstructionInfo final {
    bwConstraintType m_type = bwConstraintType::GENERIC_6DOF;
    bwRigidBody* m_bodyA = nullptr;
    bwRigidBody* m_bodyB = nullptr;
    btTransform m_frameA = btTransform::getIdentity();
    btTransform m_frameB = btTransform::getIdentity();
    bool m_useLinearReferenceFrameA = true;
    bool m_disableCollisionsBetweenLinkedBodies = false;
    btVector3 m_linearLowerLimit = btVector3(0.0f, 0.0f, 0.0f);
    btVector3 m_linearUpperLimit = btVector3(0.0f, 0.0f, 0.0f);
    btVector3 m_angularLowerLimit = btVector3(0.0f, 0.0f, 0.0f);
    btVector3 m_angularUpperLimit = btVector3(0.0f, 0.0f, 0.0f);
    btVector3 m_linearStiffness = btVector3(0.0f, 0.0f, 0.0f);
    btVector3 m_angularStiffness = btVector3(0.0f, 0.0f, 0.0f);
};

extern "C" void* bt_create_constraint_construction_info() {
    bwConstraintConstructionInfo* info = new bwConstraintConstructionInfo();
    return info;
}

extern "C" void bt_destroy_constraint_construction_info(void* info) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    delete i;
}

extern "C" void bt_constraint_construction_info_set_type(void* info, uint8_t type) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_type = static_cast<bwConstraintType>(type);
}

extern "C" void bt_constraint_construction_info_set_bodies(void* info, void* bodyA, void* bodyB) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_bodyA = static_cast<bwRigidBody*>(bodyA);
    i->m_bodyB = static_cast<bwRigidBody*>(bodyB);
}

extern "C" void bt_constraint_construction_info_set_frames(void* info, float* frameA_buffer, float* frameB_buffer) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_frameA.setFromOpenGLMatrix(frameA_buffer);
    i->m_frameB.setFromOpenGLMatrix(frameB_buffer);
}

extern "C" void bt_constraint_construction_info_set_use_linear_reference_frame_a(void* info, uint8_t useLinearReferenceFrameA) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_useLinearReferenceFrameA = useLinearReferenceFrameA;
}

extern "C" void bt_constraint_construction_info_set_disable_collisions_between_linked_bodies(void* info, uint8_t disableCollisionsBetweenLinkedBodies) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_disableCollisionsBetweenLinkedBodies = disableCollisionsBetweenLinkedBodies;
}

extern "C" void bt_constraint_construction_info_set_linear_limits(void* info, float* lowerLimit_buffer, float* upperLimit_buffer) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_linearLowerLimit = btVector3(lowerLimit_buffer[0], lowerLimit_buffer[1], lowerLimit_buffer[2]);
    i->m_linearUpperLimit = btVector3(upperLimit_buffer[0], upperLimit_buffer[1], upperLimit_buffer[2]);
}

extern "C" void bt_constraint_construction_info_set_angular_limits(void* info, float* lowerLimit_buffer, float* upperLimit_buffer) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_angularLowerLimit = btVector3(lowerLimit_buffer[0], lowerLimit_buffer[1], lowerLimit_buffer[2]);
    i->m_angularUpperLimit = btVector3(upperLimit_buffer[0], upperLimit_buffer[1], upperLimit_buffer[2]);
}

extern "C" void bt_constraint_construction_info_set_stiffness(void* info, float* linearStiffness_buffer, float* angularStiffness_buffer) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    i->m_linearStiffness = btVector3(linearStiffness_buffer[0], linearStiffness_buffer[1], linearStiffness_buffer[2]);
    i->m_angularStiffness = btVector3(angularStiffness_buffer[0], angularStiffness_buffer[1], angularStiffness_buffer[2]);
}

class bwConstraint final {
private:
    btTypedConstraint* m_constraint;
    bool m_disableCollisionsBetweenLinkedBodies;

public:
    bwConstraint(bwConstraintConstructionInfo* info) {
        switch (info->m_type) {
            case bwConstraintType::GENERIC_6DOF:
                m_constraint = new btGeneric6DofConstraint(
                    *info->m_bodyA->getBody(),
                    *info->m_bodyB->getBody(),
                    info->m_frameA,
                    info->m_frameB,
                    info->m_useLinearReferenceFrameA
                );
                break;
            case bwConstraintType::GENERIC_6DOF_SPRING:
                m_constraint = new btGeneric6DofSpringConstraint(
                    *info->m_bodyA->getBody(),
                    *info->m_bodyB->getBody(),
                    info->m_frameA,
                    info->m_frameB,
                    info->m_useLinearReferenceFrameA
                );
                break;
            default:
                m_constraint = nullptr;
                break;
        }

        if (info->m_type == bwConstraintType::GENERIC_6DOF) {
            btGeneric6DofConstraint* c = static_cast<btGeneric6DofConstraint*>(m_constraint);
            c->setLinearLowerLimit(info->m_linearLowerLimit);
            c->setLinearUpperLimit(info->m_linearUpperLimit);
            c->setAngularLowerLimit(info->m_angularLowerLimit);
            c->setAngularUpperLimit(info->m_angularUpperLimit);
        } else if (info->m_type == bwConstraintType::GENERIC_6DOF_SPRING) {
            btGeneric6DofSpringConstraint* c = static_cast<btGeneric6DofSpringConstraint*>(m_constraint);
            c->setLinearLowerLimit(info->m_linearLowerLimit);
            c->setLinearUpperLimit(info->m_linearUpperLimit);
            c->setAngularLowerLimit(info->m_angularLowerLimit);
            c->setAngularUpperLimit(info->m_angularUpperLimit);
            
            if (info->m_linearStiffness.x() != 0.0f) {
                c->setStiffness(0, info->m_linearStiffness.x());
                c->enableSpring(0, true);
            } else {
                c->enableSpring(0, false);
            }

            if (info->m_linearStiffness.y() != 0.0f) {
                c->setStiffness(1, info->m_linearStiffness.y());
                c->enableSpring(1, true);
            } else {
                c->enableSpring(1, false);
            }

            if (info->m_linearStiffness.z() != 0.0f) {
                c->setStiffness(2, info->m_linearStiffness.z());
                c->enableSpring(2, true);
            } else {
                c->enableSpring(2, false);
            }

            c->setStiffness(3, info->m_angularStiffness.x());
            c->enableSpring(3, true);
            c->setStiffness(4, info->m_angularStiffness.y());
            c->enableSpring(4, true);
            c->setStiffness(5, info->m_angularStiffness.z());
            c->enableSpring(5, true);
        }

        m_disableCollisionsBetweenLinkedBodies = info->m_disableCollisionsBetweenLinkedBodies;
    }

    bwConstraint(bwConstraint const&) = delete;
    bwConstraint& operator=(bwConstraint const&) = delete;

    ~bwConstraint() {
        delete m_constraint;
    }

    btTypedConstraint* getConstraint() {
        return m_constraint;
    }

    const btTypedConstraint* getConstraint() const {
        return m_constraint;
    }

    bool getDisableCollisionsBetweenLinkedBodies() const {
        return m_disableCollisionsBetweenLinkedBodies;
    }
};

extern "C" void* bt_create_constraint(void* info) {
    bwConstraintConstructionInfo* i = static_cast<bwConstraintConstructionInfo*>(info);
    bwConstraint* constraint = new bwConstraint(i);
    return constraint;
}

extern "C" void bt_destroy_constraint(void* constraint) {
    bwConstraint* c = static_cast<bwConstraint*>(constraint);
    delete c;
}

class bwPhysicsWorld final {
private:
    btDbvtBroadphase* m_broadphase;
    btDefaultCollisionConfiguration* m_collisionConfig;
    btCollisionDispatcher* m_dispatcher;
    btSequentialImpulseConstraintSolver* m_solver;
    btDiscreteDynamicsWorld* m_world;

public:
    bwPhysicsWorld() {
        m_broadphase = new btDbvtBroadphase();
        m_collisionConfig = new btDefaultCollisionConfiguration();
        m_dispatcher = new btCollisionDispatcher(m_collisionConfig);
        m_solver = new btSequentialImpulseConstraintSolver();
        m_world = new btDiscreteDynamicsWorld(m_dispatcher, m_broadphase, m_solver, m_collisionConfig);
    }

    bwPhysicsWorld(bwPhysicsWorld const&) = delete;
    bwPhysicsWorld& operator=(bwPhysicsWorld const&) = delete;

    ~bwPhysicsWorld() {
        delete m_world;
        delete m_solver;
        delete m_dispatcher;
        delete m_collisionConfig;
        delete m_broadphase;
    }
    
    void setGravity(btScalar x, btScalar y, btScalar z) {
        m_world->setGravity(btVector3(x, y, z));
    }

    void stepSimulation(btScalar timeStep, int maxSubSteps, btScalar fixedTimeStep) {
        m_world->stepSimulation(timeStep, maxSubSteps, fixedTimeStep);
    }

    void addRigidBody(bwRigidBody* body) {
        m_world->addRigidBody(body->getBody(), body->getCollisionGroup(), body->getCollisionMask());
    }

    void removeRigidBody(bwRigidBody* body) {
        m_world->removeRigidBody(body->getBody());
    }

    void addConstraint(bwConstraint* constraint) {
        m_world->addConstraint(constraint->getConstraint(), constraint->getDisableCollisionsBetweenLinkedBodies());
    }

    void removeConstraint(bwConstraint* constraint) {
        m_world->removeConstraint(constraint->getConstraint());
    }
};

extern "C" void* bt_create_world() {
    bwPhysicsWorld* world = new bwPhysicsWorld();
    return world;
}

extern "C" void bt_destroy_world(void* world) {
    bwPhysicsWorld* w = static_cast<bwPhysicsWorld*>(world);
    delete w;
}

extern "C" void bt_world_set_gravity(void* world, float x, float y, float z) {
    bwPhysicsWorld* w = static_cast<bwPhysicsWorld*>(world);
    w->setGravity(x, y, z);
}

extern "C" void bt_world_step_simulation(void* world, float timeStep, int maxSubSteps, float fixedTimeStep) {
    bwPhysicsWorld* w = static_cast<bwPhysicsWorld*>(world);
    w->stepSimulation(timeStep, maxSubSteps, fixedTimeStep);
}

extern "C" void bt_world_add_rigidbody(void* world, void* body) {
    bwPhysicsWorld* w = static_cast<bwPhysicsWorld*>(world);
    bwRigidBody* b = static_cast<bwRigidBody*>(body);
    w->addRigidBody(b);
}

extern "C" void bt_world_remove_rigidbody(void* world, void* body) {
    bwPhysicsWorld* w = static_cast<bwPhysicsWorld*>(world);
    bwRigidBody* b = static_cast<bwRigidBody*>(body);
    w->removeRigidBody(b);
}

extern "C" void bt_world_add_constraint(void* world, void* constraint) {
    bwPhysicsWorld* w = static_cast<bwPhysicsWorld*>(world);
    bwConstraint* c = static_cast<bwConstraint*>(constraint);
    w->addConstraint(c);
}

extern "C" void bt_world_remove_constraint(void* world, void* constraint) {
    bwPhysicsWorld* w = static_cast<bwPhysicsWorld*>(world);
    bwConstraint* c = static_cast<bwConstraint*>(constraint);
    w->removeConstraint(c);
}

//
