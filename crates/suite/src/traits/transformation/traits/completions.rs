use crate::traits::transformation::edit::Edit;

pub trait Completions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit>;
}