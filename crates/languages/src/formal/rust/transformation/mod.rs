mod completions;
use completions::RustCompletions;

mod formatter;
use formatter::RustFormatter;

mod propagator;
use propagator::RustPropagator;

mod suggestions;
use suggestions::RustSuggestions;

use catena_suite::traits::Transformation;

use crate::formal::Rust;

impl Transformation for Rust {
    type Completions = RustCompletions;
    type Formatter = RustFormatter;
    type Propagator = RustPropagator;
    type Suggestions = RustSuggestions;
}