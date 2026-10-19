use catena_suite::traits::comprehension::{Diagnoser, diagnoser::Diagnostic};

pub struct RustDiagnoser;

impl Diagnoser for RustDiagnoser {
    fn diagnose(
        &self,
        text: &[u8]
    ) -> Vec<Diagnostic> {
        Vec::new()
    }
}