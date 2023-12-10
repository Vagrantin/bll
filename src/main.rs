//use std::env;
use std::fs;
use bll::prendlebus;

fn main() {
    let warning_value = 400000;
    /*
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    */
    let file_path = "/sys/class/power_supply/BAT1/current_now";
    println!("In File {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let value = contents.as_str();
    println!("This is the content: {}",contents);
    let current_now = value.trim().parse::<u32>().unwrap();
    // if current now is lower than warning_value
    // trigger dbus nofitication

    if current_now <= warning_value {
        //println!("Battery is going to die !! See > {current_now}");
        let _ = prendlebus();
    }
}
