mod indexer;
use indexer::RustIndexer;

mod segmenter;
use segmenter::RustSegmenter;

use catena_suite::traits::Traversal;

use crate::formal::Rust;

impl Traversal for Rust {
    type Indexer = RustIndexer;
    type Segmenter = RustSegmenter;
}