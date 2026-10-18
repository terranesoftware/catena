use catena_suite::traits::transformation::{Edit, Propagator};

pub struct JapanesePropagator;

impl Propagator for JapanesePropagator {
    fn propagator(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}