extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

struct GameState;

impl EmptyState for GameState {
    fn on_start(&mut self, _: StateData<()>) {
        println!("Starting game!");
    }

    fn handle_event(&mut self, _: StateData<()>, event: StateEvent) -> EmptyTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                 Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                    } |
                    WindowEvent::CloseRequested => Trans::Quit,
                    _ => Trans::None,
                },
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, _: StateData<()>) -> EmptyTrans {
        println!("Computing some more whoop-ass...");
        Trans::Quit
    }
}

fn main() {
    let mut game = Application::new("assets/", GameState, ()).expect("Fatal error");
    game.run();
}
