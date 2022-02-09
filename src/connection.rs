use crate::x;
use xcb;
/*
 * Abstraction over xcb::base::Connection.
 */
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

    /// Returns the xcb::base::Connection it holds
    pub fn as_xcb_connection(&self) -> &xcb::base::Connection {
        &self.connection
    }


    /// Flushes the queued events into the X server.
    pub fn flush(&self) {
        self.connection.flush();
    }

    /// Blocking operation that returns an X event whenever it occurs.
    pub fn get_event(&self) -> Option<xcb::base::Event<xcb::ffi::xcb_generic_event_t>> {
        self.connection.wait_for_event()
    }

    /// Attaches an event listener into the root window of the Xsession. Fails if
    /// another window manager is already running
    pub fn connect_as_window_manager(&self) {
        use xcb::{
            CW_EVENT_MASK, EVENT_MASK_SUBSTRUCTURE_NOTIFY, EVENT_MASK_SUBSTRUCTURE_REDIRECT,
        };
        self.set_root_attributes_checked(&[(
            CW_EVENT_MASK,
            EVENT_MASK_SUBSTRUCTURE_NOTIFY | EVENT_MASK_SUBSTRUCTURE_REDIRECT,
        )]);
    }

    /// Gets the screen the connection was attached to
    pub fn get_screen(&self) -> xcb::Screen {
        self.connection
            .get_setup()
            .roots()
            .nth(self.screen_id as usize)
            .unwrap()
    }
    /// Sets a listener for X events on a window based on the masks provided
    pub fn set_window_attributes_checked(&self, window: xcb::Window, masks: &[(u32, u32)]) {
        xcb::change_window_attributes_checked(&self.connection, window, masks)
            .request_check()
            .unwrap();
    }

    /// Sets a listener for X events on the root window based on the masks provided
    pub fn set_root_attributes_checked(&self, masks: &[(u32, u32)]) {
        self.set_window_attributes_checked(self.get_screen().root(), masks);
    }

    /// Sets window properties
    pub fn set_window_configuration(&self, window: xcb::Window, new_geometry: x::Geometry) {
        xcb::configure_window(&self.connection, window, &new_geometry.as_config_values());
    }

    /// Sets window focus
    pub fn set_window_focus(&self, window: xcb::Window) {
        xcb::xproto::set_input_focus(&self.connection, xcb::xproto::INPUT_FOCUS_POINTER_ROOT as u8, window, xcb::base::CURRENT_TIME);
    }

}
