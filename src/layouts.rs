use crate::x::Geometry;
use xcb;

/*
 * Common Interface for layouts to interact with the window manager.
 */
pub trait Layout {
    fn get_geometries(&self, viewport: &Geometry) -> Vec<(xcb::Window, Geometry)>;
    fn add_window(&mut self, window: xcb::Window);
    fn remove_window(&mut self, window: xcb::Window);
}

/*
 * Currently default layout.
 */
pub struct StackLayout {
    windows: Vec<xcb::Window>,
    gaps: u32,
}

impl StackLayout {
    pub fn new(gaps: u32) -> StackLayout {
        StackLayout {
            windows: vec![],
            gaps,
        }
    }
}

impl Layout for StackLayout {
    fn get_geometries(&self, viewport: &Geometry) -> Vec<(xcb::Window, Geometry)> {
        let mut acc = Vec::new();
        match self.windows.len() {
            0 => {}
            1 => {
                acc.push((self.windows[0], viewport.clone()));
            }
            _ => {
                let (width, mut height) = viewport.size();
                acc.push((
                    self.windows[0],
                    Geometry::new(
                        self.gaps,
                        self.gaps,
                        width / 2 - self.gaps,
                        height - self.gaps * 2,
                    ),
                ));
                height = height / (self.windows.len() as u32 - 1);
                for (index, &window) in self.windows[1..].iter().enumerate() {
                    acc.push((
                        window,
                        Geometry::new(
                            width / 2 + self.gaps,
                            height * index as u32 + self.gaps,
                            width / 2 - 2 * self.gaps,
                            height - self.gaps,
                        ),
                    ))
                }
            }
        }
        acc
    }

    fn add_window(&mut self, window: xcb::Window) {
        self.windows.push(window)
    }

    fn remove_window(&mut self, window: xcb::Window) {
        self.windows.retain(|&x| x != window);
    }
}
