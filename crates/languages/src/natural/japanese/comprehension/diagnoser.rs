use catena_suite::traits::comprehension::{Diagnoser, diagnoser::Diagnostic};

pub struct JapaneseDiagnoser;

impl Diagnoser for JapaneseDiagnoser {
    fn diagnose(
        &self,
        text: &[u8]
    ) -> Vec<Diagnostic> {
        Vec::new()
    }
}