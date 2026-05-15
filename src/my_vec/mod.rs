use alloc::alloc::alloc;
use alloc::alloc::dealloc;
use core::alloc::Layout;

pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
}

fn layout_for_n_ts<T>(n: usize) -> Layout {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();

    unsafe { Layout::from_size_align_unchecked(size * n, align) }
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        // 1. new heap allocation with space for the new value
        // 2. copy the old values to the new allocation
        // 3. copy the new value to the new allocation
        // 4. clean up the old allocation
        // 5. update self so the fields are correct again

        // 1.
        let space_required = self.len() + 1;
        let new_ptr = unsafe { alloc(layout_for_n_ts::<T>(space_required)) } as *mut T;

        // 2.
        unsafe { core::ptr::copy(self.ptr, new_ptr, self.len()) };

        // 3.
        let end_ptr = unsafe { new_ptr.add(self.len()) };
        unsafe { core::ptr::write(end_ptr, value) };

        // 4.
        if self.len() != 0 {
            unsafe { dealloc(self.ptr as *mut u8, layout_for_n_ts::<T>(self.len())) };
        }

        // 5.
        self.ptr = new_ptr;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let ptr = self.addr_of_nth(index)?;
        Some(unsafe { &*ptr })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn addr_of_nth(&self, n: usize) -> Option<*mut T> {
        if n >= self.len() {
            None
        } else {
            let result = unsafe { self.ptr.add(n) };
            Some(result)
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len() {
                core::ptr::drop_in_place(self.ptr.add(i));
            }
            dealloc(self.ptr as *mut u8, layout_for_n_ts::<T>(self.len()))
        };
    }
}

#[test]
fn simple_test() {
    let mut v = MyVec::new();

    v.push(String::from("hello"));
    assert_eq!(*v.get(0).unwrap(), "hello");

    v.push(String::from("world"));
    assert_eq!(*v.get(0).unwrap(), "hello");
    assert_eq!(*v.get(1).unwrap(), "world");

    v.push(String::from("foo"));
    assert_eq!(*v.get(0).unwrap(), "hello");
    assert_eq!(*v.get(1).unwrap(), "world");
    assert_eq!(*v.get(2).unwrap(), "foo");

    assert_eq!(v.len(), 3);
}
