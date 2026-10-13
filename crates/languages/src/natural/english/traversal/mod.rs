mod grammar;
use grammar::EnglishGrammar;

mod index;
use index::EnglishIndex;

use catena_suite::traits::Traversal;

use crate::natural::English;

impl Traversal for English {
    type Grammar = EnglishGrammar;
    type Index = EnglishIndex;
}