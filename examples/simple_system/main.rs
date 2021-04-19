use flecs_sys::*;
use std::ffi::CString;
use std::mem::*;

#[repr(C)]
struct Message<'a> {
    text: &'a str,
}

unsafe extern "C" fn print_message(it: *mut ecs_iter_t) {
    let count = (*it).count as usize;
    let msgc = ecs_column_w_size(it, size_of::<Message>(), 1);
    let m = std::ptr::slice_from_raw_parts(msgc as *const Message, count);
    for i in 0..count {
        let msg = (*m).get(i).unwrap();
        println!("{}", msg.text);
    }
}

fn main() {
    unsafe {
        let world = ecs_init();
        let msg_name = CString::new("Message").unwrap();
        let msg_id = ecs_new_component(
            world,
            0,
            msg_name.as_ptr(),
            size_of::<Message>(),
            align_of::<Message>(),
        );

        let sys_name = CString::new("PrintMessage").unwrap();
        ecs_new_system(
            world,
            0,
            sys_name.as_ptr(),
            EcsOnUpdate as u64,
            msg_name.as_ptr(),
            Some(print_message),
        );

        let ename = CString::new("MyEntity").unwrap();
        let entity = ecs_new_entity(world, 0, ename.as_ptr(), msg_name.as_ptr());
        let mut msg = Message {
            text: "Hello, Flecs!",
        };
        let msg_ptr: *mut std::ffi::c_void = &mut msg as *mut _ as *mut std::ffi::c_void;
        ecs_set_ptr_w_id(world, entity, msg_id, size_of::<Message>(), msg_ptr);

        println!("Application simple_system is running, press CTRL+C to exit...");

        while ecs_progress(world, 0.0) {}

        ecs_fini(world);
    }
}
