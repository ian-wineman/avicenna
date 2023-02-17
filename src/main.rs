extern crate find_folder;
extern crate piston_window;

use piston_window::*;

mod input;
use input::key_press;

struct Document {
    data: Vec<String>,
    data_length: usize,
    parsed_data: Vec<String>,
    font_path: std::path::PathBuf,
    //cursor_x: f64,
    //cursor_y: f64,
}

impl Document {
    fn new() -> Document {
        // Find folder with fonts
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("fonts")
            .unwrap();

        let path = assets.join("UbuntuMono-Regular.ttf");

        return Document { data: vec![String::from("")], data_length: 0, parsed_data: vec![String::from("")], font_path: path }
    }

    fn append(&mut self, symbol: String) {
        self.data.push(symbol);
        self.data_length += 1;
    }

    fn remove(&mut self) {
        self.data.truncate(self.data_length - 1);
        self.data_length -= 1;
    }

    fn parse(&mut self) {
        let mut working_string: String = String::new();
        let mut parsed_strings: Vec<String> = Vec::new();

        for c in self.data.iter() {
            if *c == String::from("Return") {
                parsed_strings.push(working_string);
                working_string = String::from("");
            }
            else {
                working_string = working_string + &c;
            }
        }

        parsed_strings.push(working_string);

        self.parsed_data = parsed_strings;
    }

    /*
    fn render(&self) {
        //
    }
    */
}


fn main() {
    // Create Document
    let mut doc: Document = Document::new();

    // Create Window
    let mut window: PistonWindow = WindowSettings::new("Avicenna", [600, 600])
        .build()
        .unwrap();

    // Event Loop
    window.set_lazy(true);
    while let Some(e) = window.next() {
        // Get key press
        if let Some(ref args) = e.press_args() {
            let new_symbol = key_press(args);
            //println!("{}", new_symbol);

            if new_symbol == String::from("Backspace") {
                if doc.data_length != 0 {
                    doc.remove();
                }
            }
            else if new_symbol == String::from("Return") {
                doc.append(new_symbol);
            }
            else if new_symbol == String::from("") {
                // do nothing
            }
            else {
                doc.append(new_symbol);
            }
        }

        // Set window background to white
        window.draw_2d(&e, |_c, g, _device| {
            clear([1.0, 1.0, 1.0, 1.0], g);
        });

        /*
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("fonts")
            .unwrap();
        */

        // Load font
        let mut glyphs = window
            .load_font(&doc.font_path)
            //.load_font(assets.join("UbuntuMono-Regular.ttf"))
            .unwrap();

        // Draw lines of text
        //doc.render();
        let mut line_counter = 1.0;
        doc.parse();
        for line in doc.parsed_data.iter() {
            window.draw_2d(&e, |c, g, device| {
                // Determine window position to draw to
                let transform = c.transform.trans(8.0, 20.0*line_counter);
    
                // Draw on window
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 18)
                    .draw(line, &mut glyphs, &c.draw_state, transform, g)
                    .unwrap();
    
                glyphs.factory.encoder.flush(device);
            });
            line_counter += 1.0;
        }
    }
}