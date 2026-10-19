use crate::traits::transformation::edit::Edit;

pub trait Completer {
    fn complete(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit>;
}