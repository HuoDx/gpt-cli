use std::fs;
use std::io::ErrorKind;
use text_io::read;

use serde::{Deserialize, Serialize};

const CONFIG_DIR: &str = concat!(env!("HOME"), "/.gpt-cli");
const CONFIG_PATH: &str = concat!(env!("HOME"), "/.gpt-cli/config.json");
fn create_directory() {
    fs::create_dir_all(CONFIG_DIR).expect(&format!("Cannot create directory {}", CONFIG_DIR));
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub model_name: Option<String>,
    pub system_prompt: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        // let result = ;
        match fs::read_to_string(CONFIG_PATH) {
            Ok(result) => return serde_json::from_str::<Config>(&result)
            .expect("Cannot parse JSON config"),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    create_directory();
                }
                return Config {
                    api_key: None,
                    model_name: None,
                    system_prompt: None,
                };
            }
        }
        // if let Err(e) = result {
        //     println!("Error Kind: {}", e.kind());
            
        // } else {
        //     let config_string = result.unwrap();
        //     return serde_json::from_str::<Config>(&config_string)
        //         .expect("Cannot parse JSON config");
        // }
    }

    fn save(self) {
        let config_string =
            serde_json::to_string(&self).expect("Cannot encode config object to JSON");
        let result = fs::write(CONFIG_PATH, &config_string);
        if let Err(e) = result {
            if e.kind() == ErrorKind::NotFound {
                create_directory();
                fs::write(CONFIG_PATH, &config_string)
                    .expect("Cannot write file after creating the directory");
            }
        }
    }

    pub fn complete(mut self) {
        while self.api_key.is_none() {
            println!("Enter your OpenAI API Key: ");
            let key: String = read!("{}\n");
            if key.is_empty() {
                println!("Key cannot be empty! Retry.");
                continue;
            }
            self.api_key = Some(key);
        }

        while self.model_name.is_none() {
            println!("Enter the model name (default 'gpt-4'): ");
            let mut model: String = read!("{}\n");
            if model.is_empty() {
                model = String::from("gpt-4");
            }
            self.model_name = Some(model);
        }

        while self.system_prompt.is_none() {
            println!("System prompt (leave blank if you don't know what it does): ");
            let mut sys_prompt: String = read!("{}\n");
            if sys_prompt.is_empty() {
                sys_prompt = String::from(
                    "Behave like a plain UNIX bash; be as concise as possible! You can choose not to respond by saying '/NR'",
                );
            }
            self.system_prompt = Some(sys_prompt);
        }
        self.save();
    }
}

pub fn run(_: &[String]) {
    let config = Config::load();
    config.complete();
}
