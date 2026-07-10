use core::cell::Cell;
use core::ptr::NonNull;
use std::cell::RefCell;
use core::ops::Deref;
use std::rc::Weak;

use alloc::alloc::Layout;
use alloc::alloc::alloc;

fn main() {}

// A reference-counted pointer type
//
// you can think of it as:
// - lifetimeless reference
// - lifetime extender
pub struct MyRc<T> {
    ptr: NonNull<MyRcInner<T>>,
}

struct MyRcInner<T> {
    value: T,
    count: Cell<usize>,
}

fn get_layout<T>() -> Layout {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();
    unsafe { Layout::from_size_align_unchecked(size, align) }
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let value = MyRcInner { value, count: Cell::new(1) };

        let ptr = unsafe { alloc(get_layout::<MyRcInner<T>>()) } as *mut MyRcInner<T>;
        let ptr = NonNull::new(ptr).unwrap();

        unsafe { core::ptr::write(ptr.as_ptr(), value) };

        Self { ptr }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        let inner: &MyRcInner<T> = unsafe { &*self.ptr.as_ptr() };
        inner.count.set(inner.count.get() + 1);
        MyRc { ptr: self.ptr }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let inner = unsafe { &*self.ptr.as_ptr() };
        &inner.value
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let inner: &MyRcInner<T> = unsafe { &*self.ptr.as_ptr() };
        inner.count.set(inner.count.get() - 1);

        if inner.count.get() == 0 {
            unsafe {
                core::ptr::drop_in_place(self.ptr.as_ptr());
                alloc::alloc::dealloc(self.ptr.as_ptr() as *mut u8, get_layout::<MyRcInner<T>>());
            }
        }
    }
}

#[test]
fn foo() {
    let x = MyRc::new(String::from("hello"));
    assert_eq!(x.len(), 5);

    let y = MyRc::clone(&x);
    assert_eq!(y.len(), 5);

    // Borrows active:
    // - &y
    let y_ref: &MyRc<String> = &y;
    
    // Borrows active:
    // - &y
    // - &y.inner
    let y_inner: &String = y_ref;
    


    // With usize, clone does:
    // - takes the `ptr: *mut MyRcInner<String>`
    // - turns it into a `&mut MyRcInner<String>`  <-- ERROR: there is already a `&y.data` borrow - &String and `&mut RcInner<String>` at the same time, not allowed
    // - turns that into a `&mut count`
    // - adds one to the count
    // 
    // With Cell<usize>, clone does:
    // - takes the `ptr: *mut MyRcInner<String>`
    // - turns it into a `&MyRcInner<String>`
    // - turns that into a `&count`
    // - adds one to the count
    let z = MyRc::clone(y_ref);

    assert_eq!(y_inner.len(), 5);
    // Borrows active:
    // <nothing>
}

#[test]
fn they_really_are_pointing_to_the_same_string() {
    use std::cell::RefCell;
    
    let x = MyRc::new(RefCell::new(String::from("hello")));
    assert_eq!(x.borrow().len(), 5);

    let y = MyRc::clone(&x);
    assert_eq!(y.borrow().len(), 5);

    x.borrow_mut().push_str(" world");
    
    assert_eq!(x.borrow().len(), 11);
    assert_eq!(y.borrow().len(), 11);
}


#[test]
fn strong_weak() {
    use std::rc::Rc;
    use std::cell::RefCell;

   struct Person {
       name: String,
       best_friend: Option<Weak<RefCell<Person>>>,
   } 

   let a = Rc::new(RefCell::new(Person {
       name: "a".into(),
       best_friend: None,
   }));

   // a = 1
   // b = 0
   
   let b = Rc::new(RefCell::new(Person {
       name: "b".into(),
       best_friend: Some(Rc::downgrade(&a)),
   }));
   
   // a = 1  (+ 1 weak)
   // b = 1

   a.borrow_mut().best_friend = Some(Rc::downgrade(&b));
   
   // a = 1  (+ 1 weak)
   // b = 1  (+ 1 weak)

   drop(a);
   // a = 0  (+ 1 weak)  DROPPED(Person)
   // b = 1  (+ 0 weak)  
   drop(b);
   // a = 0  (+ 0 weak)  DROPPED(Person, counts)
   // b = 0  (+ 0 weak)  DROPPED(Person, counts)


   struct DoublyLinkedList<T> {
       value: T,
       next: Option<Rc<RefCell<DoublyLinkedList<T>>>>,
       prev: Option<Weak<RefCell<DoublyLinkedList<T>>>>,
   }
   
}