mod bt_bind;
mod cpp_runtime;

use bt_bind::constraint::{ConstraintConstructionInfo, ConstraintType};
use bt_bind::physics_world::PhysicsWorld;
use bt_bind::rigidbody::{MotionType, RigidbodyConstructionInfo, ShapeType};
use glam::{Mat4, Quat, Vec3, Vec4};
use rayon::iter::IntoParallelIterator;
use wasm_bindgen::prelude::*;
use rayon::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

#[link(name = "bullet")]
extern "C" {
    fn __wasm_call_ctors();
}

#[wasm_bindgen(js_name = init)]
pub fn init() {
    unsafe {
        __wasm_call_ctors();
    }

    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct BwRigidbodyConstructionInfo {
    info: RigidbodyConstructionInfo,
}

#[wasm_bindgen]
impl BwRigidbodyConstructionInfo {
    pub(crate) fn new() -> Self {
        Self {
            info: RigidbodyConstructionInfo::new(),
        }
    }

    #[wasm_bindgen(js_name = setShapeType)]
    pub fn set_shape_type(&mut self, shape_type: u8) {
        let shape_type = match shape_type {
            0 => ShapeType::Box,
            1 => ShapeType::Sphere,
            2 => ShapeType::Capsule,
            5 => ShapeType::StaticPlane,
            _ => panic!("invalid shape type"),
        };
        self.info.set_shape_type(shape_type);
    }

    #[wasm_bindgen(js_name = setShapeSize)]
    pub fn set_shape_size(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.info.set_shape_size(Vec4::new(x, y, z, w));
    }

    #[wasm_bindgen(js_name = setMotionType)]
    pub fn set_motion_type(&mut self, motion_type: u8) {
        let motion_type = match motion_type {
            0 => MotionType::Dynamic,
            1 => MotionType::Kinematic,
            2 => MotionType::Static,
            _ => panic!("invalid motion type"),
        };
        self.info.set_motion_type(motion_type);
    }

    #[wasm_bindgen(js_name = setStartTransform)]
    pub fn set_start_transform(&mut self, x: f32, y: f32, z: f32, qx: f32, qy: f32, qz: f32, qw: f32) {
        self.info.set_start_transform(Vec3::new(x, y, z), Quat::from_xyzw(qx, qy, qz, qw));
    }

    #[wasm_bindgen(js_name = setMass)]
    pub fn set_mass(&mut self, mass: f32) {
        self.info.set_mass(mass);
    }

    #[wasm_bindgen(js_name = setDamping)]
    pub fn set_damping(&mut self, linear_damping: f32, angular_damping: f32) {
        self.info.set_damping(linear_damping, angular_damping);
    }

    #[wasm_bindgen(js_name = setFriction)]
    pub fn set_friction(&mut self, friction: f32) {
        self.info.set_friction(friction);
    }

    #[wasm_bindgen(js_name = setRestitution)]
    pub fn set_restitution(&mut self, restitution: f32) {
        self.info.set_restitution(restitution);
    }

    #[wasm_bindgen(js_name = setAdditionalDamping)]
    pub fn set_additional_damping(&mut self, additional_damping: bool) {
        self.info.set_additional_damping(additional_damping);
    }

    #[wasm_bindgen(js_name = setCollisionGroupMask)]
    pub fn set_collision_group_mask(&mut self, group: u16, mask: u16) {
        self.info.set_collision_group_mask(group, mask);
    }

    #[wasm_bindgen(js_name = setSleepingThreshold)]
    pub fn set_sleeping_threshold(&mut self, linear_threshold: f32, angular_threshold: f32) {
        self.info.set_sleeping_threshold(linear_threshold, angular_threshold);
    }

    #[wasm_bindgen(js_name = setDisableDeactivation)]
    pub fn set_disable_deactivation(&mut self, disable_deactivation: bool) {
        self.info.set_disable_deactivation(disable_deactivation);
    }
}

#[wasm_bindgen]
pub struct BwPhysicsWorld {
    world: PhysicsWorld,
    transform_buffer: Vec<f32>,
}

#[wasm_bindgen]
impl BwPhysicsWorld {
    pub(crate) fn new() -> Self {
        Self {
            world: PhysicsWorld::new(),
            transform_buffer: Vec::new(),
        }
    }

    #[wasm_bindgen(js_name = setGravity)]
    pub fn set_gravity(&mut self, x: f32, y: f32, z: f32) {
        self.world.set_gravity(Vec3::new(x, y, z));
    }

    #[wasm_bindgen(js_name = createPhysicsObject)]
    pub fn create_physics_object(&mut self, id: u32) {
        self.world.create_physics_object(id);
    }

    #[wasm_bindgen(js_name = destroyPhysicsObject)]
    pub fn destroy_physics_object(&mut self, id: u32) {
        self.world.destroy_physics_object(id);
    }

    #[wasm_bindgen(js_name = createRigidbody)]
    pub fn create_rigidbody(&mut self, id: u32, info: &BwRigidbodyConstructionInfo) -> i32 {
        let physics_object = self.world.get_physics_object_mut(id);
        physics_object.create_rigidbody(&info.info)
    }

    // #[wasm_bindgen(js_name = createConstraint)]
    // pub fn create_constraint(&mut self, id: u32, info: ConstraintConstructionInfo) {
    //     let physics_object = self.world.get_physics_object_mut(id);
    //     physics_object.create_constraint(&info);
    // }

    #[wasm_bindgen(js_name = stepSimulation)]
    pub fn step_simulation(&mut self, time_step: f32, max_sub_steps: i32, fixed_time_step: f32) {
        self.world.step_simulation(time_step, max_sub_steps, fixed_time_step);
    }

    #[wasm_bindgen(js_name = getTransforms)]
    pub fn get_transforms(&mut self, id: u32) -> *const f32 {
        let physics_object = self.world.get_physics_object(id);

        self.transform_buffer.clear();
        self.transform_buffer.reserve(16 * physics_object.bodies().len());

        for body in physics_object.bodies() {
            let transform = body.get_transform();
            self.transform_buffer.extend_from_slice(transform.as_ref());
        }

        self.transform_buffer.as_ptr()
    }
}

#[wasm_bindgen(js_name = createPhysicsWorld)]
pub fn create_physics_world() -> BwPhysicsWorld {
    BwPhysicsWorld::new()
}

#[wasm_bindgen(js_name = createRigidbodyConstructionInfo)]
pub fn create_rigidbody_construction_info() -> BwRigidbodyConstructionInfo {
    BwRigidbodyConstructionInfo::new()
}

fn world_test() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.8 * 10.0, 0.0));

    let physics_object = world.create_physics_object(0);
    
    let mut info = RigidbodyConstructionInfo::new();
    info.set_shape_type(ShapeType::Box);
    info.set_shape_size(Vec4::new(1.0, 1.0, 1.0, 0.0));
    info.set_motion_type(MotionType::Dynamic);
    info.set_start_transform(Vec3::new(0.0, 10.0, 0.0), Quat::IDENTITY);
    info.set_mass(1.0);
    info.set_damping(0.0, 0.0);
    info.set_friction(0.5);
    info.set_restitution(0.0);
    info.set_additional_damping(true);
    info.set_collision_group_mask(0x0001, 0xFFFF);
    info.set_sleeping_threshold(0.8, 0.8);
    info.set_disable_deactivation(true);

    let rigidbody_handle1 = physics_object.create_rigidbody(&info);
    let rigidbody = &mut physics_object.bodies_mut()[rigidbody_handle1 as usize];
    let transform = rigidbody.get_transform();
    rigidbody.set_transform(&transform);

    let rigidbody_handle2 = physics_object.create_rigidbody(&info);
    
    let mut info = ConstraintConstructionInfo::new();
    info.set_type(ConstraintType::Generic6DofSpring);
    info.set_bodies(rigidbody_handle1, rigidbody_handle2);
    info.set_frames(&Mat4::IDENTITY, &Mat4::IDENTITY);
    info.set_use_linear_reference_frame_a(true);
    info.set_disable_collisions_between_linked_bodies(false);
    info.set_linear_limits(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    info.set_angular_limits(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    info.set_stiffness(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

    physics_object.create_constraint(&info).unwrap();

    for _ in 0..1000 {
        world.step_simulation(1.0 / 60.0, 120, 1.0 / 120.0);
    }
}

#[wasm_bindgen(js_name = test)]
pub fn test() {
    world_test();
    web_sys::console::log_1(&"start physics engine threading test".into());
    (0..2).into_par_iter().for_each(|_| {
        world_test();
    });
    web_sys::console::log_1(&"end physics engine threading test".into());
}
