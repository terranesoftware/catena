mod completions;
pub use completions::Completions;

mod formatter;
pub use formatter::Formatter;

mod propagator;
pub use propagator::Propagator;

mod suggestions;
pub use suggestions::Suggestions;

pub trait Transformation {
    type Completions: Completions;
    type Formatter: Formatter;
    type Propagator: Propagator;
    type Suggestions: Suggestions;
}