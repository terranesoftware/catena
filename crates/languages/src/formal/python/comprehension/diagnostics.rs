use catena_suite::traits::comprehension::Diagnostics;

pub struct PythonDiagnostics;

impl Diagnostics for PythonDiagnostics {
    fn diagnostics(
        &self,
        text: &[u8]
    ) {
        
    }
}