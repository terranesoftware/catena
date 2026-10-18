mod traits;
pub use traits::{Grammar, Index};

pub trait Traversal {
    type Grammar: Grammar;
    type Index: Index;
}