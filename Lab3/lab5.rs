use std::io;
use std::fs;
use std::path::Path;


/* 
SPECIFICATIONS:
    !CRITICAL
    You will need 4 CSV files containing only integers. The columns with proxy names have been mapped to an integer index, please do not include them.
    The columns with Similar to proxies need to be replaced with the proxy number. Type Calculation is mapped to 0, type Data to 1, and type IO to 2; 
*/

    
/* Read csv file */
fn parse_file(file: &str) -> Vec<Vec<f32>> {
	let contents = fs::read_to_string(&file)
		.expect("Something went wrong reading the file");
	let split_nl: Vec<&str> = contents.split('\n').collect();

	let mut matrix:Vec<Vec<f32>> = [].to_vec();
	let mut f = 1;

	for line in split_nl {
		if line.trim().len() > 0 {
			let split_del: Vec<&str> = line.split(',').collect();

			if f == 1{
				for _col_idx in 0..split_del.len(){
					matrix.push(Vec::<f32>::new());
				}
				f = 0;
			}

			for col_idx in 0..split_del.len(){
				match split_del[col_idx].parse::<i32>() {
					Ok(i) => {
						matrix[col_idx].push(i as f32);
					}
					Err(..) => {
						println!("Did not find integer. Quiting...");
						std::process::exit(1)
					}
				}
			}
		}
	}
	return matrix;
}

/* Compute LOC/method */
fn loc_per_method(historic: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
	let mut new_historic = historic.to_vec();
	new_historic.push(Vec::<f32>::new());
	for row_idx in 0..historic[0].len(){
		new_historic[historic.len() as usize].push(historic[0][row_idx]/historic[1][row_idx]);
	}
	return new_historic;
}

/* Compute mean of the vector */
fn mean(x: &Vec<f32>) -> f32 {
	let mut sum = 0.0;
	for _idx in 0..x.len(){
		sum += x[_idx as usize];
	}
	return sum / (x.len() as f32);
}

/* Compute variance of vector */
fn variance(x: &Vec<f32>) -> f32 {
	let mut var = 0.0;
	let mean_x = mean(x);
	for _idx in 0..x.len(){
		var += (x[_idx as usize] - mean_x).powf(2.0);
	}
	return var;
}

/* Get filename from user */
fn get_filename() -> String {
	// Conditional var
	let mut cond = 1;
	// Var to save input
	let mut filename = String::new();
	// Create string to capture input
	while cond == 1{
		println!("Enter Filename:");
		let mut input_text = String::new();
		io::stdin()
			.read_line(&mut input_text)
			.expect("Failed to read from stdin");
		let trimmed = input_text.trim();
		// Check if file path exists
		if Path::new(&trimmed).exists() == true {
			filename = trimmed.to_string();
			cond = 0;
		} else if Path::new(&trimmed).exists() == false {
			println!("File does not exists, Re-enter...");
		}
	}
	return filename;
}

/* Correlation of two vectors */
fn correlation(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
	let mean_x = mean(&x);
	let mean_y = mean(&y);

	let mut numerator: f32 = 0.0;
	for idx in 0..x.len(){
		numerator += (x[idx] - mean_x)*(y[idx] - mean_y);
	}

	return numerator/((variance(&x)*variance(&y)).powf(0.5));
}

