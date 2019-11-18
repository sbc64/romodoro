use libnotify;
use std::thread::sleep;
use std::time::{Duration};

fn main() {
    libnotify::init("romodor").unwrap();
    const FIVE_MINUTES: u64 = 300;
    const TWENTY_MINUTES: u64 = 1200;
    let mut duration: Duration = Duration::from_secs(FIVE_MINUTES);
    let mut message = "Resting time";
    loop {
        sleep(duration);
        let n = libnotify::Notification::new(
            &format!("{} minutes", duration.as_secs()),
            Some(message),
            None,
        );
        n.set_urgency(libnotify::Urgency::Critical);
        n.show().unwrap();
        if duration.as_secs() == FIVE_MINUTES {
            duration = Duration::from_secs(TWENTY_MINUTES);
            message = "Time to work";
        } else {
            duration = Duration::from_secs(FIVE_MINUTES);
            message = "Begin rest";
        }
    }
}
