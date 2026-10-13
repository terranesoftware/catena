mod completion;
pub use completion::Completion;

mod formatter;
pub use formatter::Formatter;

mod propagator;
pub use propagator::Propagator;

mod suggestion;
pub use suggestion::Suggestion;

pub trait Transformation {
    type Completion: Completion;
    type Formatter: Formatter;
    type Propagator: Propagator;
    type Suggestion: Suggestion;
}