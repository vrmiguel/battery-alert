use std::fs::read_to_string;

pub enum BatteryStatus {
    Unknown,
    Charging(u8),
    Discharging(u8),
    NotCharging(u8),
    Full,
}

pub struct BatteryReader;

impl BatteryReader {
    pub fn state_of_charge() -> u8 {
        let capacity = read_to_string("/sys/class/power_supply/BAT0/capacity")
            .expect("failed to read BAT0's capacity!");

        capacity.trim().parse().unwrap()
    }

    pub fn status() -> BatteryStatus {
        let capacity = &*read_to_string("/sys/class/power_supply/BAT0/status")
            .expect("failed to read BAT0's capacity!");

        let charge = Self::state_of_charge();

        match capacity.trim() {
            "Charging" => BatteryStatus::Charging(charge),
            "Discharging" => BatteryStatus::Discharging(charge),
            "Not charging" => BatteryStatus::NotCharging(charge),
            "Full" => BatteryStatus::Full,
            _ => BatteryStatus::Unknown,
        }
    }
}
