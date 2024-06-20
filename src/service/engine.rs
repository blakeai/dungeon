use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use csv::Reader;
use serde::{Deserialize, Serialize};
use crate::models::enums::weapon_proficiencies::WeaponProficiency;

#[derive(Debug, Serialize, Deserialize)]
struct Proficiencies {
    weapons: Vec<WeaponProficiency>,
    armor: String,
    saving_throws: String,
    skills: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Class {
    proficiencies: Proficiencies,
    levels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    subclasses: HashMap<String, HashMap<String, Class>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // let file_path = "path_to_your_csv_file.csv";
    // let mut rdr = Reader::from_path(file_path)?;
    // 
    // let headers = rdr.headers()?.clone();
    // let mut subclasses: HashMap<String, HashMap<String, Class>> = HashMap::new();
    // 
    // let mut current_class_name = String::new();
    // 
    // for result in rdr.records() {
    //     let record = result?;
    //     let class_name = record[0].trim().to_string();
    // 
    //     if class_name.starts_with("-") {
    //         let subclass_name = class_name.trim_start_matches("- ").to_string();
    // 
    //         let proficiency = Proficiencies {
    //             weapons: record[1].to_string(),
    //             armor: record[2].to_string(),
    //             saving_throws: record[3].to_string(),
    //             skills: record[4].to_string(),
    //         };
    // 
    //         let levels = record.iter().skip(5).map(|s| s.to_string()).collect();
    // 
    //         let class = Class {
    //             proficiencies: proficiency,
    //             levels,
    //         };
    // 
    //         if !subclasses.contains_key(&current_class_name) {
    //             subclasses.insert(current_class_name.clone(), HashMap::new());
    //         }
    // 
    //         if let Some(class_map) = subclasses.get_mut(&current_class_name) {
    //             class_map.insert(subclass_name, class);
    //         }
    //     } else {
    //         current_class_name = class_name.clone();
    // 
    //         let proficiency = Proficiencies {
    //             weapons: record[1].to_string(),
    //             armor: record[2].to_string(),
    //             saving_throws: record[3].to_string(),
    //             skills: record[4].to_string(),
    //         };
    // 
    //         let levels = record.iter().skip(5).map(|s| s.to_string()).collect();
    // 
    //         let class = Class {
    //             proficiencies: proficiency,
    //             levels,
    //         };
    // 
    //         subclasses.insert(current_class_name.clone(), HashMap::new());
    //         if let Some(class_map) = subclasses.get_mut(&current_class_name) {
    //             class_map.insert(current_class_name.clone(), class);
    //         }
    //     }
    // }
    // 
    // let root = Root { subclasses };
    // 
    // let json_output = serde_json::to_string_pretty(&root)?;
    // let output_file_path = "output.json";
    // let mut file = File::create(output_file_path)?;
    // serde_json::to_writer_pretty(&file, &root)?;
    // 
    // println!("JSON output written to {}", output_file_path);
    // 
    Ok(())
}
