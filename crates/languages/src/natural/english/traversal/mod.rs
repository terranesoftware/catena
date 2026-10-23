mod index;
use index::EnglishIndexer;

mod segmenter;
use segmenter::EnglishSegmenter;

use catena_suite::traits::Traversal;

use crate::natural::English;

impl Traversal for English {
    type Indexer = EnglishIndexer;
    type Segmenter = EnglishSegmenter;
}