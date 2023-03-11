mod internal;
use crate::internal::*;
use piston_window::*;

fn main() {
    // Create document
    let mut doc: Document = Document::new();

    // Create window
    let mut window: PistonWindow = WindowSettings::new("Avicenna", [600, 600])
        .build()
        .unwrap();

    // Event loop
    window.set_lazy(true);
    while let Some(e) = window.next() {

        // Match on event
        match e {
            Event::Input(ref input, _option) => {
                match input {
                    Input::Button(buttonargs) => {
                        match buttonargs.state {
                            ButtonState::Press => doc.key_press(&buttonargs.button),
                            ButtonState::Release => doc.key_release(&buttonargs.button), 
                        }
                    },
                    Input::Move(motion) => {
                        match motion {
                            Motion::MouseCursor(coords) => doc.mouse_move(coords),
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }
            _ => (),
        }

        // Parse document
        doc.parse();

        // Update cursor
        doc.update_cursor();
        
        // Render document
        doc.render(&mut window, e);
    }
}