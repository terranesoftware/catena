use crate::traits::transformation::edit::Edit;

pub trait Formatter {
    fn format(
        &self,
        text: &[u8]
    ) -> Vec<Edit>;
}