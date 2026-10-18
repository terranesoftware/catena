use crate::traits::transformation::edit::Edit;

pub trait Suggestions {
    fn suggestions(
        &self,
        text: &[u8]
    ) -> Vec<Edit>;
}