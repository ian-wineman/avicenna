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
    caps: bool, // is caps enabled by LShift, RShift, or caps lock?
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
            caps: false, 
        }
    }

    pub fn update_cursor(&mut self) {
        // Update cursor_x and cursor_y given cursor
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
        /*
        println!("");
        println!("data_length: {}", self.data_length);
        println!("cursor: {}", self.cursor);
        println!("data: {:?}", self.data);
        println!("parsed_data: {:?}", self.parsed_data);
        println!("x: {}, y: {}", self.cursor_x, self.cursor_y);
        println!("px: {}, py: {}", self.cursor_pixel_x, self.cursor_pixel_y);
        */
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

    pub fn key_release(&mut self, args: &Button) {
        let key: &str = match *args {
            Keyboard(Key::LShift) => { "LShift" },
            Keyboard(Key::RShift) => { "RShift" },
            _ => { "" }
        };

        match key {
            "LShift" | "RShift" => { self.caps = false },
            _ => (),
        }
    }

    pub fn key_press(&mut self, args: &Button) {
        //println!("{:?}", *args);

        let key: &str = match *args {
            Keyboard(Key::A) => { "a" },
            Keyboard(Key::B) => { "b" },
            Keyboard(Key::C) => { "c" },
            Keyboard(Key::D) => { "d" },
            Keyboard(Key::E) => { "e" },
            Keyboard(Key::F) => { "f" },
            Keyboard(Key::G) => { "g" },
            Keyboard(Key::H) => { "h" },
            Keyboard(Key::I) => { "i" },
            Keyboard(Key::J) => { "j" },
            Keyboard(Key::K) => { "k" },
            Keyboard(Key::L) => { "l" },
            Keyboard(Key::M) => { "m" },
            Keyboard(Key::N) => { "n" },
            Keyboard(Key::O) => { "o" },
            Keyboard(Key::P) => { "p" },
            Keyboard(Key::Q) => { "q" },
            Keyboard(Key::R) => { "r" },
            Keyboard(Key::S) => { "s" },
            Keyboard(Key::T) => { "t" },
            Keyboard(Key::U) => { "u" },
            Keyboard(Key::V) => { "v" },
            Keyboard(Key::W) => { "w" },
            Keyboard(Key::X) => { "x" },
            Keyboard(Key::Y) => { "y" },
            Keyboard(Key::Z) => { "z" },
            Keyboard(Key::D0) => { "0" },
            Keyboard(Key::D1) => { "1" },
            Keyboard(Key::D2) => { "2" },
            Keyboard(Key::D3) => { "3" },
            Keyboard(Key::D4) => { "4" },
            Keyboard(Key::D5) => { "5" },
            Keyboard(Key::D6) => { "6" },
            Keyboard(Key::D7) => { "7" },
            Keyboard(Key::D8) => { "8" },
            Keyboard(Key::D9) => { "9" },
            Keyboard(Key::Backspace) => { "Backspace" }, 
            Keyboard(Key::Space) => { " " },
            Keyboard(Key::Return) => { "Return" },
            Keyboard(Key::Right) => { "RightArrow" },
            Keyboard(Key::Left) => { "LeftArrow" },
            Keyboard(Key::Up) => { "UpArrow" },
            Keyboard(Key::Down) => { "DownArrow" },
            Keyboard(Key::LShift) => { "LShift" },
            Keyboard(Key::RShift) => { "RShift" },
            Keyboard(Key::Minus) => { "-" },
            Keyboard(Key::Equals) => { "=" },
            Keyboard(Key::LeftBracket) => { "[" },
            Keyboard(Key::RightBracket) => { "]" },
            Keyboard(Key::Backslash) => { "\\" },
            Keyboard(Key::Semicolon) => { ";" },
            Keyboard(Key::Period) => { "." },
            Keyboard(Key::Comma) => { "," },
            Keyboard(Key::Slash) => { "/" },
            Button::Mouse(MouseButton::Left) => { "LeftMouse" },
            _ => { "" }
        };

        match key {
            "LShift" | "RShift" => {
                self.caps = true;
            },
            "Backspace" => {
                if self.data_length != 0 && self.cursor != 0 {
                    self.remove();
                    self.update_cursor();
                }
            },
            "RightArrow" => {
                if self.cursor < self.data_length {
                    self.cursor += 1;
                }
            },
            "LeftArrow" => {
                if self.cursor != 0 {
                    self.cursor -= 1;
                }
            },
            "UpArrow" => {
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
            },
            "DownArrow" => {
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
            },
            "LeftMouse" => {
                // update cursor based on cursor_pixel_x and cursor_pixel_y

                '_iter: for cursor_position in 0..(self.data_length + 1) {
                    let mut x: usize = 0;
                    let mut y: usize = 0;
                    let mut cursor_counter: usize = 0;

                    for s in self.data.iter() {
                        if s.to_string() == String::from("Return") && cursor_counter < cursor_position {
                            x = 0;
                            y += 1;
                        }
                        else if cursor_counter < cursor_position {
                            x += 1;
                        }
                        else {
                            //
                        }

                        cursor_counter += 1;
                    }
                    let tmp_cursor_x = x;
                    let tmp_cursor_y = y;

                    let tmp_cursor_pixel_x = 6.0 + self.char_width * tmp_cursor_x as f64;
                    let tmp_cursor_pixel_y = self.font_size as f64 * tmp_cursor_y as f64 + 1.0;

                    // is self.char_width the right margin to use here?
                    if (tmp_cursor_pixel_x - self.cursor_pixel_x).abs() < 0.8*self.char_width && (tmp_cursor_pixel_y - self.cursor_pixel_y).abs() < 1.5*self.char_width {                    
                        self.cursor = cursor_position;
                        self.update_cursor();
                        break '_iter;
                    }
                }
            },
            "" => {
                // Do nothing, unrecognized symbol
            },
            _ => { 
                match self.caps {
                    true => {
                        match key {
                            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" => {
                                self.append(String::from(key.to_uppercase()));
                            },
                            "0" => { self.append(String::from(")")) },
                            "1" => { self.append(String::from("!")) },
                            "2" => { self.append(String::from("@")) },
                            "3" => { self.append(String::from("#")) },
                            "4" => { self.append(String::from("$")) },
                            "5" => { self.append(String::from("%")) },
                            "6" => { self.append(String::from("^")) },
                            "7" => { self.append(String::from("&")) },
                            "8" => { self.append(String::from("*")) },
                            "9" => { self.append(String::from("(")) },
                            "-" => { self.append(String::from("_")) },
                            "=" => { self.append(String::from("+")) },
                            "[" => { self.append(String::from("{")) },
                            "]" => { self.append(String::from("}")) },
                            "\\" => { self.append(String::from("|")) },
                            ";" => { self.append(String::from(":")) },
                            "." => { self.append(String::from(">")) },
                            "," => { self.append(String::from("<")) },
                            "/" => { self.append(String::from("?")) },
                            _ => { },
                        }
                    },
                    false => {
                        self.append(String::from(key));
                    },
                }
                self.update_cursor();
            },
        }
    }
}