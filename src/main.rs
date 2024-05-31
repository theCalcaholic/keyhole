#![allow(non_snake_case)]

mod header;

use dioxus::prelude::*;
use log::LevelFilter;
use std::process::Command;
use dioxus::prelude::ServerFnError::Serialization;
use std::fs;


fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        { header::component() }
        input { r#type: "password",
            name: "password",
            placeholder: "enter your password"
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

struct CryptDisk {
    uuid: String,
    target: String,
}

#[server(Unlock)]
async fn unlock_disk() -> Result<String, ServerFnError> {
    let crypt_disks = fs::read_to_string("/conf/conf.d/cryptroot")
        .map_err(ServerFnError::new)?
        .split("\n")
        .map(|s| s.split_whitespace());
    
    crypt_disks.map(|disk| {
        if len(disk) != 

        match Command::new("cryptsetup").arg("open").spawn() {
            Ok(_) => Ok(String::from("Succesfully unlocked disk. Booting system...")),
            Err(e) => Err(ServerFnError::new(e))
        }
    });


}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
