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
    let rigidbody = physics_object.get_body_mut(rigidbody_handle1);
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
