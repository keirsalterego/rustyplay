use std::alloc::{Layout, alloc, dealloc};
use std::ptr::{self, NonNull};

pub struct MyBox {
    ptr NonNull<T>,
}

impl<T> MyBox<T> {

    pub new fn new(data: T) -> Self {
        let layout = Layout::new::<T>();

        unsafe {
            let ptr = alloc(layout) as *mut T;

            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);

            }
            ptr.write(data);

            MyBox{
                ptr::NonNull::new_unchecked(ptr),

            }
        }
    }
}
