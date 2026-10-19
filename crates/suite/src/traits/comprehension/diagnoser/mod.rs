pub mod diagnostic;
pub use diagnostic::Diagnostic;

pub trait Diagnoser {
    fn diagnose(
        &self,
        text: &[u8]
    ) -> Vec<Diagnostic>;
}