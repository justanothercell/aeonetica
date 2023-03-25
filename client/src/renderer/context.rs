use std::{rc::Rc};

use aeonetica_engine::log;

use crate::{
    renderer::window::events::Event,
    renderer::layer::Layer
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
        self.layers.insert(self.insert_index, layer);
    }
}

pub struct Context {
    layer_stack: LayerStack,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            layer_stack: LayerStack::new(),
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
}