mod connection;
mod x;
mod layouts;
use layouts::Layout;

use xcb;

fn main() {
    use connection::Connection;
    let connection = Connection::new();
    connection.connect_as_window_manager();
    let mut layout = layouts::StackLayout::new();
    while let Some(event) = &connection.get_event() {
        match event.response_type() {
            xcb::MAP_REQUEST => {
                let casted_event: &xcb::MapRequestEvent =
                    unsafe { xcb::cast_event(event as &xcb::GenericEvent) };
                xcb::map_window(connection.as_xcb_connection(), casted_event.window());
                layout.add_window(casted_event.window());
                let geometries = layout.get_geometries(&x::Geometry::new(
                    0,
                    0,
                    connection.get_screen().width_in_pixels() as u32,
                    connection.get_screen().height_in_pixels()as u32));

                for (window, geometry) in geometries {
                    connection.set_window_configuration(window, geometry)
                }
            }
            xcb::DESTROY_NOTIFY => {
                let casted_event: &xcb::DestroyNotifyEvent =
                    unsafe { xcb::cast_event(event as &xcb::GenericEvent) };
            }
            _ => {}
        }
        connection.flush()
    }
}
