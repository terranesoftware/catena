use crate::traits::transformation::edit::Edit;

pub trait Propagator {
    fn propagator(
        &self,
        text: &[u8]
    ) -> Vec<Edit>;
}