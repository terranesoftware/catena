use catena_suite::traits::transformation::{Edit, Propagator};

pub struct RustPropagator;

impl Propagator for RustPropagator {
    fn propagator(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}