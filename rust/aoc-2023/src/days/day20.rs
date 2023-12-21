use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    state: bool,
    next: Vec<String>
}

#[derive(Debug, Clone)]
struct Conj {
    name: String,
    state: HashMap<String, bool>,
    next: Vec<String>,
}

#[derive(Debug, Clone)]
struct Broadcast {
    name: String,
    next: Vec<String>,
}

pub trait Module: Debug {
    fn send(&mut self, from: String, pulse:bool) -> Option<bool>;
    fn get_name(&self) -> String;
    fn get_next(&self) -> Vec<String>;
}

impl Module for FlipFlop {
    fn send(&mut self, from: String, pulse:bool) -> Option<bool> {
        match pulse {
            true => None,
            false => {
                self.state = !self.state;
                Some(self.state)
            }
        }
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_next(&self) -> Vec<String> {
        self.next.clone()
    }
}

impl Module for Conj {
    fn send(&mut self, from: String, pulse: bool) -> Option<bool> {
        if let Some(value) = self.state.get_mut(&*from) {
            *value = pulse;
        } else {
            panic!("Trying to send from an input to which the module isn't connected! {:?} -> {:?}", from, self.get_name());
        }
        Some(!self.state.values().all(|&s| s))
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_next(&self) -> Vec<String> {
        self.next.clone()
    }
}

impl Module for Broadcast {
    fn send(&mut self, from: String, pulse: bool) -> Option<bool> {
        Some(false)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_next(&self) -> Vec<String> {
        self.next.clone()
    }
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut modules : HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut pulses: VecDeque<(String, bool, String)> = VecDeque::new();
    let mut reverse_modules: HashSet<(String, String)> = HashSet::new();

    for row in &input {
        let split_row:Vec<&str> = row.split(|s| "->, %&".contains(s))
            .filter(|c| !c.is_empty()).collect();
        let src_name = split_row[0].to_string();
        let destinations: Vec<String> = split_row.iter().skip(1).map(|s|s.to_string()).collect();
        for dst_name in destinations {
            reverse_modules.insert((dst_name.clone(), src_name.clone()));
        }
    }
    println!("Reverse: {:?}", reverse_modules);

    for row in &input {
        let first = row.clone().remove(0);
        let split_row:Vec<&str> = row.split(|s| "->, %&".contains(s))
            .filter(|c| !c.is_empty()).collect();
        let destinations: Vec<String> = split_row.iter().skip(1).map(|s|s.to_string()).collect();
        println!("{:?}", split_row);
        let src_name = split_row[0].to_string();

        match first {
            'b' => {
                modules.insert(src_name.clone(), Box::new(Broadcast {
                    name: String::from("broadcaster"),
                    next: destinations.clone()
                }));
            },
            '%' => {
                modules.insert(src_name.clone(), Box::new(FlipFlop {
                    name: src_name.clone(),
                    state: false,
                    next: destinations.clone()
                }));
            },
            '&' => {
                let sources: HashMap<String, bool> = reverse_modules.iter()
                    .filter(|m| m.0 == src_name)
                    .map(|m| (m.1.clone(), false))
                    .collect();

                modules.insert(src_name.clone(), Box::new(Conj {
                    name: src_name.clone(),
                    state: sources,
                    next: destinations.clone()
                }));
            },
            _ => ()
        }
    }

    let mut low_pulses:i64 = 0;
    let mut high_pulses:i64 = 0;
    for module in &modules {
        println!("{:?}", module);
    }

    for _ in 0..1000 {
        pulses.push_back((String::from("button"), false, String::from("broadcaster")));
        while !pulses.is_empty() {
            let in_pulse: (String, bool, String) = pulses.pop_front().unwrap();
            // println!("Processing pulse: {:?}", in_pulse);
            let mut module = modules.get_mut(&*in_pulse.2);
            match in_pulse {
                (_, false, _) if module.is_some() => low_pulses += 1,
                (_, true, _) if module.is_some() => high_pulses += 1,
                (_, false, _) if module.is_none() => {
                    low_pulses += 1;
                    continue;
                },
                (_, true, _) if module.is_none() => {
                    high_pulses += 1;
                    continue;
                },
                _ => {}
            }
            let mut module = modules.get_mut(&*in_pulse.2).unwrap();
            let destinations = module.get_next();
            let out_pulse = module.send(in_pulse.0, in_pulse.1);
            if let Some(pulse) = out_pulse {
                for dst in destinations {
                    pulses.push_back((module.get_name(), pulse, dst.clone()));
                }
            }
        }
    }

    println!("Low: {} High: {}", low_pulses, high_pulses);

    (Some((low_pulses*high_pulses).to_string()), None)
}