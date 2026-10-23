mod indexer;
use indexer::JapaneseIndexer;

mod segmenter;
use segmenter::JapaneseSegmenter;

use catena_suite::traits::Traversal;

use crate::natural::Japanese;

impl Traversal for Japanese {
    type Indexer = JapaneseIndexer;
    type Segmenter = JapaneseSegmenter;
}