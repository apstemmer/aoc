use std::cmp::max;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {

    let mut game_num = 1;
    let mut sum_a = 0;
    let mut sum_b = 0;

    for game in input {
        let right = game.split_once(": ").unwrap().1;
        let draws: Vec<&str> =  right.split("; ").collect();

        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for draw in &draws {
            let colors: Vec<&str> = draw.split(", ").collect();
            for color in colors {
                let parts: Vec<&str> = color.split_whitespace().collect();
                match parts.as_slice() {
                    [num, "red"] => {
                        let red = num.parse::<i32>().unwrap();
                        red_max = max(red_max, red);
                    },
                    [num, "green"] => {
                        let green = num.parse::<i32>().unwrap();
                        green_max = max(green_max, green);
                    },
                    [num, "blue"] => {
                        let blue = num.parse::<i32>().unwrap();
                        blue_max = max(blue_max, blue);
                    },
                    _ => ()
                }
            }
        }

        // Part A
        if red_max <= 12 && green_max <= 13 && blue_max <= 14 {
            // Possible Game
            sum_a += game_num;
        }

        // Part B
        sum_b += red_max * green_max * blue_max;

        println!("{:?}", draws);
        println!("r:{}, g:{}, b:{}", red_max, green_max, blue_max);
        game_num += 1;
    }
    (Some(sum_a.to_string()), Some(sum_b.to_string()))
}