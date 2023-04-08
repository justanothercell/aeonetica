use std::rc::Rc;

use aeonetica_engine::log;

use crate::{
    renderer::window::events::Event,
    renderer::layer::Layer
};

use super::postprocessing::PostProcessingLayer;

struct LayerStack {
    layers: Vec<Rc<dyn Layer>>,
    insert_index: usize
}

impl LayerStack {
    fn new() -> Self {
        Self {
            layers: Vec::new(),
            insert_index: 0
        }
    }

    fn push(&mut self, layer: Rc<impl Layer + 'static>) {
        self.layers.insert(self.insert_index, layer);
        self.insert_index += 1;
    }

    fn push_overlay(&mut self, layer: Rc<impl Layer + 'static>) {
        self.layers.insert(self.insert_index, layer);
    }
}

pub struct Context {
    layer_stack: LayerStack,
    post_processing_layer: Option<Rc<dyn PostProcessingLayer>>
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            layer_stack: LayerStack::new(),
            post_processing_layer: None
        }
    }

    pub fn push(&mut self, layer: Rc<impl Layer + 'static>) {
        layer.on_attach();
        if layer.is_overlay() {
            self.layer_stack.push_overlay(layer);
        }
        else {
            self.layer_stack.push(layer);
        }
    }

    pub(crate) fn on_event(&mut self, event: Event) {
        for layer in self.layer_stack.layers.iter().rev() {
            let handled = layer.on_event(&event);
            if handled {
                return;
            }
        }

        log!("Unhandled Event: {event:?}");
    }

    pub(crate) fn on_update(&mut self) {
        self.layer_stack.layers.iter().for_each(|layer| layer.on_update());
    }

    pub fn set_post_processing_layer(&mut self, post_processing_layer: Rc<dyn PostProcessingLayer>) {
        self.post_processing_layer = Some(post_processing_layer);
    }

    pub fn unset_post_processing_layer(&mut self) {
        self.post_processing_layer = None;
    }

    pub(crate) fn post_processing_layer(&self) -> &Option<Rc<dyn PostProcessingLayer>> {
        &self.post_processing_layer
    }

    pub(crate) fn finish(self) {
        for layer in self.layer_stack.layers.iter() {
            layer.on_detach();
        }
        self.post_processing_layer.map(|layer| layer.on_detach());
    }
}
