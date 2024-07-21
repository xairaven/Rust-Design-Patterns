#[no_mangle]
pub extern "C" fn dbm_iter_new(owner: *const Dbm) -> *mut DbmKeysIter {
    // THIS API IS A BAD IDEA! For real applications, use object-based design instead.
}
#[no_mangle]
pub extern "C" fn dbm_iter_next(
    iter: *mut DbmKeysIter,
    key_out: *const datum
) -> libc::c_int {
    // THIS API IS A BAD IDEA! For real applications, use object-based design instead.
}
#[no_mangle]
pub extern "C" fn dbm_iter_del(*mut DbmKeysIter) {
    // THIS API IS A BAD IDEA! For real applications, use object-based design instead.
}