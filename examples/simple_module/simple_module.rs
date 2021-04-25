use flecs_sys::*;
use std::ffi::{CStr, CString};
use std::mem::*;

#[repr(C)]
pub(crate) struct SimpleModule {
    pub(crate) position_id: ecs_id_t,
    pub(crate) velocity_id: ecs_id_t,
    pub(crate) move_id: ecs_entity_t,
}

#[repr(C)]
pub(crate) struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[repr(C)]
pub(crate) struct Velocity {
    pub(crate) x: f32,
    pub(crate) y: f32,
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

pub(crate) unsafe extern "C" fn import(world: *mut ecs_world_t) {
    let module_name = CString::new("SimpleModule").unwrap();
    let simple_module = ecs_new_module(
        world,
        0,
        module_name.as_ptr(),
        size_of::<SimpleModule>(),
        align_of::<SimpleModule>(),
    );

    let mut is_added = &false;
    let is_added_ptr: *mut bool = &mut is_added as *mut _ as *mut bool;
    let module_ptr = ecs_get_mut_w_id(world, simple_module, simple_module, is_added_ptr);
    let module: &mut SimpleModule = &mut *(module_ptr as *mut SimpleModule);

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

    let move_system = ecs_new_system(
        world,
        0,
        sys_name.as_ptr(),
        EcsOnUpdate as u64,
        comps.as_ptr(),
        Some(move_sys),
    );

    module.move_id = move_system;
    module.position_id = pos_id;
    module.velocity_id = vel_id;
}
