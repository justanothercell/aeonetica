use std::cell::RefMut;
use std::rc::Rc;

use aeonetica_engine::error::{ErrorValue, IntoError, Error, Fatality};
use aeonetica_engine::{log, error::ErrorResult, TypeId};
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::type_to_id;

use crate::{
    renderer::window::events::Event,
    renderer::layer::Layer, client_runtime::ClientRuntime, data_store::DataStore
};
use crate::client_runtime::ClientHandleBox;
use crate::renderer::Renderer;

use super::shader::PostProcessingLayer;

#[derive(Debug)]
struct LayerAlreadyExists(&'static str);

impl ErrorValue for LayerAlreadyExists {}

impl std::fmt::Display for LayerAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "layer \"{}\" already exists", self.0)
    }
}

impl IntoError for LayerAlreadyExists {
    fn into_error(self) -> Error {
        Error::new(self, Fatality::WARN, false)
    }
}

struct LayerStack {
    layer_checker: IdMap<()>,
    layers: Vec<(Rc<dyn Layer>, TypeId)>,
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

    fn push<L: Layer + 'static>(&mut self, layer: Rc<L>) {
        self.layers.insert(self.insert_index, (layer, type_to_id::<L>()));
        self.insert_index += 1;
    }

    fn push_overlay<L: Layer + 'static>(&mut self, layer: Rc<L>) {
        self.layers.insert(self.insert_index, (layer, type_to_id::<L>()));
    }
}

pub struct LayerHandles<'a> {
    handles: &'a mut IdMap<ClientHandleBox>,
    layer_id: TypeId
}

impl<'a> LayerHandles<'a> {
    pub fn update(self, renderer: &mut RefMut<Renderer>, store: &mut DataStore, delta_time: f64){
        self.handles.values_mut().filter(|chb| chb.handle.owning_layer() == self.layer_id).for_each(|chb| chb.handle.update(&mut chb.messenger, renderer, store, delta_time))
    }
}

pub struct RenderContext {
    layer_stack: LayerStack,
    post_processing_layer: Option<Rc<dyn PostProcessingLayer>>
}

impl RenderContext {
    pub(crate) fn new() -> Self {
        Self {
            layer_stack: LayerStack::new(),
            post_processing_layer: None
        }
    }

    /// Fails if the layer already exists.
    pub fn push<L: Layer + 'static>(&mut self, layer: Rc<L>) -> ErrorResult<()> {
        if self.layer_stack.layer_checker.contains_key(&type_to_id::<L>()) {
            return Err(LayerAlreadyExists(layer.name()).into_error());
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
        for (layer, _id) in self.layer_stack.layers.iter().filter(|(layer, _)| layer.active()).rev() {
            let handled = layer.on_event(handles, &event);
            if handled {
                return;
            }
        }

        log!("Unhandled Event: {event:?}");
    }

    pub(crate) fn on_update(&mut self, client: &mut ClientRuntime, store: &mut DataStore, delta_time: f64) {
        let handles = client.handles();
        self.layer_stack.layers.iter().filter(|(layer, _)| layer.active()).for_each(|(layer, id)| layer.on_update(store, LayerHandles {
            handles,
            layer_id: *id,
        }, delta_time));
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
        for (layer, _) in self.layer_stack.layers.iter() {
            layer.on_quit();
        }
        if let Some(layer) = self.post_processing_layer { layer.on_detach() }
    }
}
