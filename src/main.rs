use core::str;
use crossterm::style::Stylize;
use curl::easy::{Easy, List};
use human_bytes::human_bytes;
use sonic_rs::{JsonValueTrait, Value};
use std::{env, fs::File, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = String::new();
    File::open(format!(
        "{}/.config/get-traffic/config.json",
        env::var("HOME")?
    ))?
    .read_to_string(&mut config)?;
    let config: Value = sonic_rs::from_str(&config)?;

    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&format!(
        "http://{}:{}/traffic",
        config.get("host").as_str().unwrap(),
        config.get("port").as_u64().unwrap()
    ))?;
    let mut list = List::new();
    list.append(&format!(
        "Authorization: {}",
        config.get("token").as_str().unwrap()
    ))?;
    handle.http_headers(list)?;
    let mut transfer = handle.transfer();
    transfer.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
    })?;
    transfer.perform()?;
    drop(transfer);
    for name_and_traffic in
        unsafe { sonic_rs::to_object_iter_unchecked(str::from_utf8_unchecked(&data)) }
    {
        let (name, traffic) = name_and_traffic?;
        let traffic: Value = sonic_rs::from_str(traffic.as_raw_str())?;
        let upload = traffic.get("tx").as_u64().unwrap();
        let download = traffic.get("rx").as_u64().unwrap();
        println!(
            "{}Upload: {}Download: {}",
            format!("{:<10}", name.as_str()).blue(),
            format!("{:<10}", human_bytes(upload as f64)).red(),
            human_bytes(download as f64).green(),
        );
    }
    Ok(())
}
