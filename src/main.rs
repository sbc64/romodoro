extern crate config;
extern crate libnotify;
extern crate rodio;
use config::*;
use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct TransitionData {
    duration: Duration,
    message: String,
    sound: String,
    urgency: libnotify::Urgency,
}

enum State {
    BeginWork,
    ShortBreak,
    LongBreak,
}

fn playback(filename: String) {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let file = std::fs::File::open(filename).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}

fn process_state(data: TransitionData) {
    let urgency = data.urgency.clone();
    let n = libnotify::Notification::new(
        &format!("{} minutes done", data.duration.as_secs() / 60),
        Some(data.message.as_ref()),
        None,
    );
    thread::spawn(move || playback(data.sound));
    n.set_urgency(urgency);
    n.show().unwrap();
}

struct ConfigTable {
    sound: String,
    duration: Duration,
}

fn extract_table(table: HashMap<String, Value>) -> ConfigTable {
    let sound : String;
    match table.get("sound") {
        Some(entry) => {
            sound = entry.to_string()
            }
        None => {sound = "".to_string()}
    }

    let duration: u64;
    match table.get("duration") {
        Some(entry) => {
            duration = entry.to_string().parse::<u64>().expect("Not a number")
        }
        None => {duration = 0}
    }
    return ConfigTable {
        sound: sound,
        duration: Duration::from_secs(duration),
    }
}

fn run(settings: config::Config) {
    let mut current_state = State::BeginWork;

    let long_break = settings.get_table("long_break").expect("no long_break table");
    let long_break = extract_table(long_break);

    let short_break = settings.get_table("short_break").expect("no short_break table");
    let short_break = extract_table(short_break);

    let begin_work = settings.get_table("begin_work").expect("no begin_work table");
    let begin_work = extract_table(begin_work);


    loop {
        match current_state {
            State::BeginWork => {
                let sound = begin_work.sound.clone();
                process_state(TransitionData {
                    urgency: libnotify::Urgency::Critical,
                    message: "Begin work".to_string(),
                    sound: sound,
                    duration: begin_work.duration
                });
                sleep(begin_work.duration);
                current_state = State::ShortBreak;
            }
            State::ShortBreak => {
                let sound = short_break.sound.clone();
                process_state(TransitionData {
                    urgency: libnotify::Urgency::Low,
                    message: "Take a break 😁".to_string(),
                    sound: sound,
                    duration: short_break.duration
                });
                sleep(short_break.duration);
                current_state = State::LongBreak;
            }
            State::LongBreak => {
                let sound = long_break.sound.clone();
                process_state(TransitionData {
                    urgency: libnotify::Urgency::Low,
                    message: "Take a looooong break 😁".to_string(),
                    sound: sound,
                    duration: long_break.duration
                });
                sleep(long_break.duration);
                current_state = State::BeginWork;
            }
        }
    }
}

fn main() {
    libnotify::init("romodoro").unwrap();

    let mut config_path: String = env::var("XDG_CONFIG_HOME").unwrap();
    config_path.push_str("/romodoro");
    let mut settings = Config::default();
    settings
        .merge(config::File::with_name(config_path.as_ref()))
        .unwrap()
        .merge(config::Environment::with_prefix("ROMODORO"))
        .unwrap();

    run(settings);
}
