mod edit;
pub use edit::Edit;

mod traits;
pub use traits::{Completions, Formatter, Propagator, Suggestions};

pub trait Transformation {
    type Completions: Completions;
    type Formatter: Formatter;
    type Propagator: Propagator;
    type Suggestions: Suggestions;
}