pub use macros::GenDBOperations;
pub use futures::stream::BoxStream;
pub use futures::{StreamExt, TryStreamExt};

#[macro_export]
macro_rules! create_bound {
    ($($derives:ident),*) => {
        #[derive($($derives),*)]
        pub struct Bound<T> {
            pub min: Option<T>,
            pub max: Option<T>
        }
    };
}