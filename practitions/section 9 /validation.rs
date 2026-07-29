// 1. Определи трейт Notifier
trait Notifier {
    fn send(&self, message: &str);
    fn send_with_time(&self, message: &str) {
        print!("Time is 12:00 ");
        self.send(message);
    }
}

// 2. Определи структуру Email и SMS
struct Email {
    address: String,
}

struct SMS {
    phone_number: String,
}

// 3. Реализуй трейт для них

impl Notifier for Email {
    fn send(&self, message: &str) {
        println!("Sended email on {}: {}", self.address, message);
    }
}

impl Notifier for SMS {
    fn send(&self, message: &str) {
        println!("Sended SMS on {}: {}", self.phone_number, message);
    }
}

// 4. Напиши функцию send_alert
fn send_alert<T: Notifier>(notifier: &T, message: &str) {
    print!("ALERT: ");
    notifier.send(message);
}

fn main() {
    let mail = Email {
        address: String::from("example@mail.com"),
    };
    send_alert(&mail, "test message");

    let phone = SMS {
        phone_number: String::from("+123456789"),
    };
    send_alert(&phone, "test message");

    mail.send_with_time("timed message");

    let notifications: Vec<Box<dyn Notifier>> = vec![Box::new(mail), Box::new(phone)];

    for n in notifications {
        n.send("Message in loop");
    }
}
