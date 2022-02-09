use crate::x::Geometry;
use xcb;

pub trait Layout {
    fn get_geometries(&self, viewport: &Geometry) -> Vec<(xcb::Window, Geometry)>;
    fn add_window(&mut self, window: xcb::Window);
//    fn remove_window(&mut self, window: xcb::Window);
//    fn swap_windows(&mut self, first: xcb::Window, second: xcb::Window) -> [xcb::Window;2];
}

pub struct StackLayout {
    windows: Vec<xcb::Window>,
}
impl StackLayout {
    pub fn new() -> StackLayout {
        StackLayout { windows: vec![]}
    }
}

impl Layout for StackLayout {
    fn get_geometries(&self, viewport: &Geometry) -> Vec<(xcb::Window, Geometry)> {
        let mut acc = Vec::new();
        match self.windows.len() {
            0 => {},
            1 => {
                acc.push((self.windows[0], viewport.clone()));
            },
            _ => {
                let (width, mut height) = viewport.size();
                acc.push((self.windows[0], Geometry::new(
                    0,0, width/2, height))
                );
                height = height/(self.windows.len() as u32 - 1);
                for (index, &window) in self.windows[1..].iter().enumerate() {
                    acc.push((window, Geometry::new(width/2, height * index as u32, width, height)))
                }
            }
        }
        acc
    }
    fn add_window(&mut self, window: xcb::Window) {
        self.windows.push(window)
    }
}
