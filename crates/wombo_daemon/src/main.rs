use event_listener::Event;
use std::error::Error;
use zbus::{blocking::ConnectionBuilder, dbus_interface};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let server = Server { done: Event::new() };
    let done_listener = server.done.listen();
    let _ = ConnectionBuilder::system()?
        .name("org.matteonardi.WomboCombo")?
        .serve_at("/org/matteonardi/WomboCombo", server)?
        .build()?;

    done_listener.wait();
    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}

struct Server {
    done: Event,
}

#[dbus_interface(name = "org.matteonardi.WomboCombo")]
impl Server {
    fn say_hello(&self, name: &str) -> String {
        self.done.notify(1);
        format!("Hello {}!", name)
    }
}
