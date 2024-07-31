use std::time::Instant;

use nannou::prelude::*;

pub mod spring;
use spring::Spring;

const WAIT_CUTOFF: f64 = 2.0;

#[derive(Debug, PartialEq)]
pub enum TapTempoState {
    NoTempoSet,
    InitialTap,
    RecordingTaps,
    TempoSet,
}

pub struct Model {
    state: TapTempoState,
    pub taps: Vec<Instant>,
    pub seconds_since_last_tap: Option<f64>,
    pub bpm: f64,
    spring: Spring,
}

impl Model {
    pub fn default() -> Model {
        Model {
            state: TapTempoState::NoTempoSet,
            taps: Vec::new(),
            seconds_since_last_tap: None,
            bpm: 1.0,
            spring: Spring::new(9.0, 0.3, 0.99, 0.0, 0.0),
        }
    }

    fn calculate_bpm(&mut self) -> Option<f64> {
        if self.taps.len() < 2 {
            return None;
        }

        let mut intervals = Vec::new();
        for i in 1..self.taps.len() {
            let duration = self.taps[i].duration_since(self.taps[i - 1]);
            intervals.push(duration.as_secs_f64());
        }

        let average_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;
        Some(60.0 / average_interval)
    }

    fn timeout(&mut self) {
        self.state = TapTempoState::NoTempoSet;
        println!("tap timeout");
    }

    fn set_bpm(&mut self, bpm: f64) {
        self.state = TapTempoState::TempoSet;
        self.bpm = bpm;
        println!("bpm: {}", bpm);
    }

    pub fn update(&mut self) {
        self.spring.update(0.05);
        if let Some(last_tap) = self.taps.last() {
            let current_time = Instant::now();
            self.seconds_since_last_tap =
                Some(current_time.duration_since(*last_tap).as_secs_f64());

            if self.taps.len() >= 2 {
                let mut intervals = Vec::new();
                for i in 1..self.taps.len() {
                    let duration = self.taps[i].duration_since(self.taps[i - 1]);
                    intervals.push(duration.as_secs_f64());
                }
                let average_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;

                if self.seconds_since_last_tap >= Some(average_interval * 2.0) {
                    if let Some(bpm) = self.calculate_bpm() {
                        self.set_bpm(bpm);
                        return;
                    } else {
                        self.timeout();
                        return;
                    }
                }
            } else {
                if self.seconds_since_last_tap >= Some(WAIT_CUTOFF) {
                    self.timeout();
                    return;
                }
            }
        }
    }

    fn set_initial_time(&mut self) {
        self.state = TapTempoState::InitialTap;
        self.taps.clear();
        println!("{:?}", self.state);
        self.taps.push(Instant::now());
    }

    fn set_additional_time(&mut self) {
        self.taps.push(Instant::now());
        println!("tap");
    }

    pub fn tap(&mut self) {
        match self.state {
            TapTempoState::NoTempoSet => self.set_initial_time(),
            TapTempoState::InitialTap => {
                self.state = TapTempoState::RecordingTaps;
                self.spring.value = 0.5;
                self.set_additional_time();
            }
            TapTempoState::RecordingTaps => {
                self.spring.value = 0.5;
                self.set_additional_time()
            }
            TapTempoState::TempoSet => self.set_initial_time(),
        }
    }

    pub fn clear(&mut self) {
        self.state = TapTempoState::NoTempoSet;
        println!("{:?}", self.state);
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    match model.state {
        TapTempoState::InitialTap => model.update(),
        TapTempoState::RecordingTaps => model.update(),
        _ => {}
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();
    let x = boundary.left() + 30.0;
    let y = boundary.top() - 30.0;
    let radius = 6.0;

    let size = radius*2.0*3.0;

    match model.state {
        TapTempoState::InitialTap => {
            let time_left_normalzied =
                ((WAIT_CUTOFF) - model.seconds_since_last_tap.unwrap_or(0.0)) / WAIT_CUTOFF;
            let diminished_radius = radius * time_left_normalzied as f32;
            draw.quad()
                .rgba(0.0, 0.0, 0.0, 1.0)
                .x_y(x, y)
                .width(size)
                .height(size);
            draw.ellipse()
                .rgba(1.0, 1.0, 1.0, 0.3)
                .radius(diminished_radius)
                .x_y(x, y);
        }
        TapTempoState::RecordingTaps => {
            let scale = 1.0 + model.spring.value;
            draw.quad()
                .rgba(0.0, 0.0, 0.0, 1.0)
                .x_y(x, y)
                .width(size)
                .height(size);
            draw.ellipse()
                .rgb(1.0, 0.0, 0.0)
                .radius(radius * scale)
                .x_y(x, y);
        }
        _ => {}
    }
    draw.to_frame(app, &frame).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_no_tempo() {
        let model = Model::default();
        assert_eq!(model.state, TapTempoState::NoTempoSet);
    }

    #[test]
    fn tapping_changes_state() {
        let mut model = Model::default();
        model.tap();
        assert_eq!(model.state, TapTempoState::InitialTap);
    }

    #[test]
    fn tapping_twice_changes_state_again() {
        // after two taps, the state is changed to indicate we're recording
        let mut model = Model::default();
        model.tap();
        model.tap();
        assert_eq!(model.state, TapTempoState::RecordingTaps);
    }

    #[test]
    fn calculates_bpm_correctly() {
        // BEWARE: this test has a delay in it
        //
        // this method is not exact, so we test to make sure the difference between the expected
        // value and actual value is small

        // 500ms between taps should be 120bpm
        let expected_bpm = 120.0;

        let mut model = Model::default();
        model.tap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        model.tap();
        std::thread::sleep(std::time::Duration::from_millis(1100));
        model.update();

        let actual_bpm = model.bpm;
        let diff = (expected_bpm - actual_bpm).abs();
        assert!(diff < 1.0);
    }

    #[test]
    fn is_cancelable() {
        let mut model = Model::default();
        model.tap();
        model.clear();
        assert_eq!(model.state, TapTempoState::NoTempoSet);
    }
}
