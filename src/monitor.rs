use std::{thread, time::{Duration, Instant}};

use notify_rust::{Hint, Notification};

use crate::battery_reader::{BatteryReader, BatteryStatus};

pub type StatusHasChanged = bool;

pub struct Monitor {
    status: BatteryStatus,
    last_notified: Instant,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            status: BatteryStatus::Unknown,
            last_notified: Instant::now() - Duration::from_secs(2),
        }
    }

    #[inline]
    fn update(&mut self) -> StatusHasChanged {
        use std::mem;

        let last_status = mem::replace(&mut self.status, BatteryReader::status());
        
        mem::discriminant(&last_status) != mem::discriminant(&self.status)
    }

    #[inline]
    fn can_notify_charge(&self) -> bool {
        self.last_notified.elapsed() >= Duration::from_secs(5)
    }

    #[inline]
    fn notify_status_change(&self) {
        match self.status {
            BatteryStatus::Charging(_) => self.send_notification("Now charging", "Battery is now charging"),
            BatteryStatus::Discharging(_) => self.send_notification("Now discharging", "Battery is now discharging"),
            BatteryStatus::NotCharging(_) => self.send_notification("Not charging", "Battery is not charging"),
            BatteryStatus::Full => self.send_notification("Now full", "Battery is now full"),
            _ => {}
        }
    }

    pub fn update_and_notify(&mut self) {
        use crate::battery_reader::BatteryStatus::*;

        let status_kind_changed = self.update();

        if self.can_notify_charge() {
            match self.status {
                Full => {
                    self.send_notification("Battery's full!", "Stop charging!");
                }
                Charging(charge) if charge >= 80 => {
                    let msg = format!("Battery is at {charge}%!");
                    self.send_notification("Stop charging!", &msg);
                }
                Discharging(charge) | NotCharging(charge) if charge <= 40 => {
                    let msg = format!("Battery is at {}%!", charge);
                    self.send_notification("Start charging!", &msg);
                },
                _ => {}
            }

            self.last_notified = Instant::now();
        }

        if status_kind_changed {
            self.notify_status_change()
        }
    }

    pub fn send_notification(&self, summary: &str, msg: &str) {
        println!("[LOG] Sending notification with summary \"{}\" and message \"{}\"", summary, msg);
        Notification::new()
            .summary(summary)
            .body(msg)
            .icon("battery")
            .appname("battery-alert")
            .hint(Hint::Category("device".into())) // this is not supported by all implementations
            .timeout(Duration::from_secs(5))
            .show()
            .expect("failed to send notification");
    }

    pub fn start_loop(&mut self) -> ! {
        loop {
            self.update_and_notify();
            
            thread::sleep(Duration::from_secs(3))
        }
    }
}
