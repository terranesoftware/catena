mod indexer;
use indexer::PythonIndexer;

mod segmenter;
use segmenter::PythonSegmenter;

use catena_suite::traits::Traversal;

use crate::formal::Python;

impl Traversal for Python {
    type Indexer = PythonIndexer;
    type Segmenter = PythonSegmenter;
}