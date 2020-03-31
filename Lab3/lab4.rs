use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::io;
use std::path::Path;



/*
SPECIFICATIONS: 
    1. Enter the input data into a .txt file, with two columns, 'estimated LOC' is column 1 and 'actual LOC' is column 2. Do not provide labels for any columns. The split by a single space. 
                        Eg: 40 32
                             50 65
                             etc etc
    2. 
    !CRITICAL: data.txt CANNOT CONTAIN NON INTEGER VALUES, NON INTEGER VALUES LEAD TO INACCURATE RESULTS
    3. Program will run in a loop asking you to enter the name of the .txt file for data. Enter the keyword 'quit' to exit the program.
*/
fn main(){
    let mut flag = true;

    while flag {
        let mut estimated = Vec::new();
        let mut actual = Vec::new();
        println!("Enter the name of the data file (.txt format): "); 
        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
                        .expect("Failed to read input!");
        let quit = String::from("quit");
        let fn_length = filename.chars().count();
        if filename.trim() == quit {
            println!("Quitting program, goodbye");
            break;
        } else {
            if fn_length < 5 {
                println!("Please enter a valid file name with extension, try again!");
                continue;
            } else {
                let file_type = &filename[fn_length-4..fn_length-1];
            if file_type != "txt" {
                println!("Unsupported file type! Only .txt files supported!, please try again");
                continue;
            }
            let cwd = env::current_dir().unwrap();
            let my_str = cwd.into_os_string().into_string().unwrap();
            let temp = ["/",filename.trim()].join("");
            let result = [my_str,temp].join("");
            if Path::new(&result).exists(){
                //println!("{}", result);
                let file = File::open(filename.trim()).unwrap();
                let reader = BufReader::new(file);
                for (_index, line) in reader.lines().enumerate(){
                    let line = line.unwrap();
                    let samples = line.split_whitespace();
                    for (i,sample) in samples.enumerate(){
                        if i == 0 {
                            estimated.push(sample.to_string());
                        } else if i == 1{
                            actual.push(sample.to_string());
                        } else {
                            println!("The number of columns in data.txt doesn't match the specifications of this program!");
                        }
                    }
                }
                let mut sig_x:f64 = 0.0;
                let mut sig_y:f64= 0.0;
                let mut sig_x_sq:f64 = 0.0;
                let mut sig_xy:f64 = 0.0;
                let optb = -1.0;
                for i in 0..estimated.len(){
                    sig_x = sig_x + estimated[i].parse().unwrap_or(optb);
                    sig_y = sig_y + actual[i].parse().unwrap_or(optb);
                    sig_x_sq = sig_x_sq + estimated[i].parse().unwrap_or(optb).powi(2);
                    sig_xy = sig_xy + estimated[i].parse().unwrap_or(optb)*actual[i].parse().unwrap_or(optb);
                }
                // println!("sig(x): {} sig(y): {} sig(xy): {} sig(xsq): {}", sig_x, sig_y, sig_xy, sig_x_sq);
                let n: f64 = estimated.len() as f64;
        
                let m: f64 = (n*sig_xy - (sig_x*sig_y))/(n*sig_x_sq - sig_x.powi(2));
                let c: f64 = (sig_y*sig_x_sq - sig_x*sig_xy)/(n*sig_x_sq - sig_x.powi(2));
                println!("\tB0\t\t|\tB1");
                println!("----------------------------------------");
                println!("{}\t   {}", m, c);
            } else {
                println!("File {} not found in current directory. Try again", filename);
                continue;
            }  
            }
            
        }  
    }
}


