
fn arrangements(prefix: Vec<char>, row:Vec<char>, groups:Vec<i32>) -> i32{
    println!("{:?}, {:?}, {:?}", prefix, row, groups);
    match (row.len(), groups.len()) {
        (_, 0) => return 1,
        (0, _) => return 0,
        _ => ()
    }
    match row[0] {
        '.' => {
            let mut strip_row: Vec<char> = row.clone();
            let mut prefix_ = prefix.clone();
            prefix_.push(strip_row.remove(0));
            return arrangements(prefix_, strip_row, groups.clone());
        }
        '#' => {
            let spring_count = groups[0] as usize;
            if spring_count > row.len() {
                return 0;
            }
            if row.iter().take(spring_count).all(|c| *c == '?' || *c == '#') {
                if row.len() == spring_count {
                    let mut prefix_ = prefix.clone();
                    prefix_.append(&mut vec!['#'; spring_count]);
                    return arrangements(prefix_,  vec![], vec![]);
                } else if row[spring_count] != '#' {
                    let mut prefix_ = prefix.clone();
                    prefix_.append(&mut row.iter().take(spring_count + 1).cloned().collect());
                    return arrangements(prefix_,row.iter().skip(spring_count + 1).cloned().collect(),
                                        groups.iter().skip(1).cloned().collect());
                }
            }
            return 0;
        }
        '?' => {
            let mut fill_row = row.clone();
            let mut fill_groups = groups.clone();
            let mut empty_row = row.clone();
            let mut empty_groups = groups.clone();
            fill_row[0] = '#';
            empty_row[0] = '.';
            return arrangements(prefix.clone(), fill_row, fill_groups) + arrangements(prefix.clone(), empty_row, empty_groups)
        }
        _ => panic!("Should never happen!")
    }
    0
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut sum_a = 0;
    for row in input {
        let mut iter = row.split_whitespace();
        let springs: Vec<char> = iter.next().unwrap().chars().collect();
        let groups: Vec<i32> = iter.next().unwrap()
            .split(|c| c == ',')
            .map(|gr| gr.parse::<i32>().unwrap())
            .collect();
        let count = arrangements(vec![],springs.clone(), groups.clone());
        println!("{:?} -> {:?} -> {:?}", springs, groups, count);
        sum_a += count;
    }
    (Some(sum_a.to_string()), None)
}