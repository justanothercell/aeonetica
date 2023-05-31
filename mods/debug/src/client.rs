use std::{marker::PhantomData, cell::{RefCell, RefMut}, rc::Rc};

use aeonetica_client::{ClientMod, renderer::{window::OpenGlRenderContextProvider, context::RenderContext, layer::Layer, Renderer, builtin::Line}};
use aeonetica_engine::math::vector::Vector2;
use aeonetica_engine::Color;

pub struct DebugModClient {

}

impl DebugModClient{
    pub(crate) fn new() -> Self {
        Self {}
    }
}


impl ClientMod for DebugModClient {

}

pub struct Debug<L: Layer> {
    renderer: Rc<RefCell<DebugRenderer<L>>>
}

impl<L: Layer> Default for Debug<L> {
    fn default() -> Self {
        Self { renderer: Rc::new(RefCell::new(
            DebugRenderer { 
                phantom: Default::default(), 
                is_rendering: false, 
                renderer_ptr: 0, 
                lines: vec![], 
                usage_index: 0 
            }
        )) }
    }
}

impl<L: Layer> Debug<L> {
    pub fn renderer(&self) -> RefMut<DebugRenderer<L>> {
        self.renderer.borrow_mut()
    }
}

pub struct DebugRenderer<L: Layer> {
    phantom: PhantomData<L>,
    is_rendering: bool,
    renderer_ptr: usize,
    lines: Vec<Line>,
    usage_index: usize,
}

#[cfg(debug_assertions)]
impl<L: Layer> DebugRenderer<L> {
    pub fn start_render(&mut self, renderer: &mut Renderer) {
        if self.is_rendering {
            panic!("Debug is already in rendering. Make sure to call Debug::<L>::start_render and Debug::<L>::finish_render in your layers pre and post update methods respectively.")
        }
        self.renderer_ptr = renderer as *mut Renderer as usize;
        self.usage_index = 0;
        self.is_rendering = true;
    }

    pub fn finish_render(&mut self, renderer: &mut Renderer) {
        if !self.is_rendering {
            panic!("Debug is not currently rendering. Make sure to call Debug::<L>::start_render and Debug::<L>::finish_render in your layers pre and post update methods respectively.")
        }
        if self.renderer_ptr != renderer as *mut Renderer as usize {
            panic!("Called start and finish with different renderers. Make sure to call Debug::<L>::start_render and Debug::<L>::finish_render in your layers pre and post update methods respectively.")
        }
        for (i, line) in self.lines.iter_mut().enumerate() {
            if i < self.usage_index {
                println!("{}: {}-{}", i, line.from(), line.to());
                renderer.draw(line).expect("unable to draw line");
            } else {
                renderer.remove(line);
            }
        }
        self.lines.truncate(self.usage_index);
        self.is_rendering = false;
    }

    pub fn line(&mut self, start: Vector2<f32>, end: Vector2<f32>, w: f32, color: Color) {
        self.assert_rendering();
        if self.lines.len() > self.usage_index {
            let line = &mut self.lines[self.usage_index];
            line.set_from(start);
            line.set_to(end);
            line.set_color(color);
            line.set_weight(w);
        } else {
            self.lines.push(Line::new(start, end, w, 255, color))
        }
        self.usage_index += 1;
    }

    pub fn rect(&mut self, pos: Vector2<f32>, size: Vector2<f32>, w: f32, color: Color) {
        self.line(pos, pos + (size.x, 0.0).into(), w, [1.0, 0.0, 0.0, 1.0]);
        self.line(pos + (size.x, 0.0).into(), pos + size, w, [0.0, 1.0, 0.0, 1.0]);
        self.line(pos + size, pos + (0.0, size.y).into(), w, [0.0, 0.0, 1.0, 1.0]);
        self.line(pos + (0.0, size.y).into(), pos, w, [1.0, 0.0, 1.0, 1.0]);
    }

    fn assert_rendering(&self){
        if !self.is_rendering {
            panic!("Cannot draw while not rendering. Make sure to call Debug::<L>::start_render and Debug::<L>::finish_render in your layers pre and post update methods respectively.")
        }
    }
}

#[cfg(not(debug_assertions))]
impl<L: Layer> DebugRenderer<L> {
    pub fn start_render(&mut self, renderer: &mut Renderer) {}

    pub fn finish_render(&mut self, renderer: &mut Renderer) {}

    pub fn line(&mut self, start: Vector2<f32>, end: Vector2<f32>, w: f32, color: Color) {}

    pub fn rect(&mut self, start: Vector2<f32>, end: Vector2<f32>, w: f32, color: Color) {}

    fn assert_rendering(&self){}
}