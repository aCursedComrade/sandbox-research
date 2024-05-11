#![allow(dead_code)]
use sandbox_research::ioctl;

fn main() {
    println!("[*] This binary is used to test the interaction with the driver");
    let args: Vec<_> = std::env::args().collect();

    if let Some(action) = args.get(1) {
        if action == "test" {
            if ioctl::echo("Hello from user-land!") {
                println!("[+] Test OK")
            } else {
                println!("[!] Test failed!")
            }
        } else if action == "read" {
            let list = ioctl::read_list();
            println!("[*] ReadList: {:?}", list);
        } else if action == "write" {
            if let Some(pid) = args.get(2) {
                if ioctl::write_list(pid.parse::<usize>().unwrap()) {
                    println!("[+] WriteList OK");
                } else {
                    println!("[!] WriteList failed!");
                }
            } else {
                println!("[!] Missing target PID");
            }
        } else {
            println!("[!] Nope 2x");
        }
    } else {
        println!("[!] Nope");
    }

    println!("[*] Done");
}
