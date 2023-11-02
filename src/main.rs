use std::fs;
use std::path::Path;
use std::env;

fn main() {

    // target
    let args : Vec<String> = env::args().collect();
    let target : &str;
    if args.len() < 2 {
        target = ".";
    }else{
        target = &args[1];
    }
    

    // check if target exists
    if !Path::new(target).exists() {
        println!("{} does not exist", target);
        return;
    }
    // check if target is a directory
    if !Path::new(target).is_dir() {
        println!("{} is not a directory", target);
        return;
    }
    
    // read directory
    if let Ok(entries) = fs::read_dir(target){
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{:?}", entry.path());
            }
        }
    }
}
