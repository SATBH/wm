use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn handle_client(mut stream: UnixStream) -> usize {
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let x = response.parse::<usize>().unwrap();
    x
}

pub fn start_server(mutable: Arc<Mutex<usize>>) {
    let listener = UnixListener::bind("/tmp/wm").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                *mutable.lock().unwrap() = handle_client(stream);
                println!("value changed");
            }
            Err(err) => {
                /* connection failed */
                break;
            }
        }
    }
}
