use catena_suite::traits::transformation::{Edit, Propagator};

pub struct EnglishPropagator;

impl Propagator for EnglishPropagator {
    fn propagator(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}