fn matrix_gen(proxy_data: &Vec<Vec<f32>>) -> Vec<Vec<Vec<f32>>>{
    //println!("{:?}", proxy_data);
    let mut calc_proxies = Vec::new();
    let mut data_proxies = Vec::new();
    let mut io_proxies = Vec::new();
    let mut sums = vec![0.0, 0.0, 0.0];
    let mut lens = vec![0, 0, 0];

    for i in 0..proxy_data[0].len()
        {let temp = vec![proxy_data[0][i], proxy_data[1][i], proxy_data[2][i], proxy_data[3][i]];
        if proxy_data[2][i] == 0.0 {       
            calc_proxies.push(temp);
            sums[0] = sums[0] + proxy_data[3][i];
            lens[0] = lens[0] + 1;
        } else if proxy_data[2][i] == 1.0 {
            data_proxies.push(temp);
            sums[1] = sums[1] + proxy_data[3][i];
            lens[1] = lens[1] + 1;
        } else if proxy_data[2][i] == 2.0 {
            io_proxies.push(temp);
            sums[2] = sums[2] + proxy_data[3][i];
            lens[2] = lens[2] + 1;
        }}
    
    // println!("{:?}", calc_proxies);
    // println!("{:?}", data_proxies);
    // println!("{:?}", io_proxies);

    let mut sum = 0.0;
    let mut len = 0.0;
    let mut means = Vec::<f32>::new();
    for item in &calc_proxies{
        sum = sum + item[3] as f32;
        len = len + 1.0;
    }
     
    means.push((sum/len) as f32);
    lens[0] = len as i32;

    sum = 0.0;
    len = 0.0;
    for item in &data_proxies{
        sum = sum + item[3] as f32;
        len = len + 1.0;
    }
     
    means.push((sum/len) as f32);

    lens[1] = len as i32;

    sum = 0.0;
    len = 0.0;
    for item in &io_proxies{
        sum = sum + item[3] as f32;
        len = len + 1.0;
    }
     
    means.push((sum/len) as f32);
    lens[2] = len as i32;
    let mut sum2 = 0.0;
    //println!("Means: {:?}", means);
    let mut stds = Vec::<f32>::new();
    
    for item in &calc_proxies {
        sum2 = sum2 as f32 + ((item[3] as f32) - (means[0] as f32)).powf(2.0);
    }
    
    stds.push((sum2/((lens[0] as f32) - 1.0)).powf(0.5));
    sum2 = 0.0;
    for item in &data_proxies {
        sum2 = sum2 as f32 + ((item[3] as f32) - (means[1] as f32)).powf(2.0);
    }
    
    stds.push((sum2/((lens[1] as f32) - 1.0)).powf(0.5));
    sum2 = 0.0;
    for item in &io_proxies {
        sum2 = sum2 as f32 + ((item[3] as f32)- (means[2] as f32)).powf(2.0);
    }
    stds.push((sum2/((lens[2] as f32) - 1.0)).powf(0.5));
    //println!("{:?}", stds);

    let mut calc_mid = Vec::new();
    let mut data_mid = Vec::new();
    let mut io_mid = Vec::new();
    
    
    let mut flag1 = 0;
    let mut flag2 = 0;
    let mut flag3 = 0;
    if means[0] < 2.0*stds[0] {
        let mut tsum1 = 0.0;
        for item in &mut calc_proxies {
            tsum1 = tsum1 + item[3].log(10.0);
            item[3] = item[3].log(10.0);
        }
        means[0] = tsum1/(lens[0] as f32);
        
        let mut stsum1 = 0.0;
        for item in &calc_proxies {
            stsum1 = stsum1 + ((item[3] as f32) - (means[0])).powf(2.0);
            
        }
        stds[0] = (stsum1/((lens[0] as f32) - 1.0)).powf(0.5);
        
        flag1 = 1; 
    }
    if means[1] < 2.0*stds[1] {
        let mut tsum2 = 0.0;
        for item in &mut data_proxies {
            tsum2 = tsum2 + item[3].log(10.0);
            item[3] = item[3].log(10.0);
        }
        means[1] = tsum2/(lens[1] as f32);
        
        let mut stsum2 = 0.0;
        for item in &data_proxies {
            stsum2 = stsum2 + ((item[3] as f32) - (means[1])).powf(2.0);
            
        }
        stds[1] = (stsum2/((lens[1] as f32) - 1.0)).powf(0.5);
        
        flag2 = 1;
    }
    if means[2] < 2.0*stds[2] {
        let mut tsum3 = 0.0;
        for item in &mut io_proxies {
            tsum3 = tsum3 + item[3].log(10.0);
            item[3] = item[3].log(10.0);
        }
        means[2] = tsum3/(lens[2] as f32);
        
        let mut stsum3 = 0.0;
        for item in &io_proxies {
            stsum3 = stsum3 + ((item[3] as f32) - (means[2])).powf(2.0);
            
        }
        stds[2] = (stsum3/((lens[2] as f32) - 1.0)).powf(0.5);
        
        flag3 = 1;
    }



    let mut counter = 2.0;
    for _i in 0..5 {
        if flag1 == 0 {
            
            calc_mid.push(means[0] - counter*stds[0]);
        } else {
            calc_mid.push((10.0 as f32).powf(means[0] - counter*stds[0]));
            
        }
        if flag2 == 0 {
            
            data_mid.push(means[1] - counter*stds[1]);
        } else {
            data_mid.push((10.0 as f32).powf(means[1] - counter*stds[1]));
        }
        if flag3 == 0 {
            
            io_mid.push(means[2] - counter*stds[2]);
        } else {
            io_mid.push((10.0 as f32).powf(means[2] - counter*stds[2]));
        }
        
        counter = counter - 1.0;
       
    }
    //println!("Calc mid{:?}", calc_mid);
    let mut calc_lb = Vec::new();
    let mut calc_ub = Vec::new();
    let mut data_lb = Vec::new();
    let mut data_ub = Vec::new();
    let mut io_lb = Vec::new();
    let mut io_ub = Vec::new();
    for i in 0..5 {
        if flag1 == 0{
            calc_lb.push(calc_mid[i] - 0.5*stds[0]);
            calc_ub.push(calc_mid[i] + 0.5*stds[0]);
        } else {
            let mut temp_log = calc_mid[i].log(10.0);
            temp_log = temp_log - 0.5*stds[0];
            temp_log = (10.0 as f32).powf(temp_log);
            calc_lb.push(temp_log);
            temp_log = calc_mid[i].log(10.0) + 0.5*stds[0];
            calc_ub.push((10.0 as f32).powf(temp_log));
        }
        if flag2 == 0{
            data_lb.push(data_mid[i] - 0.5*stds[1]);
            data_ub.push(data_mid[i] + 0.5*stds[1]);
        } else {
            let mut temp_log;
            temp_log = data_mid[i].log(10.0);
            temp_log = temp_log - 0.5*stds[1];
            temp_log = (10.0 as f32).powf(temp_log);
            data_lb.push(temp_log);
            temp_log = data_mid[i].log(10.0) + 0.5*stds[1];
            data_ub.push((10.0 as f32).powf(temp_log));
        }
        if flag3 == 0{
            io_lb.push(io_mid[i] - 0.5*stds[2]);
            io_ub.push(io_mid[i] + 0.5*stds[2]);
        } else {
            let mut temp_log;
            temp_log = io_mid[i].log(10.0);
            temp_log = temp_log - 0.5*stds[2];
            temp_log = (10.0 as f32).powf(temp_log);
            io_lb.push(temp_log);
            temp_log = io_mid[i].log(10.0) + 0.5*stds[2];
            io_ub.push((10.0 as f32).powf(temp_log));
        }
        
    }

    let mut calc_cum = Vec::new();
    let mut data_cum = Vec::new();
    let mut io_cum = Vec::new();
    let mut size_matrix = Vec::new();

    
    calc_cum.push(calc_mid);
    calc_cum.push(calc_lb);
    calc_cum.push(calc_ub);
    
    data_cum.push(data_mid);
    data_cum.push(data_lb);
    data_cum.push(data_ub);
    
    io_cum.push(io_mid);
    io_cum.push(io_lb);
    io_cum.push(io_ub);

    size_matrix.push(calc_cum);
    size_matrix.push(data_cum);
    size_matrix.push(io_cum);
    
    //println!("2 - {:?}", calc_proxies);
    
    size_matrix

    

}

