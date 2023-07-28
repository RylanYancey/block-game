
mod core;
mod event;
mod renderer;
mod res;
mod config;
mod resources;
mod systems;
mod sys;

use winit::event::VirtualKeyCode;

use winit::{
    event_loop::{
        EventLoop, ControlFlow,
    },
    window::WindowBuilder,
};

use event::CoreEvent;
use renderer::Renderer;
use resources::*;
use systems::*;
use sys::Systems;

pub use crate::sys::Stage;
pub use crate::core::Core;
pub use crate::resources::*;
pub use crate::res::Res;

pub type KeyCode = VirtualKeyCode;

pub trait CoreId {
    fn id() -> usize;
}

pub fn start<F>(loader: F) 
    where F: FnOnce(&mut Core)
{
    pollster::block_on(run(loader));
}

async fn run<F>(loader: F) 
    where F: FnOnce(&mut Core)
{
    // Initialize Core 
    let mut core = Core::new();

    // load core resources
    load_core_resources(&mut core);

    // load core systems
    load_core_systems(&mut core);

    // run the loader function
    loader(&mut core);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("block game")
        .build(&event_loop).unwrap();

    // instantiate the renderer
    let mut renderer = Renderer::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        
        let event = CoreEvent::get_event(event, renderer.get_window().id());

        match event {
            CoreEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit
            },

            CoreEvent::Resized(physical_size) => {
                renderer.resize(physical_size)
            },

            CoreEvent::ScaleFactorChanged(new_inner_size) => {
                renderer.resize(new_inner_size);
            },

            CoreEvent::RedrawRequested => {
                
            },

            CoreEvent::MainEventsCleared => {
                renderer.get_window().request_redraw();
            },

            CoreEvent::KeyboardInput(keycode) => {
                let input = core.res.get_mut::<Input>();
                input.update(keycode);
            },

            _ => { }
        }

    });
}