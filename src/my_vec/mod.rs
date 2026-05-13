use alloc::alloc::alloc;
use alloc::alloc::dealloc;
use core::alloc::Layout;

const X: core::ptr::NonNull<i32> = core::ptr::NonNull::dangling();

pub struct MyVecI32 {
    ptr: *mut i32,
    len: usize,
}

fn layout_for_n_i32s(n: usize) -> Layout {
    unsafe { Layout::from_size_align_unchecked(4 * n, 4) }
}

impl MyVecI32 {
    pub fn new() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push(&mut self, value: i32) {
        // 1. new heap allocation with space for the new value
        // 2. copy the old values to the new allocation
        // 3. copy the new value to the new allocation
        // 4. clean up the old allocation
        // 5. update self so the fields are correct again

        // 1.
        let space_required = self.len() + 1;
        let new_ptr = unsafe { alloc(layout_for_n_i32s(space_required)) } as *mut i32;

        // 2.
        unsafe { core::ptr::copy(self.ptr, new_ptr, self.len()) };

        // 3.
        let end_ptr = unsafe { new_ptr.add(self.len()) };
        unsafe { core::ptr::write(end_ptr, value) };

        // 4.
        // todo think about freeing the Ts individually
        if self.len() != 0 {
            unsafe { dealloc(self.ptr as *mut u8, layout_for_n_i32s(self.len())) };
        }

        // 5.
        self.ptr = new_ptr;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        let ptr = self.addr_of_nth(index)?;
        Some(unsafe { &*ptr })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn addr_of_nth(&self, n: usize) -> Option<*mut i32> {
        if n >= self.len() {
            None
        } else {
            let result = unsafe { self.ptr.add(n) };
            Some(result)
        }
    }
}

impl Drop for MyVecI32 {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr as *mut u8, layout_for_n_i32s(self.len())) };
    }
}

#[test]
fn simple_test() {
    let mut v = MyVecI32::new();

    v.push(1);
    assert_eq!(*v.get(0).unwrap(), 1);

    v.push(10);
    assert_eq!(*v.get(0).unwrap(), 1);
    assert_eq!(*v.get(1).unwrap(), 10);

    v.push(100);
    assert_eq!(*v.get(0).unwrap(), 1);
    assert_eq!(*v.get(1).unwrap(), 10);
    assert_eq!(*v.get(2).unwrap(), 100);

    assert_eq!(v.len(), 3);
}