fn main()
{
    let mut size_matrix = Vec::new();
    size_matrix.push(Vec::new());
    size_matrix[0].push(Vec::new());
    println!("Enter the name of the csv file with proxy data");
    let filename = get_filename();
    let historic = parse_file(&filename);
    

    // Var to save input
    println!("Enter the name of the csv file with LOCe, LOCa, De and Da values");
    
	let filename = get_filename();
	let estimates = parse_file(&filename);

    let his_loc_p_method = loc_per_method(historic);
    
    size_matrix=  matrix_gen(&his_loc_p_method);
    

    let mut vec_meth = Vec::new();
    let mut vec_rel_loc = Vec::new();
    let mut vec_type = Vec::new();
    let mut vec_size = Vec::new();

    let vec_cal_ub;
    let vec_cal_lb;
    let vec_cal_m;

    let vec_io_ub;
    let vec_io_lb;
    let vec_io_m;

    let vec_data_ub;
    let vec_data_lb;
    let vec_data_m;

    vec_cal_m=size_matrix[0][0].to_vec();
    vec_cal_ub=size_matrix[0][2].to_vec();
    vec_cal_lb=size_matrix[0][1].to_vec();

    vec_io_m=size_matrix[2][0].to_vec();
    vec_io_ub=size_matrix[2][2].to_vec();
    vec_io_lb=size_matrix[2][1].to_vec();

    vec_data_m=size_matrix[1][0].to_vec();
    vec_data_ub=size_matrix[1][2].to_vec();
    vec_data_lb=size_matrix[1][1].to_vec();
    
    println!("Enter the name of the csv file with base addition data");
    let vectbatab=parse_file(&get_filename());
    
    for i in 0..vectbatab[0].len()
    {
        vec_meth.push(vectbatab[0][i]);
        vec_type.push(vectbatab[2][i]);
        vec_rel_loc.push(his_loc_p_method[3][(vectbatab[1][i]-1.0) as usize]);
        vec_size.push(0.0);
    }
    println!("Enter the name of the csv file with NO data");
    let vectbatno=parse_file(&get_filename());
    
    for i in 0..vectbatno[0].len()
    {
        vec_meth.push(vectbatno[0][i]);
        vec_type.push(vectbatno[2][i]);
        vec_rel_loc.push(his_loc_p_method[3][(vectbatno[1][i]-1.0) as usize]);
        vec_size.push(0.0);
    }

    let mut pos=0;

    for i in 0..vec_type.len()
    {
        if vec_type[i]==0.0
        {
            if vec_rel_loc[i] as f32>vec_cal_ub[vec_cal_ub.len()-1]
            {
                pos=4;
            }
            else
            {
                for j in 0..vec_cal_m.len()
                {
                    if vec_rel_loc[i] as f32>=vec_cal_lb[j] && vec_rel_loc[i] as f32<=vec_cal_ub[j]
                    {
                        pos=j
                    }
                }
            }
            vec_size[i]=vec_meth[i] as f32*vec_cal_m[pos];
        }

        if vec_type[i]==2.0
        {
            if vec_rel_loc[i] as f32>vec_io_ub[vec_io_ub.len()-1]
            {
                pos=4;
            }
            else
            {
                for j in 0..vec_io_m.len()
                {
                    if vec_rel_loc[i] as f32>=vec_io_lb[j] && vec_rel_loc[i] as f32<=vec_io_ub[j]
                    {
                        pos=j
                    }
                }
            }
            vec_size[i]=vec_meth[i] as f32*vec_io_m[pos]
        }

        if vec_type[i]==1.0
        {   
            if vec_rel_loc[i] as f32>vec_data_ub[vec_data_ub.len()-1]
            {
                pos=4;
            }
            else
            {
                for j in 0..vec_data_m.len()
                {
                    if vec_rel_loc[i] as f32>=vec_data_lb[j] && vec_rel_loc[i] as f32<=vec_data_ub[j]
                    {
                        pos=j;
                    }
                }
            }
            vec_size[i]=vec_meth[i] as f32*vec_data_m[pos]
        }
    }
    // println!("{:?}", vec_data_m);
    // println!("{:?}",vec_meth);
    // println!("{:?}",vec_rel_loc);
    // println!("{:?}",vec_type);
    
    let mut estimated_size = 0.0;
    let base_size = 369.0;
    let loc_deleted = 14.0;


    for num in &vec_size {
        estimated_size = estimated_size + num;
    }
    //estimated_size = estimated_size + base_size - loc_deleted;

    let r = correlation(&estimates[0], &estimates[1]);
    println!("Correlation between LOCe and LOCa: {}", r);

    let t_crit = 1.06717;
    let t_stat_loce_loca = t_value(r, 20);
    println!("t_stat LOCe LOCa: {}", t_stat_loce_loca);
    let mut theta = Vec::new();
    let mut x;
    let mut y;
    let range_1: f32;
    let mut lpi_1: f32;
    let upi_1: f32;
    let productivity: f32;
    let mut regression_loca: f32;
    if t_stat_loce_loca > t_crit {
        println!("LOCe and LOCa are signifcantly correlated");
        x = estimates[0].to_vec();
        y = estimates[1].to_vec();
        theta = regression(&x, &y);
        println!("Regression params for LOCe vs LOCa: {:?}", theta);
        regression_loca = theta[0] * estimated_size + theta[1];
        regression_loca = regression_loca + base_size - loc_deleted;
        range_1 = range(&x, &y, t_crit, theta[0], theta[1], regression_loca);
        lpi_1 = regression_loca - range_1;
        if lpi_1 < 1.0 {
            lpi_1 = 0.0/0.0;
        }
        upi_1 = regression_loca + range_1;
        
        productivity = mean(&estimates[1]) / mean(&estimates[3]);
        println!("Productivity: {}", productivity);
    } else {
        regression_loca = 0.0/0.0;
        theta.push(0.0);
        theta.push(1.0);
        range_1 = 0.0/0.0;
        lpi_1 = 0.0/0.0;
        upi_1 = 0.0/0.0;
    }

    let r_2 = correlation(&estimates[0], &estimates[3]);
    let t_stat_loce_da = t_value(r_2, 20);
    println!("t_stat LOCe Da {}",t_stat_loce_da);
    let mut theta_1 = Vec::new();
    let range_2: f32;
    let mut lpi_2: f32;
    let upi_2: f32;
    let regression_da: f32;

    if t_stat_loce_da > t_crit {
        println!("LOCe and Da are signifcantly correlated");
        x = estimates[0].to_vec();
        y = estimates[3].to_vec();
        theta_1 = regression(&x, &y);
        println!("Regression parameters for LOCe vs Da{:?}", theta_1);
        regression_da = theta_1[0] * regression_loca + theta_1[1];
        range_2 = range(&x, &y, t_crit, theta_1[0], theta_1[1], regression_da);
        lpi_2 = regression_da - range_2;
        if lpi_2 < 1.0 {
            lpi_2 = 0.0/0.0;
        }
        upi_2 = regression_da + range_2;
    } else {
        regression_da = 0.0/0.0;
        theta_1.push(0.0);
        theta_1.push(1.0);
        range_2 = 0.0/0.0;
        lpi_2 = 0.0/0.0;
        upi_2 = 0.0/0.0;
    }
    println!("Regression LOCa: {}, Range for LOCe vs LOCa: {}, LPI: {}, UPI: {}", regression_loca, range_1, lpi_1, upi_1);
    println!("Regression Da: {}, Range for LOCe vs Da: {}, LPI: {}, UPI: {}", regression_da, range_2, lpi_2, upi_2);

}

