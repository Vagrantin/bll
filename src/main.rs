use std::fs;
use bll::prendlebus;

fn main() {

    let warning_value = 600000;
    let battery_level = "/sys/class/power_supply/BAT1/charge_now";
    let battery_status = "/sys/class/power_supply/BAT1/status";
    println!("Check from {}", battery_level);
    let current_battery_level = fs::read_to_string(battery_level)
        .expect("Should have been able to read the file")
        .as_str()
        .trim()
        .parse::<u32>()
        .unwrap();
    let current_battery_status:  String = fs::read_to_string(battery_status).expect("should be ok").trim().parse().unwrap();
    //println!("{:#?}",&current_battery_status);
    //println!("This is the content: {}",current_battery_level);

    // if current now is lower than warning_value
    // trigger dbus nofitication

    
    if current_battery_level <= warning_value {
        if current_battery_status == "Discharging" {
            println!("Battery is going to ie !! See {warning_value} > {current_battery_level}");
            let _ = prendlebus();
        } else {
            print!("We are charging all good!\n")
        }
    }
}