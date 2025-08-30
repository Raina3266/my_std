// Box is a pointer that stores some data on the heap. If the data is unsized, then it's two pointers

use alloc::alloc::{ alloc, dealloc, Layout};

pub struct MyBoxI32 {
    address: *mut i32,
}

impl MyBoxI32 {
    pub fn new(data: i32) -> Self {
    // how to the address of data
        // reference -> raw pointer -> address (usize)
        // let data_address = &data as *const i32 as usize;
        // newer version: &raw const data

    // move the pointer from stack to heap
        // size: how much space I need
        // align: must be a power of two - align 4 means "address must be multiple of 4"
        let layout = unsafe { Layout::from_size_align_unchecked(4,4) };
        // two types of raw pointers: *const, *mut
        let address = unsafe { alloc(layout) } as *mut i32;

    // initialise the heap memory
        // overwrites a memory location with the given value without reading or dropping the old value
        unsafe { core::ptr::write(address, data) };

        Self {address}
    }

    pub fn get(&self) -> i32 {
        unsafe { *self.address }
    }
}

impl Drop for MyBoxI32 {
    fn drop(&mut self) {
        let layout = unsafe { Layout::from_size_align_unchecked(4, 4) };
        unsafe { dealloc(self.address as *mut u8, layout) }
    }
}

#[test]
fn get_address() {
    let my_box = MyBoxI32::new(123);
    // {foo} prints foo using trait Display
    // {foo:?} prints foo using trait Debug
    // {foo:p} prints foo using trait Pointer (prints it like a pointer: 0x12345678)
    println!("address: {:p}",my_box.address);
}

#[test]
fn can_create_and_get() {
    let my_box = MyBoxI32::new(123);
    assert_eq!(my_box.get(), 123);
}