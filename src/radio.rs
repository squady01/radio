use serde::{Deserialize, Serialize};
use std::{process::{Command, Stdio}, collections::HashMap, fs::File, io::{BufReader, Error, ErrorKind}};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Radio {
    pub name: String,
    pub stream_url: String,
}

impl Radio {
    pub fn play_radio(&self)
    {
        let mut process = Command::new("mpv")
                            .arg("--no-video")
                            .arg(&self.stream_url)
                            .stdout(Stdio::inherit())
                            .stdin(Stdio::piped())
                            .spawn()
                            .expect("Unable to launch mpv");
        
                            let _ = process.wait();
    }
    pub fn save_radios(filename: &str, radios: &HashMap<String, Radio>) -> Result<(), Error>
    {
        let file = std::fs::File::create(filename)?;
        serde_json::to_writer_pretty(file, &radios)?;

        Ok(())
    }
    pub fn add_radio(filename: &str, radios: &mut HashMap<String, Radio>, radio_to_add: &Radio) -> Result<(), Error>
    {
        radios.entry(radio_to_add.name.clone())
            .or_insert_with(|| radio_to_add.clone())
                .stream_url = radio_to_add.stream_url.clone();

        Radio::save_radios(filename, radios)
    }

    pub fn del_radio(filename: &str, radios: &mut HashMap<String, Radio>, radio_name: &str) -> Result<(), Error> {
        if radios.remove(radio_name).is_none() {
            return Err(Error::new(ErrorKind::NotFound, format!("Radio '{}' not found", radio_name)));
        }
    
        Radio::save_radios(filename, radios)
    }

    pub fn get_radio(radios: &HashMap<String, Radio>, radio_name: &str) -> Result<Radio, Error>
    {
        radios.get(radio_name)
            .cloned()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("Radio '{}' not found", radio_name)))
    }
    pub fn load_radios(file_name:&str) -> Result<HashMap<String, Radio>, Error>
    {
        let file = File::open(file_name).map_err(|err| Error::new(err.kind(), format!("Unable to open file: {}", err)))?;
        let reader = BufReader::new(file);
    
        serde_json::from_reader(reader)
            .map_err(|err| Error::new(ErrorKind::InvalidData, format!("Unable to load file, error in the Json format: {}", err)))
    }


}

 