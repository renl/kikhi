extern crate regex;

use std::thread::spawn;
use regex::Regex;
use std::env::args;
use std::process::Command;

#[cfg(test)]
mod kikhi_tests {
    #[test]
    fn test_find_wireless() {
        let w = super::find_wireless();
        assert!(w == "wlp3s0");
    }

    #[test]
    fn test_find_wired() {
        let w = super::find_wired();
        assert!(w == "enp2s0");
    }
}

fn main() {
    run_and_print("clear", &[]);
    let args = args()
        .skip(1);
    let mut children = vec![];
    for arg in args {
        children.push(match &arg as &str {
            "ls" => spawn(|| {ls()}),
            "arpw" => spawn(|| {arp("w")}),
            "arpe" => spawn(|| {arp("e")}),
            "ifc" => spawn(|| {ifconfig()}),
            t => {
                let s = String::from(t);
                spawn(move || {println!("unused input: {}", s)})
            },
        })
    }
    for child in children {
        let _ = child.join();
    }
}

fn run_and_print(cmd: &str, args: &[&str]) {
    let s = run_process(cmd, args);
    println!("____________________________________________________________________________");
    println!("Standard output for cmd: {} with arguments: {:?}", cmd, args);
    println!("----------------------------------------------------------------------------");
    println!("{}", s);
}

fn run_process(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(output).unwrap()
}

fn ifconfig() {
    run_and_print("ifconfig", &[]);
}

fn find_wireless() -> String {
    find_eth_interfaces()
        .into_iter()
        .filter(|i| is_wireless_interface(i))
        .nth(0)
        .unwrap()
}

fn find_wired() -> String {
    find_eth_interfaces()
        .into_iter()
        .filter(|i| !is_wireless_interface(i))
        .nth(0)
        .unwrap()
}

fn arp (t: &str) {
    match t {
        "w" => {
            let port = find_wireless();
            run_and_print("arp-scan", &["-I", port.as_str(), "-l"])
        },
        "e" => {
            let port = find_wired();
            run_and_print("arp-scan", &["-I", port.as_str(), "-l"])
        },
        _ => (),
    }
}

fn is_wireless_interface(interface: &str) -> bool {
    let output = run_process("iwconfig", &[interface]);
    let re = Regex::new(r"^(\w+)\s+IEEE 802").unwrap();
    match re.captures(output.as_str())
    {
        Some(_) => true,
        _ => false,
    }
}

fn find_eth_interfaces() -> Vec<String> {
    let output = run_process("ifconfig", &[]);
    let re = Regex::new(r"(\w+)\s+Link encap:Ethernet").unwrap();
    re.captures_iter(output.as_str())
        .map(|c| {
            String::from(c.get(1)
                         .unwrap()
                         .as_str())
        })
        .collect()
}

fn ls() {
    run_and_print("ls", &["-lth"]);
}
