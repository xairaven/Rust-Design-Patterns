struct Dbm { ... }

impl Dbm {
    /* ... */
    pub fn keys<'it>(&'it self) -> DbmKeysIter<'it> { ... }
    /* ... */
}

struct DbmKeysIter<'it> {
    owner: &'it Dbm,
}

impl<'it> Iterator for DbmKeysIter<'it> { ... }