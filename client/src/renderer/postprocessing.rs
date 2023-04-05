use super::shader;

pub trait PostProcessingLayer {
    fn on_attach(&self);
    fn on_detach(&self);

    fn shader(&self) -> &shader::Program;
}
