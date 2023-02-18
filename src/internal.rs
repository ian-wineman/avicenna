use piston_window::Button::Keyboard;
use piston_window::Key;	
use piston_window::*;

pub struct Document {
    data: Vec<String>,
    data_length: usize,
    parsed_data: Vec<String>,
    font_path: std::path::PathBuf,
    font_size: u32,
    cursor_x: usize, // cursor x, relative to elemnents of parsed_data
    cursor_y: usize, // cursor y, relative to elemnents of parsed_data
    char_width: f64,
}

impl Document {
    pub fn new() -> Document {
        // Find folder with fonts
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("fonts")
            .unwrap();

        let path = assets.join("UbuntuMono-Regular.ttf");

        return Document { 
            data: vec![String::from("")], 
            data_length: 0, 
            parsed_data: vec![String::from("")], 
            font_path: path, 
            font_size: 18,
            cursor_x: 0,
            cursor_y: 0,
            char_width: 0.0,
        }
    }

    pub fn append(&mut self, symbol: String) {
        self.data.push(symbol);
        self.data_length += 1;
    }

    pub fn remove(&mut self) {
        self.data.truncate(self.data_length - 1);
        self.data_length -= 1;
    }

    pub fn parse(&mut self) {
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

    pub fn render(&mut self, window: &mut PistonWindow, event: Event) {
        // White background
        window.draw_2d(&event, |_c, g, _device| {
            clear([1.0, 1.0, 1.0, 1.0], g);
        });

        // Load font
        let mut glyphs = window
            .load_font(&self.font_path)
            .unwrap();

        // Set font width
        self.char_width = glyphs.width(self.font_size, "a").expect("").ceil();

        // Draw lines of text
        let mut line_counter = 1.0;
        for line in self.parsed_data.iter() {
            window.draw_2d(&event, |c, g, device| {
                // Determine window position to draw to
                //let transform = c.transform.trans(8.0, 18.0*line_counter);
                let transform = c.transform.trans(6.0, 18.0*line_counter + 1.0);
    
                // Draw on window
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], self.font_size)
                    .draw(line, &mut glyphs, &c.draw_state, transform, g)
                    .unwrap();
    
                glyphs.factory.encoder.flush(device);
            });
            line_counter += 1.0;
        }
    }

    #[allow(unused_assignments)]
    pub fn key_press(&mut self, args: &Button) {
        //println!("{:?}", *args);

        let mut key: String = String::from("");

        match *args {
            //Keyboard(Key::A, Key::LShift) => self.append(String::from("A")),
            Keyboard(Key::A) => key = String::from("a"),
            Keyboard(Key::B) => key = String::from("b"),
            Keyboard(Key::C) => key = String::from("c"),
            Keyboard(Key::D) => key = String::from("d"),
            Keyboard(Key::E) => key = String::from("e"),
            Keyboard(Key::F) => key = String::from("f"),
            Keyboard(Key::G) => key = String::from("g"),
            Keyboard(Key::H) => key = String::from("h"),
            Keyboard(Key::I) => key = String::from("i"),
            Keyboard(Key::J) => key = String::from("j"),
            Keyboard(Key::K) => key = String::from("k"),
            Keyboard(Key::L) => key = String::from("l"),
            Keyboard(Key::M) => key = String::from("m"),
            Keyboard(Key::N) => key = String::from("n"),
            Keyboard(Key::O) => key = String::from("o"),
            Keyboard(Key::P) => key = String::from("p"),
            Keyboard(Key::Q) => key = String::from("q"),
            Keyboard(Key::R) => key = String::from("r"),
            Keyboard(Key::S) => key = String::from("s"),
            Keyboard(Key::T) => key = String::from("t"),
            Keyboard(Key::U) => key = String::from("u"),
            Keyboard(Key::V) => key = String::from("v"),
            Keyboard(Key::W) => key = String::from("w"),
            Keyboard(Key::X) => key = String::from("x"),
            Keyboard(Key::Y) => key = String::from("y"),
            Keyboard(Key::Z) => key = String::from("z"),
            Keyboard(Key::D0) => key = String::from("0"),
            Keyboard(Key::D1) => key = String::from("1"),
            Keyboard(Key::D2) => key = String::from("2"),
            Keyboard(Key::D3) => key = String::from("3"),
            Keyboard(Key::D4) => key = String::from("4"),
            Keyboard(Key::D5) => key = String::from("5"),
            Keyboard(Key::D6) => key = String::from("6"),
            Keyboard(Key::D7) => key = String::from("7"),
            Keyboard(Key::D8) => key = String::from("8"),
            Keyboard(Key::D9) => key = String::from("9"),
            Keyboard(Key::Backspace) => key = String::from("Backspace"), 
            Keyboard(Key::Space) => key = String::from(" "),
            Keyboard(Key::Return) => key = String::from("Return"),
            Keyboard(Key::Tab) => key = String::from("    "),
            _ => key = String::from("")
        }

        if key == String::from("Backspace") {
            if self.data_length != 0 {
                self.remove();
            }
        }
        else if key == String::from("") {
            // Do nothing, unrecognized symbol
        }
        else {
            self.append(key);
        }
    }
}