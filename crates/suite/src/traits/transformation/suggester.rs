use crate::traits::transformation::edit::Edit;

pub trait Suggester {
    fn suggest(
        &self,
        text: &[u8]
    ) -> Vec<Edit>;
}