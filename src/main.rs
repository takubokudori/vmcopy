// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use clap::{App, Arg, ArgMatches};
use hvctrl::hyperv::HyperVCmd;
use std::io::Write;

fn input(m: &ArgMatches, name: &str) -> String {
    match m.value_of(name) {
        Some(x) => x.to_string(),
        None => {
            print!("{}: ", name);
            std::io::stdout().flush().expect("Failed to flush stdout");
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_string()
        }
    }
}

fn input_vm_name(m: &ArgMatches, name: &str, cmd: &HyperVCmd) -> String {
    match m.value_of(name) {
        Some(x) => x.to_string(),
        None => {
            let vms: Vec<String> = cmd.list_vms().expect("Failed to list VMs").iter()
                .map(|vm| vm.name.as_ref().unwrap().to_owned()).collect();
            println!("id: name");
            println!("-------");
            vms.iter().enumerate().for_each(|(i, name)| println!("{}: {}", i, name));
            println!("-------");
            print!("{} id: ", name);
            std::io::stdout().flush().expect("Failed to flush stdout");
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let s = s.trim_end();
            match s.parse::<usize>() {
                Ok(x) => {
                    // ID
                    vms[x].clone()
                }
                Err(_) => {
                    // VMName
                    s.to_string()
                }
            }
        }
    }
}

fn main() {
    let m = App::new("VMCopy")
        .arg(Arg::with_name("VMName").short("n").long("VMName").takes_value(true))
        .arg(Arg::with_name("SourcePath").short("s").long("SourcePath").takes_value(true))
        .arg(Arg::with_name("DestinationPath").short("d").long("DestinationPath").takes_value(true))
        .get_matches();
    let cmd = hvctrl::hyperv::HyperVCmd::new();
    let vm_name = input_vm_name(&m, "VMName", &cmd);
    let src = input(&m, "SourcePath");
    let dst = input(&m, "DestinationPath");
    let cmd = cmd.vm(&vm_name);

    println!("\nVMName: {}", vm_name);
    println!("SourcePath: {}", src);
    println!("DestinationPath: {}", dst);
    {
        print!("\nOK?[Y/N] ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();
        if s.starts_with("N") || s.starts_with("n") {
            println!("aborted!");
            return;
        }
    }

    match cmd.copy_from_host_to_guest(&src, &dst, true, true) {
        Ok(_) => println!("success!"),
        Err(x) => println!("Failed to copy a file: {:?}", x),
    }
}
