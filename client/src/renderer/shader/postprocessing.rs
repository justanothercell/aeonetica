use super::shader;

pub trait PostProcessingLayer {
    fn on_attach(&self);
    fn on_detach(&self);

    fn post_processing_shader(&self) -> &shader::Program;
    fn uniforms<'a>(&self) -> &'a[(&'a str, &'a dyn shader::Uniform)] { &[] }
}
