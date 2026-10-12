pub mod comprehension;
pub use comprehension::Comprehension;

pub mod transformation;
pub use transformation::Transformation;

pub mod traversal;
pub use traversal::Traversal;

pub trait Language: Comprehension + Transformation + Traversal {}