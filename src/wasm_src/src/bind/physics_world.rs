use glam::Vec3;

#[link(name = "bullet")]
extern "C" {
    fn bt_create_world() -> *mut std::ffi::c_void;

    fn bt_destroy_world(world: *mut std::ffi::c_void);

    fn bt_world_set_gravity(world: *mut std::ffi::c_void, x: f32, y: f32, z: f32);

    fn bt_world_step_simulation(world: *mut std::ffi::c_void, time_step: f32, max_sub_steps: i32, fixed_time_step: f32);

    fn bt_world_add_rigidbody(world: *mut std::ffi::c_void, body: *mut std::ffi::c_void);

    fn bt_world_remove_rigidbody(world: *mut std::ffi::c_void, body: *mut std::ffi::c_void);

    fn bt_world_add_constraint(world: *mut std::ffi::c_void, constraint: *mut std::ffi::c_void);

    fn bt_world_remove_constraint(world: *mut std::ffi::c_void, constraint: *mut std::ffi::c_void);
}

pub(crate) struct PhysicsWorld {
    world: *mut std::ffi::c_void,
}

impl PhysicsWorld {
    pub(crate) fn new() -> Self {
        let world = unsafe { bt_create_world() };
        Self { world }
    }

    pub(crate) fn set_gravity(&self, gravity: Vec3) {
        unsafe { bt_world_set_gravity(self.world, gravity.x, gravity.y, gravity.z) };
    }

    pub(crate) fn step_simulation(&self, time_step: f32, max_sub_steps: i32, fixed_time_step: f32) {
        unsafe { bt_world_step_simulation(self.world, time_step, max_sub_steps, fixed_time_step) };
    }

    pub(crate) fn add_rigidbody(&self, body: *mut std::ffi::c_void) {
        unsafe { bt_world_add_rigidbody(self.world, body) };
    }

    pub(crate) fn remove_rigidbody(&self, body: *mut std::ffi::c_void) {
        unsafe { bt_world_remove_rigidbody(self.world, body) };
    }

    pub(crate) fn add_constraint(&self, constraint: *mut std::ffi::c_void) {
        unsafe { bt_world_add_constraint(self.world, constraint) };
    }

    pub(crate) fn remove_constraint(&self, constraint: *mut std::ffi::c_void) {
        unsafe { bt_world_remove_constraint(self.world, constraint) };
    }
}

impl Drop for PhysicsWorld {
    fn drop(&mut self) {
        unsafe { bt_destroy_world(self.world) };
    }
}
