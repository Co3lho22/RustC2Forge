pub fn help() {
    let commands = r#"

List of available Commands:
----------------------------------------------------------------------------------------

help                           :    Shows available Commands

exit                           :    To terminate session

keylog_on                                       :    To start keylogger

keylog_dump                                     :    To print keystrokes

keylog_off                                      :    To close keylogger and self destruct the logged file

"#;

    println!("{}", commands);
}
