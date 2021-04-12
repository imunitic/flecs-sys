use flecs_sys::*;
use std::ffi::CString;
use std::mem::*;

#[repr(C)]
struct Position {
    x: u32,
    y: u32,
}

unsafe extern "C" fn move_sys(it: *mut ecs_iter_t) {
    let count = (*it).count as usize;
    let posc = ecs_column_w_size(it, size_of::<Position>(), 1);
    let p = std::ptr::slice_from_raw_parts_mut(posc as *mut Position, count)
        .as_mut()
        .unwrap();
    for i in 0..count {
        let position = p.get_mut(i).unwrap();
        position.x += 10;
        position.y += 10;

        println!("Moved to {} {}", position.x, position.y);
    }
}

fn main() {
    unsafe {
        let world = ecs_init();
        let pos_id = ecs_new_id(world);
        let pos_size = size_of::<Position>();
        let pos_align = align_of::<Position>();
        let pos_name = CString::new("Position").unwrap();
        let pos_com = ecs_new_component(world, pos_id, pos_name.as_ptr(), pos_size, pos_align);

        let sys_name = CString::new("Move").unwrap();
        let sys = ecs_new_system(
            world,
            ecs_new_id(world),
            sys_name.as_ptr(),
            EcsOnUpdate as u64,
            pos_name.as_ptr(),
            Some(move_sys),
        );

        let ename = CString::new("MyEntity").unwrap();
        let entity = ecs_new_entity(world, ecs_new_id(world), ename.as_ptr(), pos_name.as_ptr());
        let mut p = Position{x: 0, y: 0};
        let p_ptr: *mut std::ffi::c_void = &mut p as *mut _ as *mut std::ffi::c_void;
        ecs_set_ptr_w_id(world, entity, pos_com, size_of::<Position>(), p_ptr);

        while ecs_progress(world, 0.0) {}
    }
}
