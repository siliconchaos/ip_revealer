// use std::net::UdpSocket;
use reqwest;
use local_ip_address::local_ip;

fn get_wan_ip() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://api.ipify.org")?
        .text()?;
    Ok(resp)
}

fn get_local_ip() -> Result<String, Box<dyn std::error::Error>> {
    let ip = local_ip()?;
    Ok(ip.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wan_ip = get_wan_ip()?;
    let local_ip = get_local_ip()?;

    println!("External (WAN) IP: {}", wan_ip);
    println!("Local IP         : {}", local_ip);

    Ok(())
}