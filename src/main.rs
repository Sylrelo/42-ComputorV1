use std::env;
use regex::Regex;
use std::str::FromStr;
use std::process;
use colored::Colorize;

fn solve(a: f32, b: f32, c: f32) {
    let d:f32;
    let a2:f32;

    d   = b * b - 4.0 * a * c;
    a2  = 2.0 * a;

    if d > 0.0 {
        let x0 = (-b + d.sqrt()) / a2;
        let x1 = (-b - d.sqrt()) / a2;
        println!("{}", "Disriminant is positive, two solutions : ".green());
        println!("x0 : {}", x0);
        println!("x1 : {}", x1);
    }
    if d == 0.0 {
        let x0 = -b / a2;
        println!("{}", "Discriminant is null, one solution :".yellow());
        println!("x0 : {}", x0);
    }
    if d < 0.0 {
        println!("{}", "Disriminant is negative, no real solutions : ".red());
        println!("{} + i * sqrt({}) / {}", -b, d, a2);
        println!("{} - i * sqrt({}) / {}", -b, d, a2);
    }
}

fn haschr(data: &str, c: char) -> i32 {
    let search_power =  data.to_string().find(c);
    if let Some(e) = search_power {
        return e as i32;
    } else {
        return -1;
    }
}

fn get_power(data: &str) -> usize {
    let power_pos:i32           = haschr(data, '^');
    let mut data_str: Vec<_>    = data.chars().collect();
    let power:u32;

    if power_pos != -1 {
        power = data_str[(power_pos + 1) as usize].to_digit(10).unwrap(); // protéger le unwrap()
    } else {
        if haschr(data, 'x') != -1 || haschr(data, 'X') != -1 {
            power = 1;
        } else {
            power = 0;
        }
    }
    data_str.clear();
    return power as usize;
}

fn match_eq(reg: &Regex, data: &str) -> f32 {
    let reg_capture     = reg.captures(data);
    let mut value:f32   = 1.0;
    match reg_capture {
        Some(result) => {
            let tmp         = result.get(0).map(|s| s.as_str()).unwrap().to_string();
            let tmp_float   = f32::from_str(&tmp);   
            if let Ok(e) = tmp_float {
                value = e;
            } 
        }
        None => {
            let sign = haschr(data, '-');
            if sign != -1 {
                value = -1.0;
            }
        }
    }
    data.clear();
    return value;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong number of arguments\n");
        process::exit(0);
    }
    let  eq: Vec<String>    = args[1].split("=").map(|s| s.to_string().split_whitespace().collect()).collect();
    let base_regex          = Regex::new(r"([+-]?[0-9]*[.]?[0-9]+?\*?[xX]\^[0-9]+)|([+-]?[0-9]*[.]?[0-9]+[xX]?)|([+-]?([xX]\^?[0-9]?))").unwrap();
    let reg_get             = Regex::new(r"(^[+-]?[0-9]*[.]?[0-9]+?)").unwrap();

    let mut val: [f32; 3] = [0.0; 3];

    for cap in base_regex.captures_iter(&eq[0]){
        let power:usize     = get_power(&cap[0]);
        val[power]          = match_eq(&reg_get, &cap[0]);
    }
    println!("\n{:+}x^2{:+}x^1{:+}x^0 = {}\n", val[2], val[1], val[0], eq[1]);
    if eq.len() == 2 {
        for cap in base_regex.captures_iter(&eq[1]){ 
            let power:usize     = get_power(&cap[0]);
            val[power]          -= match_eq(&reg_get, &cap[0]);
        }
     println!("\n{:+}x^2{:+}x^1{:+}x^0 = 0\n", val[2], val[1], val[0]);
    }
    solve(val[2], val[1], val[0]);
    // println!("\n");
}

// 42*x  >  à gérer