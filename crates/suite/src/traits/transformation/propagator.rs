pub trait Propagator {
    fn propagator(
        &self,
        text: &[u8]
    );
}