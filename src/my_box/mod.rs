use core::ops::Deref;
use core::ops::DerefMut;

use core::alloc::Layout;
use core::ptr::NonNull;
use alloc::alloc::alloc;


#[repr(transparent)]
pub struct MyBox<T> {
    ptr: NonNull<T>,
    // ptr: *mut T that is never null
}


fn get_layout<T>() -> Layout {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();
    unsafe { Layout::from_size_align_unchecked(size, align) }
}



// move is:
// - "destructive"  - the old location is unavailable (destroyed) - this is NOTHING TO DO WITH `Drop`. This is 100% a compiler-error-message thing. This does not do anything at runtime, juts gives you errors at compile time
// - "bitwise"      - the memory in the original place is copied bit-for-bit exactly to the new place
// - "copy"         

// #[test]
// fn use_dog() {
//     // pretend `String` is copy
//     let s = String::from("hello\n");
//     let t = s;

//     drop(t);
//     drop(s);
// }

// assert_eq!(t, t.clone());

impl<T> MyBox<T> {
    pub fn as_non_null(&self) -> NonNull<T> {
        unsafe { core::mem::transmute(self) }
    }
    
    pub fn new(value: T) -> Self {
        // *mut T = &mut T (except, YOU are responsible for safety, not the compiler)
        // *mut T is a "raw pointer"
        // 1. allocate space on the heap for the i32 (and remember the pointer)
        // 2. move the i32 to the heap
        // 3. use the pointer we got in step 1 to create a `Self` and return it
        
        let ptr = unsafe { alloc(get_layout::<T>()) } as *mut T;
        let ptr = NonNull::new(ptr).unwrap();

        unsafe { core::ptr::write(ptr.as_ptr(), value) };

        Self { ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.ptr.as_ptr());
            alloc::alloc::dealloc(self.ptr.as_ptr() as *mut u8, get_layout::<T>());
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &<Self as Deref>::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

#[test]
fn simple_test() {
    let mut my_box = MyBox::new(vec![1, 2, 3]);
    let get = &*my_box;
    assert_eq!(get.clone(), vec![1, 2, 3]);

    *my_box = vec![1, 2, 3, 4];
    assert_eq!(my_box.len(), 4);
}

#[test]
fn size_correct() {
    use core::mem::size_of;
    
    assert_eq!(size_of::<MyBox<i32>>(), size_of::<usize>());
    assert_eq!(size_of::<Option<MyBox<i32>>>(), size_of::<usize>());
}

// let b = MyBox::new(123);
// 
// A implements Deref<Target = B>
// - "A is a wrapper around a B"
// - "A contains a B"
// 
// Deref enables two language features:
// - a.b() will work if `b` is a method on the deref target
// - if you have a function that takes &B, you can pass a &A
// - fn foo(x: &[u8]) {}   -> let v = vec![1, 2, 3]; foo(&v)
// 
// [T] - a slice - 
// - &[T]  &str
// - Box<[T]>  Box<str>
// - Arc<[T]>  Arc<str>
// 
// Box<T> implements Deref<Target = T>
// Vec<T> implements Deref<Target = [T]>
// - so, &Vec<T> can be treated like a &[T]
// 


// #[test]
// fn foo() {
//     fn print_length(s: &str) {
//         println!("{}", s.len());
//     }
    
//     let s = String::from("hello");
//     let b = Box::new(s);
//     b.to_lowercase();
//     print_length(&b);
//     let v = vec![2, 3, 4];
//     v.iter();
//     // a.b();
//     // 1. look at type of `a`, does it have a method called `b`?
//     // 2. if not, look at all the traits that are in-scope (i.e. there is a `use whatever::Trait` in this file). Do any of them BOTH:
//     //   - are implemented by `a`
//     //   - have a method called `b`
//     // 3. if not, does `a` implement `Deref`? If it does, find the target type, and go back to step 1
    
// }
