mod description;
pub use description::Description;

mod diagnostic;
pub use diagnostic::Diagnostic;

mod highlight;
pub use highlight::Highlight;

pub trait Comprehension {
    type Description: Description;
    type Diagnostic: Diagnostic;
    type Highlight: Highlight;
}