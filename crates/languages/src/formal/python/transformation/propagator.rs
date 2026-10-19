use catena_suite::traits::transformation::{Edit, Propagator};

pub struct PythonPropagator;

impl Propagator for PythonPropagator {
    fn propagate(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}