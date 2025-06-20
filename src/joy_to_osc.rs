use gilrs::{Gilrs, Event, EventType};
use std::time::Duration;
use std::thread::sleep;
use nannou_osc as osc;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let port = 1234;
    let target_addr = format!("{}:{}", "127.0.0.1", port);
    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");

    loop {
        while let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::ButtonChanged(button, value, _) => {
                    match button {
                        gilrs::Button::LeftTrigger2 => {
                            let osc_addr = "/playhead/l2".to_string();
                            let args = vec![osc::Type::Float(value)];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::RightTrigger2 => {
                            let osc_addr = "/playhead/r2".to_string();
                            let args = vec![osc::Type::Float(value)];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        _ => {}
                    }
                    // println!("button changed: {:?} {}", button, value);
                }
                _ => {}
            }
        }

        // sleep to avoid high CPU usage
        sleep(Duration::from_millis(10));
    }
}
