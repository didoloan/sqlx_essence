pub use essence_macros::GenDBOperations;
pub use futures::stream::BoxStream;
pub use futures::{StreamExt, TryStreamExt};

#[macro_export]
macro_rules! create_bound {
    ($($derives:ident),*) => {
        #[derive(Debug, Default, $($derives),*)]
        pub struct Bound<T> {
            pub min: Option<T>,
            pub max: Option<T>
        }
    };
}

#[cfg(test)]
mod mysql_crud_tests;

