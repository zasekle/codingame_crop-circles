use std::io;
use std::str;

enum WhatToDo {
    PLANT,
    MOW,
    REVERSE
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let instructions_string = input_line.trim_matches('\n').to_string();

    let instructions = instructions_string.split(" ");

    //true is planted, false is mowed
    let mut field = vec![vec![true; 19]; 25];

    for instruction_str in instructions {
        let full_instruction = instruction_str.as_bytes().to_vec();

        let (instruction, what_to_do) =
            if full_instruction[0] as char == 'P' {
                if full_instruction[5] as char == 'M' {
                    (&full_instruction[8..], WhatToDo::REVERSE)
                } else {
                    (&full_instruction[5..], WhatToDo::PLANT)
                }
            } else {
                (&full_instruction[0..], WhatToDo::MOW)
            };

        let center_x = (instruction[0] - ('a' as u8)) as f32;
        let center_y = (instruction[1] - ('a' as u8)) as f32;
        let radius = str::from_utf8(&instruction[2..]).expect("Vec<u8> was not utf8 format").parse::<f32>().unwrap()/2.0;

        for (y, row) in field.iter_mut().enumerate() {
            for (x, b) in row.iter_mut().enumerate() {
                let distance = f32::sqrt((center_y - y as f32).powf(2.0) + (center_x - x as f32).powf(2.0));
                if distance <= radius {
                    *b = match &what_to_do {
                        WhatToDo::PLANT => {
                            true
                        },
                        WhatToDo::MOW => {
                            false
                        },
                        WhatToDo::REVERSE => {
                            !*b
                        }
                    };
                }
            }
        }
    }

    for row in field {
        let mut row_str = String::new();
        for b in row {
            if b {
                row_str += "{}";
            } else {
                row_str += "  ";
            }
        }
        println!("{row_str}");
    }
}
