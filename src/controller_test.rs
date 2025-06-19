use gilrs::{Gilrs, Event, EventType};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    println!("Listening for controller input...");

    loop {
        while let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    println!("Button pressed: {:?}", button);
                }
                EventType::ButtonReleased(button, _) => {
                    println!("Button released: {:?}", button);
                }
                EventType::ButtonChanged(button, value, _) => {
                    println!("button changed: {}", value);
                }
                EventType::AxisChanged(axis, value, _) => {
                    println!("Axis changed: {}", value);
                }
                _ => {}
            }
        }

        // sleep to avoid high CPU usage
        sleep(Duration::from_millis(10));
    }
}
