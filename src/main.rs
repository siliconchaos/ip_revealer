use std::error::Error;
use reqwest;
use local_ip_address::local_ip;
use clap::{Command, Arg, ArgAction};

fn get_wan_ip() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://api.ipify.org")?
        .text()?;
    Ok(resp)
}

fn get_local_ip() -> Result<String, Box<dyn Error>> {
    let ip = local_ip()?;
    Ok(ip.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("IP Revealer")
        .version("0.2.0")
        .author("Robert Wallace")
        .about("Reveals your external (WAN) and local IP addresses")
        .arg(Arg::new("wan")
            .long("wan")
            .help("Show WAN IP only")
            .action(ArgAction::SetTrue)
            .conflicts_with("local"))
        .arg(Arg::new("local")
            .long("local")
            .help("Show local IP only")
            .action(ArgAction::SetTrue)
            .conflicts_with("wan"))
        .arg(Arg::new("raw")
            .long("raw")
            .action(ArgAction::SetTrue)
            .help("Show raw IP addresses only"))
        .get_matches();

    let wan = matches.get_flag("wan");
    let local = matches.get_flag("local");
    let raw = matches.get_flag("raw");

    if !wan && !local {
        // if netiher wan or local is specified, show both
        let wan_ip = get_wan_ip()?;
        let local_ip = get_local_ip()?;

        if raw {
            println!("{}\n{}", wan_ip, local_ip);
        } else {
            println!("External (WAN) IP: {}", wan_ip);
            println!("Local IP         : {}", local_ip);
        }
    } else {
        if wan {
            let wan_ip = get_wan_ip()?;
            if raw {
                println!("{}", wan_ip);
            } else {
                println!("External (WAN) IP: {}", wan_ip);
            }
        }
        if local {
            let local_ip = get_local_ip()?;
            if raw {
                println!("{}", local_ip);
            } else {
                println!("Local IP: {}", local_ip);
            }
        }
    }

    Ok(())
}