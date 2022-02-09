mod connection;
mod layouts;
mod x;
use layouts::Layout;

use xcb;

fn main() {
    use connection::Connection;
    /*
     * Here we instantiate a connection and make it so
     * it listens for events we care about in X, like
     * window creation and destruction.
     */
    let connection = Connection::new();
    connection.connect_as_window_manager();
    /*
     * Current default Layout based on dwm. (Master & Stack)
     */
    let mut layout = layouts::StackLayout::new();

    /*
     * This is the event loop. Where we receive all the
     * required events from the X server and act accordingly
     * based on the event type.
     */
    while let Some(event) = &connection.get_event() {
        match event.response_type() {
            /*
             * MAP_REQUEST handler. It's called whenever a window wants
             * to be shown in the X server.
             */
            xcb::MAP_REQUEST => {
                let casted_event: &xcb::MapRequestEvent =
                    unsafe { xcb::cast_event(event as &xcb::GenericEvent) };
                xcb::map_window(connection.as_xcb_connection(), casted_event.window());
                layout.add_window(casted_event.window());
                let geometries = layout.get_geometries(&x::Geometry::new(
                    0,
                    0,
                    connection.get_screen().width_in_pixels() as u32,
                    connection.get_screen().height_in_pixels() as u32,
                ));

                for (window, geometry) in geometries {
                    connection.set_window_configuration(window, geometry)
                }
            }
            /*
             * DESTROY_NOTIFY handler. It's called whenever a window dies.
             */
            xcb::DESTROY_NOTIFY => {
                let casted_event: &xcb::DestroyNotifyEvent =
                    unsafe { xcb::cast_event(event as &xcb::GenericEvent) };
                layout.remove_window(casted_event.window())
            }
            _ => {}
        }
        connection.flush()
    }
}
