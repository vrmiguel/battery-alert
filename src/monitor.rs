use std::{thread, time::Duration};

use notify_rust::{Hint, Notification};

use crate::battery_reader::BatteryReader;

pub struct Monitor;

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_for_action(&self) -> bool {
        let mut action_taken = false;

        use crate::battery_reader::BatteryStatus::*;

        match BatteryReader::status() {
            Full => {
                self.send_notification("Battery's full!", "Stop charging!");
                action_taken = true;
            }
            Charging(charge) if charge >= 80 => {
                let msg = &*format!("Battery is at {}%!", charge);
                self.send_notification("Stop charging!", msg);
                action_taken = true;
            }
            Discharging(charge) | NotCharging(charge) if charge <= 40 => {
                let msg = &*format!("Battery is at {}%!", charge);
                self.send_notification("Start charging!", msg);
                action_taken = true;
            },
            _ => {}
        }

        action_taken
    }

    pub fn send_notification(&self, summary: &str, msg: &str) {
        println!("[LOG] Sending notification with summary \"{}\" and message \"{}\"", summary, msg);
        Notification::new()
            .summary(summary)
            .body(msg)
            .icon("battery")
            .appname("battery-alert")
            .hint(Hint::Category("device".into())) // this is not supported by all implementations
            .timeout(0)
            .show()
            .expect("failed to send notification");
    }

    pub fn start_loop(&self) -> ! {
        loop {
            let charge = BatteryReader::state_of_charge();
            println!("Charge: {}", charge);
            let action_was_taken = self.check_for_action();
            if action_was_taken {
                thread::sleep(Duration::from_secs(210));
            }
            thread::sleep(Duration::from_secs(90))
        }
    }
}
