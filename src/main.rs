use std::{env, fs, error::Error};
use rand::seq::IteratorRandom;
use colored::Colorize;


fn generatename<'a>(keywords: &'a Vec<&str>, keywordcount: usize ) -> String {
    let mut rng = rand::thread_rng();
    let slug = keywords.into_iter().choose_multiple(&mut rng, keywordcount);
    let mut items = Vec::<&str>::with_capacity(keywordcount);
    for item in &slug {
        items.push(item);
    }
    items.join("-")
}


fn help() {
    println!("Usage:
    seo-rename <directory> <keyword,keyword..>");
}


fn main() -> Result<(), Box<dyn Error>>  {
    let extensions = "jpg,webp,png,jpeg".split(",").collect::<Vec<&str>>();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let dictionary = &args[2].replace(" ","");
        let keywords = dictionary.split(",").collect::<Vec<&str>>();
        println!("{} {}", "Dictionary:", keywords.join(" - ").green().bold());
        let current_dir = &args[1]; // let current_dir = env::current_dir()?;
        println!("{} {}", "Working directory:", current_dir.green().bold());
        for entry in fs::read_dir(current_dir)? {
            let mut x = 1;
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let extension = path.extension().unwrap().to_str().unwrap();

            if metadata.is_file() && extensions.contains(&extension) {
                let mut done = false;
                let mut keywordcount: usize = 2;
                print!("{} {}", path.file_name().unwrap().to_str().unwrap(), "=".red());
                while !done {
                    if x % 5 == 0 {
                        keywordcount = keywordcount + 1
                    }
                    let newfilename: String = format!("{}.{}", generatename(&keywords, keywordcount), path.extension().unwrap().to_str().unwrap());
                    let newpath = path.with_file_name(newfilename.to_lowercase());
                    if !newpath.exists() {
                        println!("{} {}", ">".red(), newfilename.to_lowercase().green());
                        fs::rename(path.to_str().unwrap(), newpath.to_str().unwrap())?;
                        done = true;
                    } else {
                        print!("{}", "=".red());
                        x = x + 1;
                    }
                }
            }
        }
    } else {
        help();
    }
    Ok(())
}
