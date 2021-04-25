mod simple_module;
use std::{ffi::CString, mem::size_of};

use flecs_sys::*;

fn main() {
    unsafe {
        let world = ecs_init();

        let sm_ptr: *mut std::ffi::c_void = &mut simple_module::SimpleModule {
            position_id: 0,
            velocity_id: 0,
            move_id: 0,
        } as *mut _ as *mut std::ffi::c_void;

        let module_cstr = CString::new("SimpleModule").unwrap();
        let module_name = ecs_module_path_from_c(module_cstr.as_ptr());

        ecs_import(
            world,
            Some(simple_module::import),
            module_name,
            sm_ptr,
            size_of::<simple_module::SimpleModule>(),
        );

        let smodule: &mut simple_module::SimpleModule =
            &mut *(sm_ptr as *mut simple_module::SimpleModule);

        let comps = CString::new("simple.module.Position, simple.module.Velocity").unwrap();
        let ename = CString::new("MyEntity").unwrap();
        let entity = ecs_new_entity(world, 0, ename.as_ptr(), comps.as_ptr());

        let mut p = simple_module::Position { x: 0.0, y: 0.0 };
        let p_ptr: *mut std::ffi::c_void = &mut p as *mut _ as *mut std::ffi::c_void;
        ecs_set_ptr_w_id(
            world,
            entity,
            smodule.position_id,
            size_of::<simple_module::Position>(),
            p_ptr,
        );

        let mut v = simple_module::Velocity { x: 1.0, y: 1.0 };
        let v_ptr: *mut std::ffi::c_void = &mut v as *mut _ as *mut std::ffi::c_void;
        ecs_set_ptr_w_id(
            world,
            entity,
            smodule.velocity_id,
            size_of::<simple_module::Velocity>(),
            v_ptr,
        );

        println!("Application move_system is running, press CTRL+C to exit...");

        while ecs_progress(world, 0.0) {}

        ecs_fini(world);
    }
}
