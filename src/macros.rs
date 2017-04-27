// TODO: This could be named better
macro_rules! impl_llvm_ref {
    ($dest: tt, $ref: ty) => {
        impl From<$ref> for $dest {
            fn from(ptr: $ref) -> Self {
                $dest {
                    ptr: ptr,
                }
            }
        }

        impl From<$dest> for $ref {
            fn from(s: $dest) -> Self {
                s.ptr
            }
        }
    }
}

// This should only be used for static strings
macro_rules! c_str_to_str {
    ($s:expr) => {
        ::std::str::from_utf8(CStr::from_ptr($s).to_bytes()).unwrap()
    }
}

