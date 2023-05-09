use std::any::type_name;
use std::rc::Rc;

use aeonetica_engine::log;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::type_to_id;

use crate::{
    renderer::window::events::Event,
    renderer::layer::Layer, client_runtime::ClientRuntime, data_store::DataStore
};

use super::shader::PostProcessingLayer;

struct LayerStack {
    layer_checker: IdMap<()>,
    layers: Vec<Rc<dyn Layer>>,
    insert_index: usize
}

impl LayerStack {
    fn new() -> Self {
        Self {
            layer_checker: Default::default(),
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

    /// Fails if the layer already exists.
    pub fn push<L: Layer + 'static>(&mut self, layer: Rc<L>) -> Result<(), String> {
        if self.layer_stack.layer_checker.contains_key(&type_to_id::<L>()) {
            return Err(format!("Layer {} was already pushed onto the layer stack", type_name::<L>()));
        }
        self.layer_stack.layer_checker.insert(type_to_id::<L>(), ());
        layer.on_attach();
        if layer.is_overlay() {
            self.layer_stack.push_overlay(layer);
        }
        else {
            self.layer_stack.push(layer);
        }
        Ok(())
    }

    pub(crate) fn on_event(&mut self, client: &mut ClientRuntime, event: Event) {
        let handles = client.handles();
        for layer in self.layer_stack.layers.iter().filter(|layer| layer.active()).rev() {
            let handled = layer.on_event(handles, &event);
            if handled {
                return;
            }
        }

        log!("Unhandled Event: {event:?}");
    }

    pub(crate) fn on_update(&mut self, client: &mut ClientRuntime, store: &mut DataStore, delta_time: f64) {
        let handles = client.handles();
        self.layer_stack.layers.iter().filter(|layer| layer.active()).for_each(|layer| layer.on_update(store, handles, delta_time));
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
        if let Some(layer) = self.post_processing_layer { layer.on_detach() }
    }
}
