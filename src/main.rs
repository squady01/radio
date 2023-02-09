use std::{env};

use radio::radio::Radio;
use clap::{Args, Parser, Subcommand};

const RADIOS_FILE: &str = "radios.json";



#[derive(Debug, Subcommand)]
enum RadioArgs {
    /// Add a radio to the radio list
    Add(AddRadio),  
    /// Remove a radio from the radio list
    Del(DelRadio),  
    /// Play a radio
    Play(PlayRadio),
    /// Show the radio list
    List
}

#[derive(Debug, Args)]
pub struct AddRadio{
    /// name of the radio
    pub radio_name: String,
    /// stream url of the radio
    pub radio_stream: String
}
#[derive(Debug, Args)]
pub struct PlayRadio {
    /// name of the radio
    pub radio_name: String,
}
#[derive(Debug, Args)]
pub struct DelRadio {
    /// name of the radio
    pub radio_name: String,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, about="Please note: mpv must be installed on your system.")]
struct RadioCommand
{
    #[clap(subcommand)]
    pub first_arg: RadioArgs,
}

pub fn contruct_radios_path(file_name: &str) -> Result<String, &str>
{
    let mut file_path = env::current_exe().expect("Unable to find current exe path");
    file_path.pop();
    file_path.push(file_name);
    let file_path = file_path.to_str().expect("Unable to convert path to str format");

    Ok(file_path.to_owned())
}

fn main() {
    
    
    let radios_file = contruct_radios_path(RADIOS_FILE).unwrap();

    if let Ok(mut radios) = Radio::load_radios(&radios_file) {
        if let Ok(command) = RadioCommand::try_parse() {
            match command.first_arg {
                RadioArgs::Add(arg) => {
                    let radio = Radio { name: arg.radio_name, stream_url: arg.radio_stream };
                    match Radio::add_radio(&radios_file, &mut radios, &radio)
                    {
                        Ok(_) => println!("Radio {} added", radio.name),
                        Err(error) => println!("{}",error),
                    }
                },
                RadioArgs::Del(radio) => {
                    match  Radio::del_radio(&radios_file, &mut radios, &radio.radio_name)
                    {
                        Ok(_) => println!("Radio {} removed", &radio.radio_name),
                        Err(error) => println!("{}",error),
                    }
                }
                RadioArgs::Play(arg) => {
                    match Radio::get_radio(&radios, &arg.radio_name) 
                    {
                        Ok(radio) => radio.play_radio(),
                        Err(error) => println!("{error}")
                    }
                },
                RadioArgs::List => {
                    println!("found {} radio(s)", radios.len());
                    for (_, radio) in radios.iter() {
                        println!("[{:10} => {}]", radio.name, radio.stream_url);
                    }
                }
            }
        } else {
            println!("{}", RadioCommand::try_parse().err().unwrap());
        }
    } else {
        println!("{}", Radio::load_radios(RADIOS_FILE).err().unwrap());
    }
    

    
}

