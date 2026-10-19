use catena_suite::traits::comprehension::{Diagnoser, diagnoser::Diagnostic};

pub struct PythonDiagnoser;

impl Diagnoser for PythonDiagnoser {
    fn diagnose(
        &self,
        text: &[u8]
    ) -> Vec<Diagnostic> {
        Vec::new()
    }
}