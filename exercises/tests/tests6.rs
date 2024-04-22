// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
   // SAFETY: The `ptr` contains an owned box of `Foo` by contract.
    // We reconstruct the box from the raw pointer using Box::from_raw.
    let ret: Box<Foo> = Box::from_raw(ptr);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: Some("hello".to_owned()) });

        let ptr_1 = &data.a as *const u128 as usize;
        
        // SAFETY: We pass an owned box of `Foo`.
        let raw_ptr = Box::into_raw(data);
        
        // SAFETY: We pass a valid raw pointer to `raw_pointer_to_box`.
        let ret = unsafe { raw_pointer_to_box(raw_ptr) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert_eq!(ret.b, Some("hello".to_owned()));  
    }
}
