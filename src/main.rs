mod connection;
mod layouts;
mod socket;
mod x;
use connection::Connection;
use layouts::Layout;
use socket::server;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use xcb;

struct Workspaces {
    layouts: Vec<RefCell<layouts::StackLayout>>,
    current: Arc<Mutex<usize>>,
}

fn manage_windows(connection: &Connection, manager: &impl Layout) {
    let geometries = manager.get_geometries(&x::Geometry::new(
        0,
        0,
        connection.get_screen().width_in_pixels() as u32,
        connection.get_screen().height_in_pixels() as u32,
    ));

    for (window, geometry) in geometries {
        connection.set_window_configuration(window, geometry);
    }
}

fn main() {
    /*
     * Here we instantiate a connection and make it so
     * it listens for events we care about in X, like
     * window creation and destruction.
     */
    let connection = Connection::new();
    connection.connect_as_window_manager();

    let mut workspaces = Workspaces {
        layouts: (1..9)
            .map(|x: i32| RefCell::new(layouts::StackLayout::new(5)))
            .collect(),
        current: Arc::new(Mutex::new(0)),
    };

    {
        let current = workspaces.current.clone();
        let server_thread = std::thread::spawn(move || server::start_server(current));
    }

    /*
     * Current default Layout based on dwm. (Master & Stack)
     */
    let mut layout = RefCell::new(layouts::StackLayout::new(5));
    let mut current = 0;

    /*
     * This is the event loop. Where we receive all the
     * required events from the X server and act accordingly
     * based on the event type.
     */
    loop {
        if let Some(event) = &connection.poll_event() {
            match event.response_type() {
                /*
                 * MAP_REQUEST handler. It's called whenever a window wants
                 * to be shown in the X server.
                 */
                xcb::MAP_REQUEST => {
                    let casted_event: &xcb::MapRequestEvent =
                        unsafe { xcb::cast_event(event as &xcb::GenericEvent) };
                    xcb::map_window(connection.as_xcb_connection(), casted_event.window());
                    let layout = &workspaces.layouts[current];
                    layout.borrow_mut().add_window(casted_event.window());
                    connection.set_window_attributes_checked(casted_event.window(), &[(
                        xcb::CW_EVENT_MASK,
                        xcb::EVENT_MASK_ENTER_WINDOW,
                    )]);
                    manage_windows(&connection, &*layout.borrow());
                    connection.set_window_focus(casted_event.window());
                    connection.flush();
                }

                xcb::ENTER_NOTIFY => {
                    let casted_event: &xcb::EnterNotifyEvent =
                        unsafe { xcb::cast_event(event as &xcb::GenericEvent) };
                    connection.set_window_focus(casted_event.event());
                    connection.flush();
                }

                /*
                 * DESTROY_NOTIFY handler. It's called whenever a window dies.
                 */
                xcb::DESTROY_NOTIFY => {
                    let casted_event: &xcb::DestroyNotifyEvent =
                        unsafe { xcb::cast_event(event as &xcb::GenericEvent) };

                    for layout in &workspaces.layouts {
                        layout.borrow_mut().remove_window(casted_event.window());
                    }

                    let layout = &workspaces.layouts[current];
                    manage_windows(&connection, &*layout.borrow());
                    connection.flush();
                }
                _ => {
                    println!("{}", event.response_type())
                }
            }
        }
        
        let workspace_number = *workspaces.current.lock().unwrap();
        if current != workspace_number {
            let previous_layout = &workspaces.layouts[current];
            for window in (*previous_layout.borrow_mut()).get_windows() {
                xcb::unmap_window(connection.as_xcb_connection(), window);
            }
            current = workspace_number;
            manage_windows(&connection, &*workspaces.layouts[current].borrow());
            for window in (*workspaces.layouts[current].borrow_mut()).get_windows() {
                xcb::map_window(connection.as_xcb_connection(), window);
            }
        }
        connection.flush();
        
    }
}
