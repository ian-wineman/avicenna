use piston_window::Button::Keyboard;
use piston_window::Key;	
use piston_window::*;

pub struct Document {
    data: Vec<String>,
    data_length: usize,
    parsed_data: Vec<String>,
    font_path: std::path::PathBuf,
    font_size: u32,
    char_width: f64,
    cursor: usize,       // cursor position in data
    cursor_x: usize,     // cursor position in parsed_data
    cursor_y: usize,     // cursor position in parsed_data
    cursor_pixel_x: f64, // cursor position in window
    cursor_pixel_y: f64, // cursor position in window
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
            char_width: 0.0,
            cursor: 0,
            cursor_x: 0,
            cursor_y: 0,
            cursor_pixel_x: 0.0,
            cursor_pixel_y: 0.0,
        }
    }

    pub fn update_cursor(&mut self) {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut cursor_counter: usize = 0;

        for s in self.data.iter() {
            if s.to_string() == String::from("Return") && cursor_counter < self.cursor {
                x = 0;
                y += 1;
            }
            else if cursor_counter < self.cursor {
                x += 1;
            }
            else {
                //
            }

            cursor_counter += 1;
        }
        self.cursor_x = x;
        self.cursor_y = y;
    }

    pub fn append(&mut self, symbol: String) {
        self.data_length += 1;

        // Append at cursor position
        let mut counter: usize = 0;
        let mut result: Vec<String> = Vec::new();

        for s in self.data.iter() {
            
            if counter != self.cursor {
                result.push(s.to_string());
            }
            else {
                result.push((*symbol).to_string());
                result.push(s.to_string());
            }
            
            counter += 1;
        }
        self.cursor += 1;
        self.data = result;
    }

    pub fn remove(&mut self) {
        self.data_length -= 1;

        // Remove at cursor position
        let mut counter: usize = 0;
        let mut result: Vec<String> = Vec::new();

        for s in self.data.iter() {
            
            if self.cursor > 0 {
                if counter != self.cursor - 1 {
                    result.push(s.to_string());
                }
                else {
                    //
                }
            }
            
            counter += 1;
        }
        if self.cursor > 0 {
            self.cursor -= 1;
        }
        self.data = result;
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

        // Testing
        println!("");
        println!("data_length: {}", self.data_length);
        println!("cursor: {}", self.cursor);
        println!("data: {:?}", self.data);
        println!("parsed_data: {:?}", self.parsed_data);
        println!("x: {}, y: {}", self.cursor_x, self.cursor_y);
        println!("px: {}, py: {}", self.cursor_pixel_x, self.cursor_pixel_y);
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
                let transform = c.transform.trans(6.0, self.font_size as f64 * line_counter + 1.0);
    
                // Draw on window
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], self.font_size)
                    .draw(line, &mut glyphs, &c.draw_state, transform, g)
                    .unwrap();
    
                glyphs.factory.encoder.flush(device);
            });
            line_counter += 1.0;
        }

        // Draw cursor
        window.draw_2d(&event, |c, g, _| {
            let transform = c.transform.trans(6.0 + self.char_width * self.cursor_x as f64, self.font_size as f64 * self.cursor_y as f64 + 1.0);

            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 1.5, self.font_size as f64], // rectangle
                      transform, g);
        });
    }

    pub fn mouse_move(&mut self, args: &[f64; 2]) {
        //println!("{:?}", *args);

        self.cursor_pixel_x = args[0];
        self.cursor_pixel_y = args[1];
    }

    #[allow(unused_assignments)]
    pub fn key_press(&mut self, args: &Button) {
        //println!("{:?}", *args);

        let key: String = match *args {
            Keyboard(Key::A) => { String::from("a") },
            Keyboard(Key::B) => { String::from("b") },
            Keyboard(Key::C) => { String::from("c") },
            Keyboard(Key::D) => { String::from("d") },
            Keyboard(Key::E) => { String::from("e") },
            Keyboard(Key::F) => { String::from("f") },
            Keyboard(Key::G) => { String::from("g") },
            Keyboard(Key::H) => { String::from("h") },
            Keyboard(Key::I) => { String::from("i") },
            Keyboard(Key::J) => { String::from("j") },
            Keyboard(Key::K) => { String::from("k") },
            Keyboard(Key::L) => { String::from("l") },
            Keyboard(Key::M) => { String::from("m") },
            Keyboard(Key::N) => { String::from("n") },
            Keyboard(Key::O) => { String::from("o") },
            Keyboard(Key::P) => { String::from("p") },
            Keyboard(Key::Q) => { String::from("q") },
            Keyboard(Key::R) => { String::from("r") },
            Keyboard(Key::S) => { String::from("s") },
            Keyboard(Key::T) => { String::from("t") },
            Keyboard(Key::U) => { String::from("u") },
            Keyboard(Key::V) => { String::from("v") },
            Keyboard(Key::W) => { String::from("w") },
            Keyboard(Key::X) => { String::from("x") },
            Keyboard(Key::Y) => { String::from("y") },
            Keyboard(Key::Z) => { String::from("z") },
            Keyboard(Key::D0) => { String::from("0") },
            Keyboard(Key::D1) => { String::from("1") },
            Keyboard(Key::D2) => { String::from("2") },
            Keyboard(Key::D3) => { String::from("3") },
            Keyboard(Key::D4) => { String::from("4") },
            Keyboard(Key::D5) => { String::from("5") },
            Keyboard(Key::D6) => { String::from("6") },
            Keyboard(Key::D7) => { String::from("7") },
            Keyboard(Key::D8) => { String::from("8") },
            Keyboard(Key::D9) => { String::from("9") },
            Keyboard(Key::Backspace) => { String::from("Backspace") }, 
            Keyboard(Key::Space) => { String::from(" ") },
            Keyboard(Key::Return) => { String::from("Return") },
            Keyboard(Key::Right) => { String::from("RightArrow") },
            Keyboard(Key::Left) => { String::from("LeftArrow") },
            Keyboard(Key::Up) => { String::from("UpArrow") },
            Keyboard(Key::Down) => { String::from("DownArrow") },
            _ => { String::from("") }
        };

        if key == String::from("Backspace") {
            if self.data_length != 0 && self.cursor != 0 {
                self.remove();
                self.update_cursor();
            }
        }
        else if key == String::from("RightArrow") {
            if self.cursor < self.data_length {
                self.cursor += 1;
            }
        }
        else if key == String::from("LeftArrow") {
            if self.cursor != 0 {
                self.cursor -= 1;
            }
        }
        else if key == String::from("UpArrow") {
            if self.cursor_y != 0 {

                if self.parsed_data[self.cursor_y].len() < self.parsed_data[self.cursor_y - 1].len() {
                    self.cursor -= self.parsed_data[self.cursor_y - 1].len() + 1; 
                }
                else if self.parsed_data[self.cursor_y].len() == self.parsed_data[self.cursor_y - 1].len() {
                    self.cursor -= self.parsed_data[self.cursor_y].len() + 1; 
                }
                else if self.parsed_data[self.cursor_y].len() > self.parsed_data[self.cursor_y - 1].len() && self.cursor_x <= self.parsed_data[self.cursor_y - 1].len() {
                    self.cursor -= self.parsed_data[self.cursor_y - 1].len() + 1;
                }
                else {
                    self.cursor -= self.cursor_x + 1
                }
            }
        }
        else if key == String::from("DownArrow") {
            if self.cursor_y != self.parsed_data.len() - 1 {

                if self.parsed_data[self.cursor_y].len() < self.parsed_data[self.cursor_y + 1].len() {
                    self.cursor += self.parsed_data[self.cursor_y].len() + 1;
                }
                else if self.parsed_data[self.cursor_y].len() == self.parsed_data[self.cursor_y + 1].len() {
                    self.cursor += self.parsed_data[self.cursor_y].len() + 1; 
                }
                else if self.parsed_data[self.cursor_y].len() > self.parsed_data[self.cursor_y + 1].len() && self.cursor_x <= self.parsed_data[self.cursor_y + 1].len() {
                    self.cursor += self.parsed_data[self.cursor_y].len() + 1; 
                }
                else {
                    self.cursor += self.parsed_data[self.cursor_y + 1].len() + self.parsed_data[self.cursor_y].len() - self.cursor_x + 1;
                }
            }
        }
        else if key == String::from("") {
            // Do nothing, unrecognized symbol
        }
        else {
            self.append(key);
            self.update_cursor();
        }
    }
}