use xcb::base::Connection;

use xcb::{
    MAP_REQUEST, CW_EVENT_MASK, EVENT_MASK_SUBSTRUCTURE_REDIRECT, EVENT_MASK_SUBSTRUCTURE_NOTIFY, Window, GenericEvent
};

use xcb;
use std::process::Command;
fn main() {
    let (connection, _) = Connection::connect(Some(":2")).unwrap();
    let screen = connection.get_setup().roots().nth(0).unwrap();
    let mut windows: Vec<Window> = Vec::new();
    let root: Window = screen.root();
    //set_wallpaper();
    xcb::change_window_attributes_checked(
        &connection,
         root,
     &[(CW_EVENT_MASK, EVENT_MASK_SUBSTRUCTURE_NOTIFY | EVENT_MASK_SUBSTRUCTURE_REDIRECT)]).request_check().unwrap();
    while let Some(event) = &connection.wait_for_event() {
        match event.response_type() {
            MAP_REQUEST => {
                let casted_event: &xcb::MapRequestEvent = unsafe {
                    xcb::cast_event(event as &GenericEvent)
                };
                windows.push(casted_event.window());
                xcb::map_window(&connection, casted_event.window());
                manage_windows(
                    &connection,
                    (screen.width_in_pixels() as u32, screen.height_in_pixels() as u32),
                    &windows
                );

                connection.flush();
            },
            xcb::DESTROY_NOTIFY => {
                let casted_event: &xcb::DestroyNotifyEvent= unsafe {
                    xcb::cast_event(event as &GenericEvent)
                };
                if let Some(index) = windows.iter().position(|&x| x == casted_event.window()) {
                    windows.remove(index);
                }
                manage_windows(
                    &connection,
                    (screen.width_in_pixels() as u32, screen.height_in_pixels() as u32),
                    &windows
                );
                connection.flush();
            },
            _ => {}
        }
    }
}

fn set_wallpaper() {
    Command::new("sh")
        .args(["-c", "DISPLAY=:2 hsetroot -cover /home/satbh/wallpapers/everforest"])
        .output()
        .unwrap();
}

fn manage_windows(connection: &Connection ,rootwh: (u32, u32), windows: &[Window]) {
    match windows.len() {
        0 => {},
        1 => {
            let (width, height) = rootwh;
            let window = windows[0];
            let values: [(u16, u32); 4] = [
                (xcb::CONFIG_WINDOW_X as u16, 10),
                (xcb::CONFIG_WINDOW_Y as u16, 10),
                (xcb::CONFIG_WINDOW_WIDTH as u16, width - 20),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, height - 20)
            ];
            xcb::configure_window(&connection, window, &values);
        }
        _ => {
            let (width, height) = rootwh; 
            let master_values: [(u16, u32); 4] = [
                (xcb::CONFIG_WINDOW_X as u16, 0),
                (xcb::CONFIG_WINDOW_Y as u16, 0),
                (xcb::CONFIG_WINDOW_WIDTH as u16, width/2),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, height)
            ];
            xcb::configure_window(&connection, windows[0], &master_values);
            for (index, &window) in windows[1..].into_iter().enumerate() {
                let  (width, height) = (width/2, height/(windows.len() as u32 - 1));
                let values: [(u16, u32); 4] = [
                    (xcb::CONFIG_WINDOW_X as u16, width),
                    (xcb::CONFIG_WINDOW_Y as u16, height * index as u32),
                    (xcb::CONFIG_WINDOW_WIDTH as u16, width),
                    (xcb::CONFIG_WINDOW_HEIGHT as u16, height)
                ];
                xcb::configure_window(&connection, window, &values);
            }
        }
    }
}
