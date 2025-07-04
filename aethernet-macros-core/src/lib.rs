pub mod collect;
pub mod generate;
pub mod helpers;
pub mod typing;

// re-export deps so they don't need to be redefined in the proc macro crate
pub mod deps {
    pub use quote;
    pub use syn;
}
