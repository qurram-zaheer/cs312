use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::env;
use std::path::Path;

//Specifications: Counts LOC for .rs files only. Doesn't count lines with // comments. Block comment lines are counted
    
fn main() {
    let mut flag = true;
    while flag {
        println!("Enter the name of the file: "); 
        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
                        .expect("Failed to read input!");
        let fn_length = filename.chars().count();
        let file_type = &filename[fn_length-3..fn_length-1];
        if file_type != "rs" {
            println!("Unsupported file type! Only .rs files supported!, please try again");
            continue;
        }
        let cwd = env::current_dir().unwrap();
        let my_str = cwd.into_os_string().into_string().unwrap();
        let temp = ["/",filename.trim()].join("");
        let result = [my_str,temp].join("");
        if Path::new(&result).exists(){
            let file = File::open(filename.trim()).unwrap();
            let reader = BufReader::new(file);
            let mut counter = 0;
            
            for (_index, line) in reader.lines().enumerate() {
                let line = line.unwrap(); 
                let string = line.trim();
                if !string.is_empty() && string != "\n"{
                    counter = counter + 1;
                }
                counter = comment_remover(counter, string);
                
            }
            println!("LOC of {} is: {}", filename, counter);
            flag = false;
        } else {
            println!("No such file or directory, try again")
        }
    }
}

fn comment_remover(mut line_count: i32,line: &str) -> i32{
    let mut flag = 0;
    
    for (i,c) in line.chars().enumerate() {
        if i == 0 && c == '/' {
            flag = flag + 1;
        }
        if i == 1 && c == '/' {
            flag = flag + 1;
        }
    }
    if flag == 2 {

        line_count = line_count - 1;
    }
    line_count
}


