use std::{fs, env};
use std::fs::File;
use std::io::Write;

enum Operator {
    INC,
    DEC,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = match *self {
            Operator::INC => "inc",
            Operator::DEC => "dec",
        };
        write!(f, "{}", l)
    }
}

fn parse_file_as_usize(base: &str, filename: &str) -> usize {
    match fs::read_to_string(format!("{}/{}", base, filename)) {
        Ok(m) => {
            match m.trim().parse::<usize>() {
                Ok(u) => u,
                Err(_) => panic!("Failed to parse max_brightness as usize"),
            }
        },
        Err(_) => panic!("Failed to read max_brightness"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Help Message
    if args.len() <= 1 {
        println!("Missing inc or dec argument");
        return;
    }

    // inc or dec
    let operator: String = args[1].clone();
    let operator: Operator = match operator.as_ref() {
        "inc" => Operator::INC,
        "dec" => Operator::DEC,
        _ => panic!("Invalid Operator"),
    };

    let size: Option<usize> = if args.len() >= 3 {
        match args[2].clone().parse::<usize>() {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    } else {
        None
    };

    let base: &str = "/sys/class/backlight/intel_backlight";
    let max: usize = parse_file_as_usize(&base, "max_brightness");
    let brightness: usize = parse_file_as_usize(&base, "brightness");
    // Use size if provided, else 5% change
    let step: usize = match size {
        Some(s) => s,
        None => (max as f64 * 0.05) as usize,
    };

    let new_brightness: usize = match operator {
        Operator::INC => {
            if brightness + step >= max {
                max - 1
            } else {
                brightness + step
            }
        },
        Operator::DEC => {
            if step >= brightness {
                // 0 leaves screen turned off
                1
            } else {
                brightness - step
            }
        },
    };
    let mut f: File = match File::create(format!("{}/{}", &base, "brightness")) {
        Ok(c) => c,
        Err(_) => panic!("Failed to Open Brightness File"),
    };
    f.write_all(format!("{}", new_brightness).as_bytes()).expect("Failed to Write New Brightness");
}
