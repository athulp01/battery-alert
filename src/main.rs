use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const NOTIFY_EXEC: &str = "/usr/bin/notify-send";
const INTERVAL_SEC: Duration = Duration::from_secs(60);

fn send_notif(msg: &str) {
    let out = Command::new(NOTIFY_EXEC)
        .arg(msg)
        .args(&["-u", "critical"])
        .output()
        .expect("Cannot run notify-send binary");
    io::stderr().write_all(&out.stdout).unwrap();
    io::stderr().write_all(&out.stderr).unwrap();
}

fn monitor() {
    let mut alarm_10_per = false;
    let mut alarm_20_per = false;

    loop {
        let status = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .expect("Cannot read battery status file");
        let charge_rem: u8 = status.trim().parse().unwrap();

        if charge_rem <= 20 && alarm_20_per == false {
            send_notif("Battery Low ! 20% remaining");
            alarm_20_per = true;
        } else if charge_rem <= 10 && alarm_10_per == false {
            send_notif("Battery critically low!! 10% remaining");
            alarm_10_per = true;
        }

        if charge_rem > 10 { alarm_10_per = false; }
        if charge_rem > 20 { alarm_20_per = false; }

        sleep(INTERVAL_SEC);
    }
}

fn main() {
    monitor();
}
