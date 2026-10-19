mod indexer;
pub use indexer::Indexer;

mod segmenter;
pub use segmenter::Segmenter;

pub trait Traversal {
    type Indexer: Indexer;
    type Segmenter: Segmenter;
}