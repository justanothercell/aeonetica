use std::{cell::RefCell, rc::Rc};

use crate::{context::Context, events::{Event, EventType, MouseButton}};

pub(crate) struct Window {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    context: Context
}

impl Window {
    pub(crate) fn new(context: Context) -> Option<Self> {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new().
            with_resizable(true)
            .with_title("Aeonetica")
            .with_visible(true)
            .with_transparent(false)
            .with_decorations(true)
            .with_active(true)
            .build(&event_loop).ok()?;
        Some(Self {
            window,
            event_loop,
            context
        })
    }

    pub(crate) fn run(self) {
        let mut context = self.context;
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            match event {
                winit::event::Event::RedrawRequested(_) => {

                }
                winit::event::Event::RedrawEventsCleared | winit::event::Event::MainEventsCleared | winit::event::Event::NewEvents(_) => {}
                _ => {
                    println!("{event:?}");
                    context.on_event(Event::new(event.into()));
                }
            }
            context.on_update();
        });
    }
}

impl Into<MouseButton> for winit::event::MouseButton {
    fn into(self) -> MouseButton {
        match self {
            Self::Left => MouseButton::Left,
            Self::Right => MouseButton::Right,
            Self::Middle => MouseButton::Middle,
            Self::Other(code) => MouseButton::Other(code)
        }
    }
}

impl<'a> Into<EventType> for winit::event::WindowEvent<'a> {
    fn into(self) -> EventType {
        match self {
            Self::Resized(size) => EventType::WindowResize(size.width, size.height),
            Self::CloseRequested {..} => EventType::WindowClose(),
            Self::CursorMoved { position, ..} => EventType::MouseMoved(position.x as f32, position.y as f32),
            Self::MouseWheel { delta, .. } => {
                match delta {
                    winit::event::MouseScrollDelta::PixelDelta(offset) => EventType::MouseScrolled(offset.x as f32, offset.y as f32),
                    winit::event::MouseScrollDelta::LineDelta(x, y) => EventType::MouseScrolled(x, y)
                }
            }
            Self::MouseInput { state, button, .. } => {
                match state {
                    winit::event::ElementState::Pressed => EventType::MouseButtonPressed(button.into()),
                    winit::event::ElementState::Released => EventType::MouseButtonReleased(button.into())
                }
            }
            Self::KeyboardInput { input, .. } => {
                match input.state {
                    winit::event::ElementState::Pressed => EventType::KeyPressed(input.scancode),
                    winit::event::ElementState::Released => EventType::KeyReleased(input.scancode)
                }
            }
            _ => EventType::Unknown()
        }
    }
}

impl<'a, T> Into<EventType> for winit::event::Event<'a, T> {
    fn into(self) -> EventType {
        match self {
            Self::WindowEvent { event, .. } => event.into(),
            _ => EventType::Unknown()
        }
    }
}