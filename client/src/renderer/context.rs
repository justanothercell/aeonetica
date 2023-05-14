use std::cell::RefCell;
use std::rc::Rc;

use aeonetica_engine::error::{ErrorValue, IntoError, Error, Fatality};
use aeonetica_engine::{log, error::ErrorResult, TypeId};
use aeonetica_engine::math::camera::Camera;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::type_to_id;

use crate::{
    renderer::window::events::Event,
    renderer::layer::Layer, client_runtime::ClientRuntime, data_store::DataStore
};
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
    fn into_error(self) -> Box<Error> {
        Error::new(self, Fatality::WARN, false)
    }
}

pub(crate) struct LayerBox {
    pub(crate) layer: Box<dyn Layer>,
    pub(crate) camera: Camera,
    pub(crate) renderer: Renderer
}

pub(crate) struct LayerStack {
    pub(crate) layer_map: IdMap<Rc<RefCell<LayerBox>>>,
    pub(crate) layer_stack: Vec<(Rc<RefCell<LayerBox>>, TypeId)>,
    insert_index: usize
}

impl LayerStack {
    fn new() -> Self {
        Self {
            layer_map: Default::default(),
            layer_stack: Vec::new(),
            insert_index: 0
        }
    }

    fn push<L: Layer + 'static>(&mut self, layer: L) {
        let l = Rc::new(RefCell::new(LayerBox {
            camera: layer.instantiate_camera(),
            layer: Box::new(layer),
            renderer: Renderer::new(),
        }));
        self.layer_map.insert(type_to_id::<L>(), l.clone());
        self.layer_stack.insert(self.insert_index, (l, type_to_id::<L>()));
        self.insert_index += 1;
        println!("push'led")
    }

    fn push_overlay<L: Layer + 'static>(&mut self, layer: L) {
        let l = Rc::new(RefCell::new(LayerBox {
            camera: layer.instantiate_camera(),
            layer: Box::new(layer),
            renderer: Renderer::new(),
        }));
        self.layer_map.insert(type_to_id::<L>(), l.clone());
        self.layer_stack.insert(self.insert_index, (l, type_to_id::<L>()));
    }
}

pub struct RenderContext {
    pub(crate) layer_stack: LayerStack,
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
    pub fn push<L: Layer + 'static>(&mut self, layer: L) -> ErrorResult<()> {
        if self.layer_stack.layer_map.contains_key(&type_to_id::<L>()) {
            return Err(LayerAlreadyExists(layer.name()).into_error());
        }
        layer.attach();
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
        for (layer_box, _id) in self.layer_stack.layer_stack.iter()
            .filter(|(layer_box, _)| layer_box.borrow().layer.active()).rev() {
            let handled = layer_box.borrow_mut().layer.event(handles, &event);
            if handled {
                return;
            }
        }

        log!("Unhandled Event: {event:?}");
    }

    pub(crate) fn on_update(&mut self, client: &mut ClientRuntime, store: &mut DataStore, delta_time: f64) {
        let handles = client.handles();
        self.layer_stack.layer_stack.iter_mut()
            .filter(|(layer_box, _)| layer_box.borrow().layer.active())
            .for_each(|(layer_box, id)| {
                let layer_box = &mut *layer_box.borrow_mut();
                layer_box.layer.update_camera(store, &mut layer_box.camera, delta_time);
                layer_box.renderer.begin_scene(&layer_box.camera);
                layer_box.layer.pre_handles_update(store, &mut layer_box.renderer, delta_time);
                handles.iter_mut()
                    .filter(|(_id, handle_box)| handle_box.handle.owning_layer() == *id)
                    .for_each(|(_id, handle_box)| handle_box.handle.update(&mut handle_box.messenger, &mut layer_box.renderer, store, delta_time));
                layer_box.layer.post_handles_update(store, &mut layer_box.renderer, delta_time);
                layer_box.renderer.draw_vertices();
                layer_box.renderer.end_scene();
            });
    }

    pub fn set_post_processing_layer(&mut self, post_processing_layer: Rc<dyn PostProcessingLayer>) {
        post_processing_layer.attach();
        self.post_processing_layer = Some(post_processing_layer);
    }

    pub fn unset_post_processing_layer(&mut self) {
        self.post_processing_layer.as_ref().map(|layer|layer.detach());
        self.post_processing_layer = None;
    }

    pub(crate) fn post_processing_layer(&self) -> &Option<Rc<dyn PostProcessingLayer>> {
        &self.post_processing_layer
    }

    pub(crate) fn finish(self) {
        for (layer_box, _) in self.layer_stack.layer_stack.iter() {
            layer_box.borrow().layer.quit();
        }
        self.post_processing_layer.map(|layer|layer.detach());
    }
}
