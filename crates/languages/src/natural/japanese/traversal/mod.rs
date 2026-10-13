mod grammar;
use grammar::JapaneseGrammar;

mod index;
use index::JapaneseIndex;

use catena_suite::traits::Traversal;

use crate::natural::Japanese;

impl Traversal for Japanese {
    type Grammar = JapaneseGrammar;
    type Index = JapaneseIndex;
}