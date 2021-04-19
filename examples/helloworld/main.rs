use flecs_sys::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem::*;

#[repr(C)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    unsafe {
        let world = ecs_init();
        let pos_name = CString::new("Position").unwrap();
        let pos_id = ecs_new_component(
            world,
            0,
            pos_name.as_ptr(),
            size_of::<Position>(),
            align_of::<Position>(),
        );

        let ename = CString::new("MyEntity").unwrap();
        let entity = ecs_new_entity(world, 0, ename.as_ptr(), pos_name.as_ptr());

        let mut p = Position { x: 10.0, y: 20.0 };
        let p_ptr: *mut std::ffi::c_void = &mut p as *mut _ as *mut std::ffi::c_void;
        ecs_set_ptr_w_id(world, entity, pos_id, size_of::<Position>(), p_ptr);

        let pos = &*(ecs_get_w_id(world, entity, pos_id) as *const Position);
        let e_name = CStr::from_ptr(ecs_get_name(world, entity));

        println!(
            "Position of {} is {{{} {}}}",
            e_name.to_string_lossy().into_owned(),
            pos.x.to_string(),
            pos.y.to_string()
        );

        ecs_fini(world);
    }
}
