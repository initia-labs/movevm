use crate::memory::{ByteSliceView, UnmanagedVector};

#[test]
fn byte_slice_view_read_works() {
    let data = vec![0xAA, 0xBB, 0xCC];
    let view = ByteSliceView::new(&data);
    assert_eq!(view.read().unwrap(), &[0xAA, 0xBB, 0xCC]);

    let data = vec![];
    let view = ByteSliceView::new(&data);
    assert_eq!(view.read().unwrap(), &[] as &[u8]);

    let view = ByteSliceView::nil();
    assert!(view.read().is_none());
}

#[test]
fn byte_slice_view_to_owned_works() {
    let data = vec![0xAA, 0xBB, 0xCC];
    let view = ByteSliceView::new(&data);
    assert_eq!(view.to_owned().unwrap(), vec![0xAA, 0xBB, 0xCC]);

    let data = vec![];
    let view = ByteSliceView::new(&data);
    assert_eq!(view.to_owned().unwrap(), Vec::<u8>::new());

    let view = ByteSliceView::nil();
    assert!(view.to_owned().is_none());
}

#[test]
fn unmanaged_vector_new_works() {
    // With data
    let x = UnmanagedVector::new(Some(vec![0x11, 0x22]));
    assert!(!x.is_none());
    /*
    assert_ne!(x.ptr as usize, 0);
    assert_eq!(x.len, 2);
    assert_eq!(x.cap, 2);
    */

    // Empty data
    let x = UnmanagedVector::new(Some(vec![]));
    assert!(!x.is_none());
    /*
    assert_eq!(x.ptr as usize, 0x01); // We probably don't get any guarantee for this, but good to know where the 0x01 marker pointer can come from
    assert_eq!(x.len, 0);
    assert_eq!(x.cap, 0);
    */

    // None
    let x = UnmanagedVector::new(None);
    assert!(x.is_none());
    /*
    assert_eq!(x.ptr as usize, 0);
    assert_eq!(x.len, 0);
    assert_eq!(x.cap, 0);
    */
}

#[test]
fn unmanaged_vector_is_some_works() {
    let x = UnmanagedVector::new(Some(vec![0x11, 0x22]));
    assert!(x.is_some());
    let x = UnmanagedVector::new(Some(vec![]));
    assert!(x.is_some());
    let x = UnmanagedVector::new(None);
    assert!(!x.is_some());
}

#[test]
fn unmanaged_vector_is_none_works() {
    let x = UnmanagedVector::new(Some(vec![0x11, 0x22]));
    assert!(!x.is_none());
    let x = UnmanagedVector::new(Some(vec![]));
    assert!(!x.is_none());
    let x = UnmanagedVector::new(None);
    assert!(x.is_none());
}

#[test]
fn unmanaged_vector_consume_works() {
    let x = UnmanagedVector::new(Some(vec![0x11, 0x22]));
    assert_eq!(x.consume(), Some(vec![0x11u8, 0x22]));
    let x = UnmanagedVector::new(Some(vec![]));
    assert_eq!(x.consume(), Some(Vec::<u8>::new()));
    let x = UnmanagedVector::new(None);
    assert_eq!(x.consume(), None);
}

#[test]
fn unmanaged_vector_defaults_to_none() {
    let x = UnmanagedVector::default();
    assert_eq!(x.consume(), None);
}
