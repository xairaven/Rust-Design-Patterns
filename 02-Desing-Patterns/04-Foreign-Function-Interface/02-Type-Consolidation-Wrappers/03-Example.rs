pub mod unsafe_module {

    // other module content

    pub fn myset_store(myset: *mut MySetWrapper, key: datum, value: datum) -> libc::c_int {
        // DO NOT USE THIS CODE. IT IS UNSAFE TO DEMONSTRATE A PROLBEM.

        let myset: &mut MySet = unsafe {
            // SAFETY: whoops, UB occurs in here!
            &mut (*myset).myset
        };

        /* ...check and cast key and value data... */

        match myset.store(casted_key, casted_value) {
            Ok(_) => 0,
            Err(e) => e.into(),
        }
    }
}