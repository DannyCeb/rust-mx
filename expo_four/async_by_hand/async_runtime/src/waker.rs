use std::task::{RawWaker, RawWakerVTable};

// La vtable se usa como wrapper para todos los metodos del waker, que será pasado como referencia usando un unsafe pointer
static VTABLE: RawWakerVTable = RawWakerVTable::new(my_clone, my_wake, my_wake_by_ref, my_drop);

unsafe fn my_clone(raw_waker: *const ()) -> RawWaker {
    RawWaker::new(raw_waker, &VTABLE)
}

unsafe fn my_wake(raw_waker: *const ()) {
    unsafe {
        drop(Box::from_raw(raw_waker as *mut u32));
    }
}

unsafe fn my_drop(raw_waker: *const ()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}

unsafe fn my_wake_by_ref(_raw_waker: *const ()) {}

pub fn create_raw_waker() -> RawWaker {
    let data = Box::into_raw(Box::new(42));

    RawWaker::new(data as *const (), &VTABLE)
}
