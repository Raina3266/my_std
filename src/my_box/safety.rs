use alloc::alloc::{alloc, dealloc, Layout};

pub struct MyBoxI32 {
    /// This is always:
    /// - a pointer to a valid heap allocation
    /// - non-null
    /// - the correct size
    /// - the correct align
    address: *mut i32,
}

impl MyBoxI32 {

    fn layout() -> Layout {
        // SAFETY: `Layout::from_size_align_unchecked` requires that all preconditions of
        // `Layout::from_size_align` are met. These conditions are:
        // - align must be non-zero
        // - align must be a power of 2
        // - size must not overflow an `isize`
        // All of these are true
        unsafe { Layout::from_size_align_unchecked(4, 4) }
    }

    pub fn new(data: i32) -> Self {
        // SAFETY: It is safe to allocate space for an `i32` with this layout because it has the
        // correct size and align for an i32
        let address = unsafe { alloc(Self::layout()) } as *mut i32;

        // SAFETY: `core::ptr::write` requires that `address` is valid for writes, and is properly
        // aligned. It is valid for writes because we got it from `alloc`. It is correctly aligned
        // because we passed a `Layout` with align 4 to `alloc`, which guarantees that the returned
        // pointer will have at least alignment 4
        unsafe { core::ptr::write(address, data) };

        Self {address}
    }

    pub fn get(&self) -> i32 {
        // SAFETY: This address is guaranteed to always be a valid pointer to an i32, so it is safe
        // to dereference. Additionally, since `i32` is `Copy`, so there is no concern about `Drop`
        // behaviour of the created `i32`s
        unsafe { *self.address }
    }
}

impl Drop for MyBoxI32 {
    fn drop(&mut self) {
        // SAFETY: It is safe to deallocate because:
        // - self.address is currently allocated by the global allocator
        // - the layout is the same as the layout used to allocate this pointer
        unsafe { dealloc(self.address as *mut u8, Self::layout()) }; 
    }
}

#[test]
fn can_create_and_get() {
    let my_box = MyBoxI32::new(123);
    assert_eq!(my_box.get(), 123);
}