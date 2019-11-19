extern crate rodio;

use libnotify;
use std::io::BufReader;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const FIVE_MINUTES: u64 = 5;
const TWENTY_MINUTES: u64 = 20;
const BOJACK: &'static str = "/home/sebas/repos/programs/romodoro/src/bojack.mp3";
const BOWL: &'static str = "/home/sebas/repos/programs/romodoro/src/bowl.mp3";

#[derive(Debug, Copy, Clone)]
struct TransitionData {
    duration: Duration,
    message: &'static str,
    sound: &'static str,
    urgency: libnotify::Urgency,
}

enum State {
    BeginWork,
    ShortBreak,
    LongBreak,
}

fn playback(filename: &'static str) {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let file = std::fs::File::open(filename).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}

fn process_state(data: TransitionData) {
    let n = libnotify::Notification::new(
        &format!("{} minutes", data.duration.as_secs()),
        Some(data.message),
        None,
    );
    thread::spawn(move || playback(data.sound));
    n.set_urgency(data.urgency);
    n.show().unwrap();
    sleep(data.duration);
}

fn main() {
    libnotify::init("romodor").unwrap();
    let mut current_state = State::BeginWork;
    loop {
        match current_state {
            State::BeginWork => {
                process_state(TransitionData {
                    duration: Duration::from_secs(TWENTY_MINUTES),
                    message: "Begin work",
                    sound: BOJACK,
                    urgency: libnotify::Urgency::Critical,
                });
                current_state = State::ShortBreak;
            },
            State::ShortBreak => {
                process_state(TransitionData {
                    duration: Duration::from_secs(FIVE_MINUTES),
                    message: "Take a break 😁",
                    sound: BOWL,
                    urgency: libnotify::Urgency::Low,
                });
                current_state = State::BeginWork;
            }
            State::LongBreak => println!("hi"),
        }
    }
}
