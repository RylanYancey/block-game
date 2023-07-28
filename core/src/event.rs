use winit::{dpi::PhysicalSize, event::{VirtualKeyCode, Event, WindowEvent, KeyboardInput, ElementState}, window::WindowId};

pub enum CoreEvent {
    CloseRequested,
    KeyboardInput(VirtualKeyCode),
    Resized(PhysicalSize<u32>),
    ScaleFactorChanged(PhysicalSize<u32>),
    RedrawRequested,
    MainEventsCleared,
    Null,
}

impl CoreEvent {
    pub fn get_event(event: Event<()>, id: WindowId) -> Self {
        match event {

            // -- // Process Window Events // -- //

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == id => match event {
                WindowEvent::CloseRequested => return CoreEvent::CloseRequested,

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => return CoreEvent::KeyboardInput(*keycode),

                WindowEvent::Resized(physical_size) => 
                    return CoreEvent::Resized(*physical_size),

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => 
                    return CoreEvent::ScaleFactorChanged(**new_inner_size),

                _ => return CoreEvent::Null,
            }

            // -- // Process other Events // -- //

            Event::RedrawRequested(window_id) if window_id == id => 
                return CoreEvent::RedrawRequested,

            Event::MainEventsCleared => 
                return CoreEvent::MainEventsCleared,

            _ => return CoreEvent::Null,
        }
    }
}