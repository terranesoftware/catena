mod grammar;
use grammar::JapaneseGrammar;

mod index;
use index::JapaneseIndex;

use catena_core::traits::Traversal;

use crate::natural::Japanese;

impl Traversal for Japanese {
    type Grammar = JapaneseGrammar;
    type Index = JapaneseIndex;
}