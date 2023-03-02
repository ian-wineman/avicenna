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
        
        // Get key press
        if let Some(ref args) = e.press_args() {
            doc.key_press(args);
        }

        // Parse document
        doc.parse();

        // Update cursor
        doc.update_cursor();
        
        // Render document
        doc.render(&mut window, e);
    }
}