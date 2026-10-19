use crate::traits::transformation::edit::Edit;

pub trait Propagator {
    fn propagate(
        &self,
        text: &[u8]
    ) -> Vec<Edit>;
}