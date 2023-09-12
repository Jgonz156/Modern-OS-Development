use std::io;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn main() {
    let force_stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let force_stop_child_reference: Arc<AtomicBool> = Arc::clone(&force_stop);
    let sleep_handle: JoinHandle<()> = thread::spawn(move || {
        let mut sleepin_count: u64 = 1;
        while !force_stop_child_reference.load(Ordering::Relaxed) {
            println!("{} = I'm sleepin' man", sleepin_count);
            thread::sleep(Duration::from_secs(5));
            sleepin_count += 1;
        }
    });
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    if input == String::from("wake up"){
        force_stop.store(true, Ordering::Relaxed);
    }
    sleep_handle.join().unwrap();
}
