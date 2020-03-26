use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::env;
use std::path::Path;

//Specifications: Counts LOC for .rs files only. Doesn't count lines with // comments. Block comment lines are counted
//SPECIFICATION 2: CRITICAL: DO NOT USE THE SUBSTRING {RUST FUNCTION DECLARATOR} IN ANY VARIABLES OR COMMENTS FOR ACCURATE RESULTS
//Specification 3: Output maps a number to every function, prints out the mapping legend, shows function LOCs. Function LOC + import LOC = total LOC
fn main() {
    let mut flag = true;
    let mut func_count = vec![0];
    while flag {
        println!("Enter the name of the file: "); 
        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
                        .expect("Failed to read input!");
        let fun_length = filename.chars().count();
        let file_type = &filename[fun_length-3..fun_length-1];
        if file_type != "rs" {
            println!("Unsupported file type! Only .rs files supported!, please try again");
            continue;
        }
        let cwd = env::current_dir().unwrap();
        let my_str = cwd.into_os_string().into_string().unwrap();
        let temp = ["/",filename.trim()].join("");
        let result = [my_str,temp].join("");
        println!("LEGEND ------------");
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
                func_count = function_counter(line, func_count);
                
            }
            println!("---------------------");
            for index in  1..(func_count.len()){
                println!("LOC of function {} is {}",index, func_count[index]);
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

fn function_counter(line: String, mut func_count: Vec<i32>) -> Vec<i32> {
    let mut flag = 0;
    let mut flag1 = vec![0];
    let mut finder = 0;
    let mut name = vec![] ;
    let mut flag_finder = 0;
    let mut final_flag = false;
    let mut inner_flag = false;
    let mut more_flag = false;
    let mut last_flag = false;
    let mut another_flag = false;
    let i: usize = func_count.len();

    if line.trim().is_empty() {
        another_flag = true;
    }
    for (i,c) in line.chars().enumerate() {
        if i == 0 && c == '/' {
            flag = flag + 1;
        }
        if i == 1 && c == '/' {
            flag = flag + 1;
        }
        if c == 'f' {
            
            flag_finder = finder;
            last_flag = true;
            flag1[0] = 1;      
        }
        if c == 'n' && (finder == flag_finder + 1) {
            
            flag1.push(1);
            
        }
        for i in 0..(flag1.len()-1){
            if flag1[i] == 1{
                inner_flag = true;
            } else {
                inner_flag = false;
            }
        }
        if inner_flag && flag1.len() == 2 && (finder > flag_finder + 1 || last_flag)  {
            if c == '(' {
                more_flag = true;
            } 
            if !more_flag {
                name.push(c);
            }
        }
        finder += 1;
    }
    for i in 0..(flag1.len()-1){
        if flag1[i] == 1{
            final_flag = true;
        } else {
            final_flag = false;
        }
    }
    if flag !=2 && final_flag && flag1.len() == 2{ 
        func_count.push(0);
        
        
    } else {
        func_count[i - 1] += 1;
        if another_flag {
            func_count[i-1] -=1;
        }
    }
    
    let s: String = name.iter().collect();
    let v: Vec<&str> = s.split(' ').collect();
    for i in 0..(v.len()){
        if i == 1 {
            println!("Function {}:\t{}",func_count.len() - 1, v[i]);
        }
        
    }
    func_count
}
