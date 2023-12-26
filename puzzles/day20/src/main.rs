use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Rc;
use crate::Module::{Broadcaster, Conjunction, FlipFlop};

fn main() {
    let input = include_str!("../input_test.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    let mut module_map = parse(input);
    let res = process_pulse(&mut module_map);
    println!("res: {:?}", res);
    res.0 * res.1
}

fn second_part(input: &str) -> u128 {
    0
}

fn process_pulse<'a>(module_map: &'a mut HashMap<&'a str, Rc<RefCell<Module<'a>>>>) -> (u64, u64) {
    let mut low_pulses = 1;
    let mut high_pulses = 0;
    let mut queue = VecDeque::new();
    if let Broadcaster {destinations} = module_map.get("broadcaster").unwrap().clone() {
        destinations.iter().for_each(|&dest| {
            queue.push_back((Pulse::Low, dest.to_string()));
            low_pulses += 1;
        });
    }
    while !queue.is_empty() {
        let (pulse, dest) = queue.pop_front().unwrap();
        let module = module_map.get_mut(dest.as_str()).unwrap();
        let (next_pulse, next_destinations) = module.borrow().pass_pulse(pulse, dest.as_str());
        match next_pulse {
            Pulse::Low => low_pulses += next_destinations.len() as u64,
            Pulse::High => high_pulses += next_destinations.len() as u64,
        }
        next_destinations.into_iter().for_each(|d|{
            queue.push_back((next_pulse, d));
        });
    }

    (low_pulses, high_pulses)
}

fn parse(input: &str) -> HashMap<&str, Rc<RefCell<Module>>> {
    let mut conjunctions_to_complete = Vec::new();
    let mut map = input.lines()
        .map(|line| {
            let (module, destinations) = line.split_once(" -> ").unwrap();
            let new_module = Module::parse(module, destinations);
            let module = if module.starts_with(|pat| pat == '%' || pat == '&') {
                &module[1..]
            } else {
                module
            };
            if let Conjunction {destinations:_, memory: _} = new_module {
                conjunctions_to_complete.push(module);
            }
            (module, Rc::new(RefCell::new(new_module)))
        })
        .collect::<HashMap<_, _>>();

    for conj in conjunctions_to_complete {
        let sources = map.iter()
            .filter(|(_, module)| {
                let destinations = match module.borrow().deref() {
                    FlipFlop { destinations, is_on: _ } => destinations,
                    Conjunction { destinations, memory: _ } => destinations,
                    Broadcaster { destinations } => destinations,
                };
                destinations.iter().any(|&s| s == conj)
            })
            .map(|(&key, _)| (key.to_string(), Pulse::Low))
            .collect::<HashMap<String, Pulse>>();

        if let Conjunction {destinations: _, memory } = map.get_mut(conj).unwrap().get_mut() {
            memory.extend(sources);
        }
    }
    map
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop {
        destinations: Vec<&'a str>,
        is_on: bool,
    },
    Conjunction {
        destinations: Vec<&'a str>,
        memory: HashMap<String, Pulse>
    },
    Broadcaster {
        destinations: Vec<&'a str>,
    },
}

impl<'a> Module<'a> {
    fn parse(module: &'a str, destinations: &'a str) -> Self {
        let destinations = destinations.split(",")
            .map(|dest| dest.trim())
            .collect();
        if module.starts_with("%") {
            FlipFlop {
                destinations,
                is_on: false,
            }
        } else if module.starts_with("&") {
            Conjunction {
                destinations,
                memory: HashMap::new(),
            }
        } else {
            Broadcaster {
                destinations,
            }
        }
    }

    fn pass_pulse(&'a mut self, pulse: Pulse, source: &str) -> (Pulse, Vec<String>) {
        match (self, pulse) {
            (FlipFlop {destinations, is_on}, Pulse::Low) => {
                if *is_on {
                    *is_on = false;
                    (Pulse::Low, destinations.iter().map(|v| v.to_string()).collect())
                } else {
                    *is_on = true;
                    (Pulse::High, destinations.iter().map(|v| v.to_string()).collect())
                }
            },
            (FlipFlop {destinations: _, is_on: _}, Pulse::High) => {
                (Pulse::Low, Vec::new())
            },
            (Conjunction {destinations, memory}, pulse) => {
                memory.insert(source.to_string(), pulse);
                if memory.iter().all(|(_, p)| *p == Pulse::High) {
                    (Pulse::Low, destinations.iter().map(|v| v.to_string()).collect())
                } else {
                    (Pulse::High, destinations.iter().map(|v| v.to_string()).collect())
                }
            },
            (Broadcaster {destinations}, pulse) => {
                (pulse.clone(), destinations.iter().map(|v| v.to_string()).collect())
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}