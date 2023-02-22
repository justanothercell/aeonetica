use std::{rc::Rc, vec};

use crate::{
    window::{Window, self},
    layers::Layer, events::Event
};

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
        self.layers.push(layer);
    }
}

pub(crate) struct Context {
    running: bool,
    layer_stack: LayerStack,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            running: false,
            layer_stack: LayerStack::new(),
        }
    }

    pub(crate) fn push(&mut self, layer: Rc<impl Layer + 'static>) {
        layer.on_attach();
        self.layer_stack.push(layer);
    }

    pub(crate) fn on_event(&mut self, event: Event) {
        println!("Event {event:?}");
    }

    pub(crate) fn on_update(&mut self) {

    }

    fn on_window_close(&mut self) -> bool {
        self.running = false;
        true
    }
}