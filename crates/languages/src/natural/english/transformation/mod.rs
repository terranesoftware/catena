mod completions;
use completions::EnglishCompletions;

mod formatter;
use formatter::EnglishFormatter;

mod propagator;
use propagator::EnglishPropagator;

mod suggestions;
use suggestions::EnglishSuggestions;

use catena_suite::traits::Transformation;

use crate::natural::English;

impl Transformation for English {
    type Completions = EnglishCompletions;
    type Formatter = EnglishFormatter;
    type Propagator = EnglishPropagator;
    type Suggestions = EnglishSuggestions;
}