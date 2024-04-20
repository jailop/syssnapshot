use std::cmp::Ordering;
use sysinfo::{System, Disks};
use pnet::datalink;
use pnet::ipnetwork::IpNetwork;

pub mod utils;
pub mod performance;

fn show_system_info() {
    println!("System name   : {}", System::name().unwrap());
    println!("Kernel version: {}", System::kernel_version().unwrap());
    println!("Hostname      : {:?}", System::host_name().unwrap());
}

fn show_cpu_usage() {
    let mut sys = System::new();
    sys.refresh_cpu();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();
    let mut accum = 0.0;
    for (idx, cpu) in sys.cpus().iter().enumerate() {
        let usage = cpu.cpu_usage();
        accum += usage;
        println!("CPU{:2}  : {:5.1}%", idx, usage);
    }
    println!("Average: {:5.1}%", accum / sys.cpus().len() as f32);
}

fn show_memory_usage() {
    let mut sys = System::new();
    sys.refresh_memory();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let used_memory_pct = used_memory as f32 / total_memory as f32 * 100.0;
    println!("Total memory   : {}", utils::format_bytes(total_memory));
    println!("Used memory    : {}", utils::format_bytes(used_memory));
    println!("Used memory (%): {:.1}%", used_memory_pct);
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let used_swap_pct = used_swap as f32 / total_swap as f32 * 100.0;
    println!("Total swap     : {}", utils::format_bytes(total_swap));
    println!("Used swap      : {}", utils::format_bytes(used_swap));
    println!("Used swap (%)  : {:.1}%", used_swap_pct);
}

fn show_disk_usage() {
    let disks = Disks::new_with_refreshed_list();
    println!(
        "{:<20} {:<20} {:<6} {:<10} {:<12} {:8}",
        "Name", "Mount", "Format", "Total", "Usage", "Usage (%)"
    );
    for disk in &disks {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        println!(
            "{:<20} {:<20} {:<6} {:<10} {:>12} {:.1}%",
            disk.name().to_string_lossy(),
            disk.mount_point().to_string_lossy(),
            disk.file_system().to_string_lossy(),
            utils::format_bytes(total_space),
            utils::format_bytes(available_space),
            available_space as f32 / total_space as f32 * 100.0,
        );
    }
}

fn show_network_usage() {
    let ifaces = datalink::interfaces();
    println!("{:12} {}", "Name", "Addresses");
    for iface in ifaces {
        let mut ips: Vec<String> = Vec::new();
        for ip in iface.ips {
            match ip {
                IpNetwork::V4(ipv4) => {
                    ips.push(ipv4.ip().to_string());
                },
                IpNetwork::V6(ipv6) => {
                    ips.push(ipv6.ip().to_string());
                }
            }
        }
        println!("{:12} {}", iface.name, ips.join(", "));
    }
}

/*
fn show_network_usage() {
    let networks = Networks::new_with_refreshed_list();
    println!(
        "{:<10} {:14} {:>18} {:>18}", 
        "Name", "MAC", "Received", "Transmited"
    );
    for (iface, network) in &networks {
        println!(
            "{:<10} {:14} {:>18} {:>18}", 
            iface,
            network.mac_address().to_string(),
            format_bytes(network.total_received()),
            format_bytes(network.total_transmitted()),
        );
    }
}
*/

struct Process {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
}

impl Clone for Process {
    fn clone(&self) -> Process {
        Process {
            pid: self.pid,
            name: self.name.clone(),
            cpu_usage: self.cpu_usage,
            memory_usage: self.memory_usage,
        }
    }
}

fn top_processes(n: usize) -> Vec<Process> {
    let mut sys = System::new();
    sys.refresh_processes();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_processes();
    let mut processes: Vec<Process> = sys
        .processes()
        .iter()
        .map(|(_, process)| {
            Process {
                pid: process.pid().as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
            }
        })
        .collect();
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(Ordering::Equal));
    let mut top_processes: Vec<Process> = processes.iter()
        .take(n)
        .cloned()
        .collect();
    top_processes.sort_by(|a, b| b.memory_usage.partial_cmp(&a.memory_usage).unwrap_or(Ordering::Equal));
    top_processes
}

fn show_top_processes() {
    println!(
        "{:>8} {:<40} {:>10} {:<10}",
        "PID", "Name", "CPU (%)", "Memory"
    );
    for process in top_processes(10) {
        println!(
            "{:8} {:<40} {:9.1}% {:>10}",
            process.pid,
            process.name,
            process.cpu_usage,
            utils::format_bytes(process.memory_usage)
        );
    }
}

fn main() {
    show_system_info();
    println!("{}", "\nCPU USAGE:\n");
    show_cpu_usage();
    println!("{}", "\nMEMORY USAGE:\n");
    show_memory_usage();
    println!("{}", "\nTOP PROCESSES:\n");
    show_top_processes();
    println!("{}", "\nDISK USAGE:\n");
    show_disk_usage();
    println!("");
    let _ = performance::disk();
    println!("{}", "\nNETWORK INTERFACES:\n");
    show_network_usage();
}
