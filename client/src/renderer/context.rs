use std::cell::RefCell;
use std::rc::Rc;

use aeonetica_engine::{Id, TypeId, error::*, log, math::camera::Camera, util::{id_map::IdMap, type_to_id}};

use crate::client_runtime::ClientHandleBox;
use crate::{renderer::{window::events::Event, layer::Layer, Renderer}, client_runtime::ClientRuntime, data_store::DataStore};

use super::{buffer::framebuffer::FrameBuffer, layer::LayerUpdater, shader::PostProcessingLayer};

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

impl LayerBox {
    fn attach(&mut self) {
        self.layer.attach(&mut self.renderer)
    }

    fn quit(&mut self) {
        self.layer.quit(&mut self.renderer)
    }

    fn on_render(&mut self, id: &mut Id, handles: &mut IdMap<ClientHandleBox>, target: &FrameBuffer, store: &mut DataStore, delta_time: f64) {
        self.layer.update_camera(store, &mut self.camera, delta_time);
        self.renderer.on_layer_update(&self.camera, target, LayerUpdater::new(&mut self.layer, handles, *id, store), delta_time);
    }
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

        let mut l = LayerBox {
            camera: layer.instantiate_camera(),
            layer: Box::new(layer),
            renderer: Renderer::new(),
        };
        
        l.attach();

        let l: Rc<RefCell<_>> = Rc::new(RefCell::new(l));

        self.layer_map.insert(type_to_id::<L>(), l.clone());
        self.layer_stack.insert(self.insert_index, (l, type_to_id::<L>()));
        self.insert_index += 1;
    }

    fn push_overlay<L: Layer + 'static>(&mut self, layer: L) {
        let mut l = LayerBox {
            camera: layer.instantiate_camera(),
            layer: Box::new(layer),
            renderer: Renderer::new(),
        };

        l.attach();

        let l: Rc<RefCell<_>> = Rc::new(RefCell::new(l));

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
        if layer.is_overlay() {
            self.layer_stack.push_overlay(layer);
        }
        else {
            self.layer_stack.push(layer);
        }
        Ok(())
    }

    pub(crate) fn on_event(&mut self, client: &mut ClientRuntime, event: Event, store: &mut DataStore) {
        for (layer_box, id) in self.layer_stack.layer_stack.iter()
            .filter(|(layer_box, _)| layer_box.borrow().layer.active()).rev() {
            if layer_box.borrow_mut().layer.event(&event) { return; }
            if client.handles.iter_mut()
                .filter(|(_, h_box)| h_box.handle.owning_layer() == *id)
                .any(|(_, h_box)| h_box.handle.event(&event, &mut h_box.messenger, &mut layer_box.borrow_mut().renderer, store)) { return; }
        }

        log!(PACK, "Unhandled Event: {event:?}");
    }

    pub(crate) fn on_render(&mut self, client: &mut ClientRuntime, target: &FrameBuffer, store: &mut DataStore, delta_time: f64) {
        let handles = client.handles();
        self.layer_stack.layer_stack.iter_mut()
            .filter(|(layer_box, _)| layer_box.borrow().layer.active())
            .for_each(|(layer_box, id)| layer_box.borrow_mut().on_render(id, handles, target, store, delta_time));
    }

    pub fn set_post_processing_layer(&mut self, post_processing_layer: Rc<dyn PostProcessingLayer>) {
        post_processing_layer.attach();
        self.post_processing_layer = Some(post_processing_layer);
    }

    pub fn unset_post_processing_layer(&mut self) {
        if let Some(layer) = self.post_processing_layer.as_ref() { layer.detach() }
        self.post_processing_layer = None;
    }

    pub(crate) fn post_processing_layer(&self) -> &Option<Rc<dyn PostProcessingLayer>> {
        &self.post_processing_layer
    }

    pub(crate) fn finish(self) {
        for (layer_box, _) in self.layer_stack.layer_stack.iter() {
            layer_box.borrow_mut().quit();
        }
        self.post_processing_layer.map(|layer|layer.detach());
    }
}
