mod traits;
pub use traits::{Indexer, Segmenter};

pub trait Traversal {
    type Indexer: Indexer;
    type Segmenter: Segmenter;
}