mod completion;
use completion::RustCompletion;

mod formatter;
use formatter::RustFormatter;

mod propagator;
use propagator::RustPropagator;

mod suggestion;
use suggestion::RustSuggestion;

use catena_core::traits::Transformation;

use crate::formal::Rust;

impl Transformation for Rust {
    type Completion = RustCompletion;
    type Formatter = RustFormatter;
    type Propagator = RustPropagator;
    type Suggestion = RustSuggestion;
}