mod grammar;
pub use grammar::Grammar;

mod index;
pub use index::Index;

pub trait Traversal {
    type Grammar: Grammar;
    type Index: Index;
}