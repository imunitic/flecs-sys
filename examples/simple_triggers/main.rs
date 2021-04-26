use flecs_sys::*;
use std::ffi::*;
use std::mem::*;

#[repr(C)]
struct Position {
    x: f32,
    y: f32,
}

unsafe extern "C" fn add_position(it: *mut ecs_iter_t) {
    let count = (*it).count as usize;

    let posc = ecs_column_w_size(it, size_of::<Position>(), 1);
    let p = std::ptr::slice_from_raw_parts_mut(posc as *mut Position, count)
        .as_mut()
        .unwrap();

    for i in 0..count {
        let position = p.get_mut(i).unwrap();
        position.x = 10.0;
        position.y = 20.0;

        println!("Position added");
    }
}

unsafe extern "C" fn remove_position(it: *mut ecs_iter_t) {
    let count = (*it).count as usize;

    let posc = ecs_column_w_size(it, size_of::<Position>(), 1);
    let p = std::ptr::slice_from_raw_parts_mut(posc as *mut Position, count)
        .as_mut()
        .unwrap();

    for i in 0..count {
        let position = p.get_mut(i).unwrap();
        println!("Position removed -> {{ {} {} }}", position.x, position.y);
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

        let add_trigger = CString::new("AddPosition").unwrap();
        ecs_new_trigger(
            world,
            0,
            add_trigger.as_ptr(),
            EcsOnAdd as u64,
            pos_name.as_ptr(),
            Some(add_position),
        );

        let remove_trigger = CString::new("RemovePosition").unwrap();
        ecs_new_trigger(
            world,
            0,
            remove_trigger.as_ptr(),
            EcsOnRemove as u64,
            pos_name.as_ptr(),
            Some(remove_position),
        );

        let ename = CString::new("MyEntity").unwrap();
        let entity = ecs_new_entity(world, 0, ename.as_ptr(), pos_name.as_ptr());

        ecs_remove_entity(world, entity, pos_id);
        ecs_add_entity(world, entity, pos_id);
        ecs_add_entity(world, entity, pos_id);

        ecs_fini(world);
    }
}
