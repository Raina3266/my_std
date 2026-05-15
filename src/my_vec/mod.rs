use alloc::alloc::alloc;
use alloc::alloc::dealloc;
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

fn layout_for_capacity<T>(cap: usize) -> Layout {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();

    unsafe { Layout::from_size_align_unchecked(size * cap, align) }
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        let ptr = unsafe { alloc::alloc::alloc(layout_for_capacity::<T>(cap)) as *mut T};
        let non_null = NonNull::new(ptr).unwrap();
        Self { ptr: non_null, len: 0, cap }
    }

    pub fn push(&mut self, value: T) {
        if !self.can_insert_without_reallocating() {
            if self.cap == 0 {
                self.reallocate(8);
            } else {
                self.reallocate(self.cap * 2);
            }
        }

        let end_ptr = unsafe { self.ptr.add(self.len()) };
        unsafe { core::ptr::write(end_ptr.as_ptr(), value) };
        self.len +=1
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
            let result = unsafe { self.ptr.as_ptr().add(n) };
            Some(result)
        }
    }

    fn can_insert_without_reallocating(&self) -> bool {
        self.len < self.cap
    }

    fn reallocate(&mut self, new_cap: usize) {
        // 1. new heap allocation with space for the new value
        // 2. copy the old values to the new allocation
        // 4. clean up the old allocation
        // 5. update self so the fields are correct again

        // 1.
        let new_ptr = unsafe { alloc(layout_for_capacity::<T>(new_cap)) } as *mut T;

        // 2.
        unsafe { core::ptr::copy(self.ptr.as_ptr(), new_ptr, self.len()) };

        // 4.
        if self.cap != 0 {
            unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout_for_capacity::<T>(self.len())) };
        }

        // 5.
        self.ptr = NonNull::new(new_ptr).unwrap();
        self.cap = new_cap;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len() {
                core::ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }
            dealloc(self.ptr.as_ptr() as *mut u8, layout_for_capacity::<T>(self.cap))
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

#[test]
fn with_capacity_test() {
    let v = MyVec::<String>::with_capacity(10);
    assert!(v.can_insert_without_reallocating());
}
