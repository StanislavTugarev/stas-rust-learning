use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    // Поток захватывает лок и паникует, не отпустив его "нормально"
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        // {
        let mut guard = data_clone.lock().unwrap();
        guard.push(4);
        // }
        panic!("паника во время владения локом");
    });

    // join вернёт Err, потому что поток запаниковал
    let _ = handle.join();

    println!("is_poisoned: {}", data.is_poisoned());

    // Теперь lock() возвращает Err(PoisonError)
    match data.lock() {
        Ok(guard) => println!("Ok: {:?}", *guard),
        Err(poisoned) => {
            println!("Err: {}", poisoned);
            // Данные всё ещё доступны через into_inner()
            let guard = poisoned.into_inner();
            println!("данные внутри: {:?}", *guard);
        }
    }

    // Снимаем отравление, после этого lock() снова возвращает Ok
    data.clear_poison();
    println!("после clear_poison: {:?}", *data.lock().unwrap());
}
