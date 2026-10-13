mod grammar;
use grammar::RustGrammar;

mod index;
use index::RustIndex;

use catena_suite::traits::Traversal;

use crate::formal::Rust;

impl Traversal for Rust {
    type Grammar = RustGrammar;
    type Index = RustIndex;
}