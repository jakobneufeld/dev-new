
use clap::{App, Arg, SubCommand};
use std::fs::*;
use std::path::*;
use clipboard::osx_clipboard::OSXClipboardContext;
use toml::from_str;
use std::io::Read;
use std::io::Write;
use toml::Value;
use dirs;
use duct::cmd;
use std::process::exit;
use clipboard::ClipboardProvider;
use dialoguer::Select;
fn main() {
    let clap_app = App::new("dev-new")
            .about("Creates a new dev app")
        
            .arg(Arg::with_name("name").help("The name of the project").required(true))
            
            .get_matches();

    
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config/dev-new.toml");
    let mut config_file = File::open(config_path).expect("No config file");
    let mut config_file_contents = String::new();
     config_file.read_to_string(& mut config_file_contents).unwrap();
     let config_toml: toml::Value = toml::from_str(config_file_contents.as_str()).expect("Bad toml");
     let mut projects = PathBuf::new();
     let projects_path = config_toml["projects"].as_str().unwrap();
     projects.push(projects_path);
    // Now we can generate the project
    
    let mut  current_project_dir = PathBuf::new();
    current_project_dir.push(projects_path);
    current_project_dir.push(clap_app.value_of("name").unwrap());
    
    // Now lets ask the user which type of object he wants
    let mut project_type_select = Select::new();
    project_type_select.items(&["Rust", "Vue js", "Custom"]);


    println!("Choose what type of project you want to continue");

    let choice = project_type_select.interact();
    match choice.unwrap() {
        0 => {
            generate_rust_project(&current_project_dir,clap_app.value_of("name").unwrap() )
        },
        1=> { generate_vue_project(&current_project_dir, clap_app.value_of("name").unwrap())},
        2=> {custom(&current_project_dir, clap_app.value_of("name").unwrap(), config_toml)}
        _ => {panic!("Critical Error")}
    }
    println!("Perfect. Copy the clipboard contents to your shell to get started");


}

fn custom(dir: &PathBuf, name: &str, toml: toml::Value) {
    l

}
fn generate_rust_project(dir: &PathBuf, name: &str ) {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config/dev-new.toml");
    let mut config_file = File::open(config_path).expect("No config file");
    let mut config_file_contents = String::new();
     config_file.read_to_string(& mut config_file_contents).unwrap();
     let config_toml: toml::Value = toml::from_str(config_file_contents.as_str()).expect("Bad toml");
     let mut projects = PathBuf::new();
     let projects_path = config_toml["projects"].as_str().unwrap();
     projects.push(projects_path);
    println!("Preparing rust project");
    let what_to_do = format!("cd {} && cargo new {} && cd {} ", projects_path , name, dir.as_path().display());
    println!("You should do this, or just paste from the clipboard {}",  what_to_do);
    OSXClipboardContext::new().unwrap().set_contents(what_to_do).expect("Can  not write to clipboard");
}

fn generate_vue_project(dir: &PathBuf, name: &str ) {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config/dev-new.toml");
    let mut config_file = File::open(config_path).expect("No config file");
    let mut config_file_contents = String::new();
     config_file.read_to_string(& mut config_file_contents).unwrap();
     let config_toml: toml::Value = toml::from_str(config_file_contents.as_str()).expect("Bad toml");
     let mut projects = PathBuf::new();
     let projects_path = config_toml["projects"].as_str().unwrap();
     projects.push(projects_path);
    println!("Preparing vue project");
    let what_to_do = format!("cd {} && vue create {} && cd {} ", projects_path , name, dir.as_path().display());
    println!("You should do this, or just paste from the clipboard {}",  what_to_do);
    OSXClipboardContext::new().unwrap().set_contents(what_to_do).expect("Can  not write to clipboard");
}
