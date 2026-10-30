fn main() {
    let f = some_fn;
    let mut notification_method: fn(&str) = email_notification;
    let p = |x: i32| x + 1;
    notification_method = sms_notification;
    notify_user(notification_method, "Alice"); // email_notification -> fn(&str)
    notify_user(sms_notification, "Alice");
}

fn some_fn(x: i32) {
    println!("{x}")
}

fn email_notification(user: &str) {
    println!("Sending email to {user}...");
}

fn sms_notification(user: &str) {
    println!("Sending SMS to {user}...")
}

fn notify_user(method: fn(&str), s: &str) {
    method(s);
}
