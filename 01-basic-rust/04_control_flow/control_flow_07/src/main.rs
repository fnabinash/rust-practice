// Create a program that simulates a basic traffic light system.

fn main() {
    loop {
        println!("🔴 Please stop for while.");
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        println!("🟢 Thanks for your patient. You can go now.");
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        println!("🟡 Please slow down.");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
