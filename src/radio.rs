use serde::{Deserialize, Serialize};
use std::{process::{Command, Stdio}, collections::HashMap, fs::File, io::BufReader};



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
    pub fn save_radios(filename: &str, radios: &HashMap<String, Radio>) -> Result<bool, String>
    {
        if let Ok(file) = std::fs::File::create(filename)
        {
            return match serde_json::to_writer_pretty(&file, &radios)
            {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
        Ok(true)
    }
    pub fn add_radio(filename: &str, radios: &mut HashMap<String, Radio>, radio_to_add: &Radio) -> Result<bool, String>
    {
        match radios.get_mut(&radio_to_add.name)
        {
            Some(radio) => radio.stream_url = radio_to_add.stream_url.clone(),
            None => {
                radios.insert(radio_to_add.name.clone(), radio_to_add.clone());
            }
        }

        Radio::save_radios(filename, radios)
    }

    pub fn del_radio(filename: &str, radios: &mut HashMap<String, Radio>, radio_name: &str) -> Result<bool, String>
    {
       
        if let None = radios.remove(radio_name)
        {
            return Err(format!("Radio '{}' not found", radio_name));
        }

        Radio::save_radios(filename, radios)
    }

    pub fn get_radio(radios: &HashMap<String, Radio>, radio_name: &str) -> Result<Radio, String>
    {
        radios.get(radio_name).cloned().ok_or(format!("Radio {radio_name} not found").to_string())
    }
    pub fn load_radios(file_name:&str) -> Result<HashMap<String, Radio>, String>
    {
        match File::open(file_name)
        {
            Ok(file) => 
            {
                let reader = BufReader::new(file);
                match serde_json::from_reader::<_, HashMap<String, Radio>>(reader)
                {
                    Ok(radi) => Ok(radi),
                    Err(_) => Err("Unable to load file, error in the Json format".to_string()),
                }
            },
            Err(_) => Ok(HashMap::new())
        }
    }


}

 