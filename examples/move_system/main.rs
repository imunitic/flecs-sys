use flecs_sys::*;
use std::ffi::{CStr, CString};
use std::mem::*;

#[repr(C)]
struct Position {
    x: f32,
    y: f32,
}

#[repr(C)]
struct Velocity {
    x: f32,
    y: f32,
}

unsafe extern "C" fn move_sys(it: *mut ecs_iter_t) {
    let count = (*it).count as usize;

    let posc = ecs_column_w_size(it, size_of::<Position>(), 1);
    let velc = ecs_column_w_size(it, size_of::<Velocity>(), 2);

    let p = std::ptr::slice_from_raw_parts_mut(posc as *mut Position, count)
        .as_mut()
        .unwrap();

    let v = std::ptr::slice_from_raw_parts_mut(velc as *mut Velocity, count)
        .as_mut()
        .unwrap();

    for i in 0..count {
        let position = p.get_mut(i).unwrap();
        let velocity = v.get_mut(i).unwrap();

        position.x += velocity.x * (*it).delta_time;
        position.y += velocity.y * (*it).delta_time;

        let ents = std::ptr::slice_from_raw_parts((*it).entities, count);
        let e_name = CStr::from_ptr(ecs_get_name((*it).world, (&*ents)[i]));

        println!(
            "{} moved to {{.x = {}, .y = {}}}",
            e_name.to_string_lossy().into_owned(),
            position.x,
            position.y
        );
    }
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

        let vel_name = CString::new("Velocity").unwrap();
        let vel_id = ecs_new_component(
            world,
            0,
            vel_name.as_ptr(),
            size_of::<Velocity>(),
            align_of::<Velocity>(),
        );

        let comps = CString::new("Position, Velocity").unwrap();
        let sys_name = CString::new("Move").unwrap();

        ecs_new_system(
            world,
            0,
            sys_name.as_ptr(),
            EcsOnUpdate as u64,
            comps.as_ptr(),
            Some(move_sys),
        );

        let ename = CString::new("MyEntity").unwrap();
        let entity = ecs_new_entity(world, 0, ename.as_ptr(), comps.as_ptr());

        let mut p = Position { x: 0.0, y: 0.0 };
        let p_ptr: *mut std::ffi::c_void = &mut p as *mut _ as *mut std::ffi::c_void;
        ecs_set_ptr_w_id(world, entity, pos_id, size_of::<Position>(), p_ptr);

        let mut v = Velocity { x: 1.0, y: 1.0 };
        let v_ptr: *mut std::ffi::c_void = &mut v as *mut _ as *mut std::ffi::c_void;
        ecs_set_ptr_w_id(world, entity, vel_id, size_of::<Velocity>(), v_ptr);

        println!("Application move_system is running, press CTRL+C to exit...");

        while ecs_progress(world, 0.0) {}

        ecs_fini(world);
    }
}
