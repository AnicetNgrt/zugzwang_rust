use std::fmt;
use std::cmp;
use std::vec::Vec;
use unicode_width::UnicodeWidthChar;

pub type Grid = Vec<Vec<String>>;

pub struct Canvas {
    pub grid: Grid,
    pub width: usize,
    pub height: usize,
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();

        let canvas = self;

        string += "\n";
        for line in canvas.grid.iter() {
            for s in line.iter() {
                string += s;
            }
            string += "\n";
        }
  
        write!(f, "{}", string)
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid::new();

        for _ in 0..height {
            let mut line: Vec<String> = Vec::new();
            for _ in 0..width {
                line.push(space());
            }
            grid.push(line);
        }

        Canvas { grid, width, height }
    }

    pub fn put<S: ToString>(&mut self, x: usize, y: usize, s: S) {
        self.grid[y][x] = s.to_string();
        let mut screen_width = 0;
        for c in s.to_string().chars() {
            screen_width += UnicodeWidthChar::width(c).unwrap_or(1);
        }

        for shift in 1..screen_width {
            self.grid[y][x+shift] = empty();
        }
    }

    pub fn put_splitted<S: ToString>(&mut self, x: usize, y: usize, s: S) {
        for (count, ch)  in s.to_string().chars().enumerate() {
            self.put(x+count, y, String::from(ch));
        }
    }

    pub fn get(&self, x: usize, y: usize) -> String {
        self.grid[y][x].to_owned()
    }

    pub fn add_borders(&self) -> Self {
        let mut canvas = self.add_padding(1, 1, 1, 1);
        
        for x in 1..canvas.width-1 {
            canvas.put(x, 0, "─");
        }
        for y in 1..canvas.height-1 {
            canvas.put(0, y, "│");
            canvas.put(canvas.width-1, y, "│");
        }
        for x in 1..canvas.width-1 {
            canvas.put(x, canvas.height-1, "─");
        }
        canvas.put(0, 0, "┌");
        canvas.put(canvas.width-1, 0, "┐");
        canvas.put(0, canvas.height-1, "└");
        canvas.put(canvas.width-1, canvas.height-1, "┘");

        canvas
    }

    pub fn add_padding(&self, left: usize, right: usize, top: usize, bottom: usize) -> Self {
        let mut canvas = Canvas::new(self.width + left + right, self.height + top + bottom);
        canvas.stamp(self, left, top);
        canvas
    }

    pub fn stamp(&mut self, canvas: &Canvas, left: usize, top: usize) {
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                self.put(x + left, y + top, canvas.get(x, y));
            }
        }
    }

    pub fn add_right(&self, right: &Canvas) -> Self {
        let mut canvas = Canvas::new(
            self.width+right.width,
            cmp::max(self.height, right.height)
        );
        canvas.stamp(self, 0, 0);
        canvas.stamp(right, self.width, 0);
        canvas
    }

    pub fn add_bottom(&self, bottom: &Canvas) -> Self {
        let mut canvas = Canvas::new(
            cmp::max(self.width, bottom.width),
            self.height+bottom.height,
        );
        canvas.stamp(self, 0, 0);
        canvas.stamp(bottom, 0, self.height);
        canvas
    }
}

fn space() -> String {
    String::from(" ")
}

fn empty() -> String {
    String::new()
}

pub trait Drawable {
    fn to_canvas(&self) -> Canvas;
}