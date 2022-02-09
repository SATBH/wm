use crate::x;
use xcb;

pub struct Connection {
    connection: xcb::base::Connection,
    screen_id: i32,
}

impl Connection {
    pub fn new() -> Connection {
        let (connection, screen_id) = xcb::base::Connection::connect(None).unwrap();
        Connection {
            connection,
            screen_id,
        }
    }

    pub fn as_xcb_connection(&self) -> &xcb::base::Connection {
        &self.connection
    }

    pub fn flush(&self) {
        self.connection.flush();
    }

    pub fn get_event(&self) -> Option<xcb::base::Event<xcb::ffi::xcb_generic_event_t>> {
        self.connection.wait_for_event()
    }

    pub fn connect_as_window_manager(&self) {
        use xcb::{
            CW_EVENT_MASK, EVENT_MASK_SUBSTRUCTURE_NOTIFY, EVENT_MASK_SUBSTRUCTURE_REDIRECT,
        };
        self.set_root_attributes_checked(&[(
            CW_EVENT_MASK,
            EVENT_MASK_SUBSTRUCTURE_NOTIFY | EVENT_MASK_SUBSTRUCTURE_REDIRECT,
        )]);
    }

    pub fn get_screen(&self) -> xcb::Screen {
        self.connection
            .get_setup()
            .roots()
            .nth(self.screen_id as usize)
            .unwrap()
    }

    pub fn set_window_attributes_checked(&self, window: xcb::Window, masks: &[(u32, u32)]) {
        xcb::change_window_attributes_checked(&self.connection, window, masks)
            .request_check()
            .unwrap();
    }

    pub fn set_root_attributes_checked(&self, masks: &[(u32, u32)]) {
        self.set_window_attributes_checked(self.get_screen().root(), masks);
    }

    pub fn set_window_configuration(&self, window: xcb::Window, new_geometry: x::Geometry) {
        xcb::configure_window(&self.connection, window, &new_geometry.as_config_values());
    }
}
