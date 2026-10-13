mod grammar;
use grammar::PythonGrammar;

mod index;
use index::PythonIndex;

use catena_suite::traits::Traversal;

use crate::formal::Python;

impl Traversal for Python {
    type Grammar = PythonGrammar;
    type Index = PythonIndex;
}