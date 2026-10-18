mod completions;
use completions::JapaneseCompletions;

mod formatter;
use formatter::JapaneseFormatter;

mod propagator;
use propagator::JapanesePropagator;

mod suggestions;
use suggestions::JapaneseSuggestions;

use catena_suite::traits::Transformation;

use crate::natural::Japanese;

impl Transformation for Japanese {
    type Completions = JapaneseCompletions;
    type Formatter = JapaneseFormatter;
    type Propagator = JapanesePropagator;
    type Suggestions = JapaneseSuggestions;
}