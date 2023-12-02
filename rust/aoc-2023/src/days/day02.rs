use std::cmp::max;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {

    let mut game_num = 1;
    let mut sum_a = 0;

    for game in input {
        let right = game.split_once(": ").unwrap().1;
        let draws: Vec<&str> =  right.split("; ").collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for draw in &draws {
            let colors: Vec<&str> = draw.split(", ").collect();
            for color in colors {
                let parts: Vec<&str> = color.split_whitespace().collect();
                match parts.as_slice() {
                    [num, "red"] => red = max(red, num.parse::<i32>().unwrap()),
                    [num, "green"] => green = max(green,num.parse::<i32>().unwrap()),
                    [num, "blue"] => blue = max(blue,num.parse::<i32>().unwrap()),
                    _ => ()
                }
            }
            println!("r:{}, g:{}, b:{}", red, green, blue);
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            // Possible Game
            sum_a += game_num;
        }
        println!("{:?}", draws);
        game_num += 1;
    }
    (Some(sum_a.to_string()), None)
}