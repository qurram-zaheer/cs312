use std::io;
use std::f32::consts::PI;
use std::f32;
use std::process;

fn main() {
    let input_vec: Vec<f32> = input_vars();
    println!("{:?}", input_vec);
    let deg_freedom = input_vec[1];
    let t_val = input_vec[0];

    let curve_area = simpsons(0.0 - t_val,t_val,deg_freedom) / 2.0;
    println!("{}", curve_area);
    let mut tails_input = String::new();
    let final_output: f32;
    println!("Enter the number of tails!");
    loop {
        io::stdin()
        .read_line(&mut tails_input)
        .expect("Unable to read from stdin!");
        let tails: i32 = tails_input.trim().parse().expect("Please enter 1 or 2");
        if tails == 1 {
            final_output = 0.5 + curve_area;
        } else if tails == 2 {
            final_output = 2.0 * curve_area;
        } else {
            println!("Please enter 1 or 2 and try again!");
            continue;
        }
    
        println!("Final value: {}", final_output);
        break;
    }
    


}

fn input_vars() -> Vec<f32>{
    let mut input_vars = Vec::new();
    let mut t_input = String::new();
    println!("Enter the t-value");
    io::stdin()
        .read_line(&mut t_input)
        .expect("Unable to read from stdin!");
    let t_val:f32 = t_input.trim().parse().expect("Please enter a valid float");
    let mut deg_freedom_input = String::new();
    println!("Enter the degrees of freedom");
    io::stdin()
        .read_line(&mut deg_freedom_input)
        .expect("Unable to read from stdin!");
    let deg_freedom: f32 = deg_freedom_input.trim().parse().expect("Please enter a valid float");
    if deg_freedom <= 0.0 || t_val < 0.0{
        println!("Please enter >0 value for deg_freedon and >=0 value for t");
        process::exit(1);
    }
    input_vars.push(t_val);
    input_vars.push(deg_freedom);
    input_vars
    
}

fn gamma_func(x:f32) -> f32{
    if x == 1.0 {
        1.0
    } else if x == 0.5 {
        PI.powf(0.5)
    } else {
        (x - 1.0) * gamma_func(x - 1.0)
    }
}

fn dist(x:f32, deg_freedom: f32) -> f32{
    let output = ((gamma_func((deg_freedom + 1.0)/2.0))/((deg_freedom * PI).powf(0.5) * gamma_func(deg_freedom / 2.0))) * (1.0 + (x.powf(2.0)/deg_freedom)).powf(0.0-((deg_freedom + 1.0)/2.0));
    output
}
fn simpsons(a:f32, b:f32, deg_freedom: f32) -> f32{
    let mut n:i32 = 2;
    let mut error:f32 = 999.0;
    let mut output_vec =  Vec::new();
    
    while error >= 0.01 {
        n = n*2;
        let mut odd_sum = 0.0;
        let mut even_sum = 0.0;
        let h:f32 = (b - a) / (n as f32);
        println!("h: {}, n: {}", h, n);
        for i in 0..n{
            if i%2 == 0 {
                even_sum = even_sum + dist(a + i as f32*h, deg_freedom)
            }
            else {
                odd_sum = odd_sum + dist(a + i as f32*h, deg_freedom)
            }
        }
        let output = (h/3.0) * (a + b + 4.0*odd_sum + 2.0 * even_sum);
        if n != 4 {
            error = (output - output_vec[output_vec.len() - 1] as f32).abs();
        }
        output_vec.push(output)

    }
    println!("{:?}", output_vec);
    output_vec[output_vec.len() - 1]
}