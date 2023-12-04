// contents of rust_debug_utils/src/lib.rs
use std::fmt;

#[derive(Debug)]
pub struct TypeOf<T>(pub T);

impl<T: fmt::Debug> fmt::Display for TypeOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " <{}> = {:#?}", std::any::type_name::<T>(), self.0)
    }
}

#[macro_export]
macro_rules! debug {
    ($x:ident) => {
        println!("{{{}}}{}", stringify!($x), $crate::TypeOf($x));
    };
}

