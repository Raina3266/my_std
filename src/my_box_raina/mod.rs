use core::alloc::Layout;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

#[repr(transparent)]
struct MyBoxRaina<T> {
    pointer: NonNull<T>,
}

fn get_layout<T>() -> Layout {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();
    unsafe { Layout::from_size_align_unchecked(size, align) }
}

impl<T> MyBoxRaina<T> {
    fn new(input: T) -> Self {
        let pointer = unsafe { alloc::alloc::alloc(get_layout::<T>()) } as *mut T;
        let pointer = NonNull::new(pointer).unwrap();
        unsafe {
            core::ptr::write(pointer.as_ptr(), input);
        }
        Self { pointer }
    }   

    fn into_inner(self) -> T {
        let t = unsafe { core::ptr::read(self.pointer.as_ptr()) };
        unsafe { alloc::alloc::dealloc(self.pointer.as_ptr() as *mut u8, get_layout::<T>()) };
        core::mem::forget(self);
        t
    }
}

impl<T> Drop for MyBoxRaina<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.pointer.as_ptr());
            alloc::alloc::dealloc(self.pointer.as_ptr() as *mut u8, get_layout::<T>())
        };
    }
}

impl<T> DerefMut for MyBoxRaina<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.pointer.as_ptr() }
    }
}

impl<T> Deref for MyBoxRaina<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.pointer.as_ptr() }
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

#[test]
fn into_inner_test() {
    let inner = String::from("hello");
    let mb = MyBoxRaina::new(inner);
    let inner_again = mb.into_inner();
    
    assert_eq!(inner_again, "hello");
    // <- HERE
    
}

// #[test]
// fn print_sizes() {
//     println!("{}", core::mem::size_of::<i32>());  // 4
//     println!("{}", core::mem::size_of::<i64>());  // 8
//     println!("{}", core::mem::size_of::<[i64; 10]>());  // 80
//     println!("{}", core::mem::size_of::<Option<i32>>());  // 8
//     println!("{}", core::mem::size_of::<Option<i64>>());  // 16
//     println!("{}", core::mem::size_of::<MyBoxRaina<i32>>());  // 8
//     println!("{}", core::mem::size_of::<Option<MyBoxRaina<i32>>>());  // 16
//     println!("{}", core::mem::size_of::<Box<i32>>());  // 8
//     println!("{}", core::mem::size_of::<Option<Box<i32>>>());  // 8

//     panic!();
// }
