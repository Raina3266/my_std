use core::alloc::Layout;

struct MyVecI32Raina {
    pointer: *mut i32,
    len: usize,
}

fn get_layout(n: usize) -> Layout {
    unsafe { Layout::from_size_align_unchecked(4 * n, 4) }
}

impl MyVecI32Raina {
    fn new() -> Self {
        Self {
            pointer: core::ptr::null_mut(),
            len: 0,
        }
    }

    fn get(&self, index: usize) -> Option<&i32> {
        let pointer = self.address_of_nth_thing(index)?;
        Some(unsafe { &*pointer })
    }

    fn push(&mut self, value: i32) {
        let required_space = self.len + 1;
        let layout = get_layout(required_space);
    }

    fn address_of_nth_thing(&self, n: usize) -> Option<*mut i32> {
        if n > self.len {
            None
        } else {
            let result = unsafe { self.pointer.add(n) };
            Some(result)
        }
    }
}
