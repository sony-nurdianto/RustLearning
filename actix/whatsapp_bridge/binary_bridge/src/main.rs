use core::panic;

use bridge_lib::app::app::tokio_main;

fn main() {
    let server = tokio_main();
    match server {
        Ok(app) => app,
        Err(err) => panic!("Failed To Run Server {err}"),
    }
}
