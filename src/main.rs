extern crate rand;
use std::{env, fs, error::Error};
use rand::seq::IteratorRandom;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn generatename<'a>(keywords: &'a Vec<&str>) -> String {
    let mut rng = rand::thread_rng();
    let slug = keywords.into_iter().choose_multiple(&mut rng, 3);
    let mut items = Vec::<&str>::with_capacity(3);
    for item in &slug {
        items.push(item);
    }
    // print_type_of(&items);
    items.join("-")
}

fn help() {
    println!("Usage:
    seorename <directory> <keyword,keyword..>");
}


fn main() -> Result<(), Box<dyn Error>>  {

    let extensions = "jpg,webp,png,jpeg".split(",").collect::<Vec<&str>>();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        // let dictionary = "star,galaxy,battle,wars,r2d2,battlestar,obi-one,ciubecca,luke,leila".split(",");
        let dictionary = &args[2].replace(" ","");
        let keywords = dictionary.split(",").collect::<Vec<&str>>();
        println!("Dictionary: {}", keywords.join(" - "));
        let current_dir = &args[1]; // let current_dir = env::current_dir()?;
        println!("Working directory: {}", current_dir);
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let extension = path.extension().unwrap().to_str().unwrap();

            if metadata.is_file() && extensions.contains(&extension) {
                let mut done = false;
                while !done {
                    let newfilename: String = format!("{}.{}", generatename(&keywords), path.extension().unwrap().to_str().unwrap());
                    // let newfilename: String = path.file_name().unwrap().to_str().unwrap().into();
                    let newpath = path.with_file_name(newfilename.to_lowercase());
                    if !newpath.exists() {
                        println!("{} => {}", path.to_str().unwrap(), newpath.to_str().unwrap());
                        fs::rename(path.to_str().unwrap(), newpath.to_str().unwrap())?;
                        done = true;
                    } else {
                        println!("New iteration")
                    }
                }
            }
        }
    } else {
        help();
    }
    Ok(())
}
