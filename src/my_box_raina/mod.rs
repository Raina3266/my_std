use core::alloc::Layout;
use core::ops::{Deref, DerefMut};

struct MyBoxRaina<T> {
    pointer: *mut T,
}

// Box::new(123), Box::new(1_000_000_000)
// Box::new(123), Box::new(String::from("hello"))

fn get_layout<T>() -> Layout {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(size, align) }
}

impl<T> MyBoxRaina<T> {
    fn new(input: T) -> Self {
        let layout = get_layout::<T>();
        let pointer = unsafe { alloc::alloc::alloc(layout) };

        unsafe { core::ptr::write(pointer as *mut T, input) };
        Self {
            pointer: pointer as *mut T,
        }
    }
}

impl<T> Drop for MyBoxRaina<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.pointer);
            alloc::alloc::dealloc(self.pointer as *mut u8, get_layout::<T>());
        }
    }
}

impl<T> Deref for MyBoxRaina<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.pointer }
    }
}

impl<T> DerefMut for MyBoxRaina<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.pointer }
    }
}

// A: Deref<Target = B>
// - Enables `*a` syntax
// - Allows calling functions defined for `B` on a value of type `A`, if using `a.foo()` syntax
// - Allows automatically converting (coercing) &A into &B  (e.g. &String -> &str)

#[test]
fn simple_test() {
    let mut mb = MyBoxRaina::new(String::from("hello"));
    let _s: &str = &mb;
    mb.push_str(" world");
    assert_eq!(mb.len(), 11);
}
