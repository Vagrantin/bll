use dbus::blocking::Connection;
use dbus::{Message, arg::messageitem::MessageItemArray};
use std::{time::Duration, vec};

pub fn prendlebus() -> Result<(), Box<dyn std::error::Error>> {
    let timeout = Duration::new(2000,0);
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Create a message that calls the Notify method of the Notifications interface
    let mut m = Message::new_method_call(
        "org.freedesktop.Notifications", // destination
        "/org/freedesktop/Notifications", // path
        "org.freedesktop.Notifications", // interface
        "Notify" // method
        ).unwrap();
    
    // Append the required arguments to the message
    let notificationlevel: u32 = 0;
    let actions = MessageItemArray::new(vec![],"as".into()).unwrap();
    let hints = MessageItemArray::new(vec![],"a{sv}".into()).unwrap();
    let timingout = 5000;
    /* To not forget what it means
    m = Message::append3(m,"AppName",notificationlevel,"icon")
        .append3("summary", "body", actions)
        .append2(hints, timingout);
    */
    m = Message::append3(m,"bll",notificationlevel,"/usr/share/icons/ubuntu-mono-dark/status/24/unity-battery-020.svg")
        .append3("BATTERY", "Your Battery is running low !!", actions)
        .append2(hints, timingout);
        // Send the message to the bus and handle the possible errors
        match conn.channel().send_with_reply_and_block(m, timeout){
        //match conn.send_with_reply_and_block(m, 2000) {
        Ok(_) => println!("Notification sent successfully"),
        Err(e) => println!("Error: {}", e),
        }

    Ok(())
}