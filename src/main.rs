extern crate config;
extern crate libnotify;
extern crate rodio;
use config::*;
use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

struct TransitionData {
    duration: Duration,
    message: String,
    sound: String,
    urgency: libnotify::Urgency,
}


#[derive(Copy,Clone)]
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
        &format!("{} min timer started", data.duration.as_secs() / 60),
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

    let long_break = settings.get_table("long_break").expect("no long_break table");
    let long_break = extract_table(long_break);

    let short_break = settings.get_table("short_break").expect("no short_break table");
    let short_break = extract_table(short_break);

    let begin_work = settings.get_table("begin_work").expect("no begin_work table");
    let begin_work = extract_table(begin_work);

    let flow_order = settings.get_array("order").expect("no order variable");

    let mut flow = Vec::<State>::new();

    for idx in 0..flow_order.len() {
        let temp = flow_order[idx].clone();
        let value = match temp.into_str() {
            Ok(i) => {i}
            Err(_) => {"not good".to_string()}
        };
        println!("Sequence {}:{}", idx, value);
        if "BeginWork" == &value {
                flow.push(State::BeginWork)
        }
        if "LongBreak" == &value {
                flow.push(State::LongBreak)
        }
        if "ShortBreak" == &value {
                flow.push(State::ShortBreak)
        }
    }


    let mut index: usize = 0;
    let mut current_state = flow[index];
    loop {
        println!("Playing idx: {}", index);
        match current_state {
            State::BeginWork => {
                let sound = begin_work.sound.clone();
                process_state(TransitionData {
                    urgency: libnotify::Urgency::Critical,
                    message: "Begin work".to_string(),
                    sound: sound,
                    duration: begin_work.duration
                });
                thread::sleep(begin_work.duration);
            }
            State::ShortBreak => {
                let sound = short_break.sound.clone();
                process_state(TransitionData {
                    urgency: libnotify::Urgency::Low,
                    message: "Take a break 😁".to_string(),
                    sound: sound,
                    duration: short_break.duration
                });
                thread::sleep(short_break.duration);
            }
            State::LongBreak => {
                let sound = long_break.sound.clone();
                process_state(TransitionData {
                    urgency: libnotify::Urgency::Low,
                    // emoji is a medidating person
                    message: "Take a looooong break 🧘🏼‍♂️".to_string(),
                    sound: sound,
                    duration: long_break.duration
                });
                thread::sleep(long_break.duration);
            }
        }

        index += 1;
        if index >= flow.len() {
            index = 0;
        }
        current_state = flow[index];
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
