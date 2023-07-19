use std::collections::HashMap;
use std::fs;

pub fn load_env() -> HashMap<String, String>{
    let contents = fs::read_to_string(".env")
        .expect("Le fichier env est inexistant.");
    let lines : Vec<&str> = contents.split("\n").collect();
    let mut map = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    map
}