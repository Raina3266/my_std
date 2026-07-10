use core::{alloc::Layout, ptr::NonNull};

struct MyVecRaina<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

fn get_layout_cap<T>(cap: usize) -> Layout {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();
    unsafe { Layout::from_size_align_unchecked(size * cap, align) }
}

impl<T> MyVecRaina<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        let ptr = unsafe { alloc::alloc::alloc(get_layout_cap::<T>(cap)) } as *mut T;
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            len: 0,
            cap,
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        let pointer = self.addrress_of_nth(index)?;
        Some(unsafe { &*pointer })
    }
    fn addrress_of_nth(&self, index: usize) -> Option<*mut T> {
        if index < self.len {
            let pointer = unsafe { self.ptr.as_ptr().add(index) };
            Some(pointer)
        } else {
            None
        }
    }
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            if self.cap == 0 {
                self.reallocate(8);
            } else {
                self.reallocate(self.cap * 2);
            }
        }

        let end_pointer = unsafe { self.ptr.as_ptr().add(self.len) };
        unsafe { core::ptr::write(end_pointer, value) };
        self.len += 1;
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if self.len == self.cap {
            if self.cap == 0 {
                self.reallocate(8);
            } else {
                self.reallocate(self.cap * 2);
            }
        }

        let pointer_to_insert = unsafe { self.ptr.as_ptr().add(index) };
        // 1. ptr = good, cap = 8, len = 0, index = 0
        // 2. ptr = good, cap = 8, len = 1, index = 0
        

        let shift = |index_to_shift: usize| {
            unsafe {
                core::ptr::copy(
                    self.ptr.add(index_to_shift).as_ptr(),
                    self.ptr.add(index_to_shift + 1).as_ptr(),
                    1,
                )
            };
        };

        // for n in (index..self.len).rev() {
        //     // 1. n = []
        //     // 2. n = [0]
        //     shift(n)
        // }
        
        for n in (1..=self.len - index).rev() {
            // 1. n = []
            // 2. n = [1]
            let src_pointer = unsafe { pointer_to_insert.add(n - 1) };
            let dst_pointer = unsafe { pointer_to_insert.add(n) };
            unsafe { core::ptr::copy(src_pointer, dst_pointer, 1) };
        }

        unsafe { core::ptr::write(pointer_to_insert as *mut T, value) };
        self.len += 1;
    }

    fn reallocate(&mut self, new_cap: usize) {
        // 1. alloc more space
        let new_ptr = unsafe { alloc::alloc::alloc(get_layout_cap::<T>(new_cap)) } as *mut T;
        // 2. move old value to new allocation
        unsafe { core::ptr::copy(self.ptr.as_ptr(), new_ptr, self.len) };
        // 3. clean up the old value
        if self.cap != 0 {
            unsafe {
                alloc::alloc::dealloc(self.ptr.as_ptr() as *mut u8, get_layout_cap::<T>(self.len))
            };
        }
        // 4. update value
        self.ptr = NonNull::new(new_ptr).unwrap();
        self.cap = new_cap
    }

    fn check_invariants(&self) {
        assert!(self.len <= self.cap);
        for i in 0..self.len {
            assert!(self.get(i).is_some());
        }
    }
}

impl<T> Drop for MyVecRaina<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                #[cfg(test)]
                {
                    dbg!(self.len);
                    dbg!(i);
                }
                self.get(i).unwrap();
                self.check_invariants();
                core::ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }
            alloc::alloc::dealloc(self.ptr.as_ptr() as *mut u8, get_layout_cap::<T>(self.cap));
        }
    }
}

#[test]
fn very_simple_test() {
    let mut v = MyVecRaina::new();
    // v.check_invariants();
    // v.push(String::from("a"));
    v.check_invariants();
    v.insert(0, String::from("bb"));
    v.check_invariants();
    v.insert(0, String::from("ccc"));
    v.check_invariants();
}

#[test]
fn simple_test() {
    let mut v = MyVecRaina::new();

    v.push(String::from("hello"));
    assert_eq!(*v.get(0).unwrap(), "hello");

    v.push(String::from("world"));
    assert_eq!(*v.get(0).unwrap(), "hello");
    assert_eq!(*v.get(1).unwrap(), "world");

    v.push(String::from("foo"));
    assert_eq!(*v.get(0).unwrap(), "hello");
    assert_eq!(*v.get(1).unwrap(), "world");
    assert_eq!(*v.get(2).unwrap(), "foo");

    assert_eq!(v.len, 3);

    v.insert(1, String::from("new"));
    assert_eq!(*v.get(0).unwrap(), "hello");
    assert_eq!(*v.get(1).unwrap(), "new");
    assert_eq!(*v.get(2).unwrap(), "world");

    assert_eq!(v.len, 4);

    v.insert(3, String::from("boo"));
    assert_eq!(*v.get(4).unwrap(), "boo");
    assert_eq!(*v.get(5).unwrap(), "foo");

    assert_eq!(v.len, 5);
}
