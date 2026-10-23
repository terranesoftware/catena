use catena_suite::traits::comprehension::{Diagnoser, diagnoser::Diagnostic};

pub struct EnglishDiagnoser;

impl Diagnoser for EnglishDiagnoser {
    fn diagnose(
        &self,
        text: &[u8]
    ) -> Vec<Diagnostic> {
        Vec::new()
    }
}