fn t_value(r: f32, n: i32) -> f32 {
    let t_stat = r.abs()*((n-2) as f32).powf(0.5)/(1.0-(r.powf(2.0))).powf(0.5);
    t_stat
}

fn regression(x: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    let mut sig_x:f32 = 0.0;
    let mut sig_y:f32= 0.0;
    let mut sig_x_sq:f32 = 0.0;
    let mut sig_xy:f32 = 0.0;
    for i in 0..x.len(){
        sig_x = sig_x + x[i];
        sig_y = sig_y + y[i];
        sig_x_sq = sig_x_sq + x[i].powi(2);
        sig_xy = sig_xy + y[i]*x[i];
    }
    // println!("sig(x): {} sig(y): {} sig(xy): {} sig(xsq): {}", sig_x, sig_y, sig_xy, sig_x_sq);
    let n: f32 = x.len() as f32;

    let m: f32 = (n*sig_xy - (sig_x*sig_y))/(n*sig_x_sq - sig_x.powi(2));
    let c: f32 = (sig_y*sig_x_sq - sig_x*sig_xy)/(n*sig_x_sq - sig_x.powi(2));
    let mut theta = Vec::new();
    theta.push(m);
    theta.push(c);
    theta
}

fn range(x: &Vec<f32>, y: &Vec<f32>, t_crit: f32, b1: f32, b0: f32, e_value: f32) -> f32 {
    let mut sum = 0.0;
    let mut sum1 = 0.0;
    for i in 0..x.len() {
        sum = sum + ((y[i] - b1*x[i] - b0).powf(2.0));
        sum1 = sum1 + (x[i] - mean(x)).powf(2.0);
    }
    
    let output = t_crit * ((sum/x.len() as f32)).powf(0.5) * ((1.0 + ((1.0/((x.len() as f32) as f32 - 2.0))) + (e_value/sum1))).powf(0.5);
    output
}