use gilrs::{Gilrs, Event, EventType};
// use std::time::Duration;
// use std::thread::sleep;
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
                EventType::ButtonPressed(button, _code) => {
                    // println!("button changed: {:?}", button);
                    match button {
                        gilrs::Button::West => {
                            let osc_addr = "/playhead/cue_jump".to_string();
                            let args = vec![osc::Type::String("West".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::North => {
                            let osc_addr = "/playhead/cue_jump".to_string();
                            let args = vec![osc::Type::String("North".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::East => {
                            let osc_addr = "/playhead/cue_jump".to_string();
                            let args = vec![osc::Type::String("East".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::South => {
                            let osc_addr = "/playhead/cue_jump".to_string();
                            let args = vec![osc::Type::String("South".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::DPadLeft => {
                            let osc_addr = "/playhead/cue_set".to_string();
                            let args = vec![osc::Type::String("West".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::DPadUp => {
                            let osc_addr = "/playhead/cue_set".to_string();
                            let args = vec![osc::Type::String("North".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::DPadRight => {
                            let osc_addr = "/playhead/cue_set".to_string();
                            let args = vec![osc::Type::String("East".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::DPadDown => {
                            let osc_addr = "/playhead/cue_set".to_string();
                            let args = vec![osc::Type::String("South".to_string())];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::RightTrigger => {
                            let osc_addr = "/playhead/fwd_lock".to_string();
                            let args = vec![];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::LeftTrigger => {
                            let osc_addr = "/playhead/rev_lock".to_string();
                            let args = vec![];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        _ => {}
                    }
                }
                EventType::ButtonChanged(button, value, _) => {
                    match button {
                        gilrs::Button::LeftTrigger2 => {
                            let osc_addr = "/playhead/rev".to_string();
                            let args = vec![osc::Type::Float(value)];
                            let packet = (osc_addr, args);
                            sender.send(packet).ok();
                        }
                        gilrs::Button::RightTrigger2 => {
                            let osc_addr = "/playhead/fwd".to_string();
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
        // sleep(Duration::from_millis(10));
    }
}
