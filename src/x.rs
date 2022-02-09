/*
 * Abstraction over window geometries to implement some conveniences
 */
#[derive(Clone)]
pub struct Geometry {
    position: (u32, u32),
    size: (u32, u32),
}

impl Geometry {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Geometry {
        Geometry {
            position: (x, y),
            size: (width, height),
        }
    }
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn as_config_values(&self) -> [(u16, u32); 4] {
        let &Geometry {
            position: (x, y),
            size: (width, height),
        } = self;
        [
            (xcb::CONFIG_WINDOW_X as u16, x),
            (xcb::CONFIG_WINDOW_Y as u16, y),
            (xcb::CONFIG_WINDOW_WIDTH as u16, width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, height),
        ]
    }
}
