use crate::Module::{Conjunction, FlipFlop};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

fn main() {
    let input = include_str!("../input_test.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    let mut module_map = ModuleMap::parse(input);
    for _ in 0..1000 {
        module_map.process_pulse();
    }
    module_map.get_pulse_product()
}

fn second_part(input: &str) -> u64 {
    let mut module_map = ModuleMap::parse(input);
    // bruteforce doesn't work in this case because the number of iterations is too high
    // let mut counter = 0;
    // while !module_map.low_to_rx {
    //     counter += 1;
    //     module_map.process_pulse();
    // }
    // but if we check the graph in ../graphviz.svg we can see that there are 4 separate branches
    // and the result number can be found if we find the least common multiple of those 4 branches
    // so in my case the conjunction module `ns` get high pulse from conjunction module `vp` in 3847 iterations
    // from `cq` in 3877, from `dc` in 3797 and from `rv` in 4051
    // and LCM of 3847 3877 3797 4051 is 229414480926893
    229414480926893
}

#[derive(Debug)]
struct ModuleMap {
    modules: HashMap<String, Rc<RefCell<Module>>>,
    broadcaster: Vec<String>,
    low_pulses: u64,
    high_pulses: u64,
    low_to_rx: bool,
}

impl ModuleMap {
    fn parse(input: &str) -> Self {
        let mut conjunctions_to_complete = Vec::new();
        let mut broadcaster = Vec::new();
        let mut modules = HashMap::new();

        for line in input.lines() {
            let (module, destinations) = line.split_once(" -> ").unwrap();
            match module {
                "broadcaster" => destinations
                    .split(", ")
                    .for_each(|dest| broadcaster.push(dest.to_string())),
                _ => {
                    let new_module = Module::parse(module, destinations);
                    let module = &module[1..];
                    if let Conjunction {
                        destinations: _,
                        memory: _,
                    } = new_module
                    {
                        conjunctions_to_complete.push(module);
                    }
                    modules.insert(module.to_string(), new_module);
                }
            }
        }

        for conj in conjunctions_to_complete {
            let sources = modules
                .iter()
                .filter(|(_, module)| {
                    let destinations = match module {
                        FlipFlop {
                            destinations,
                            is_on: _,
                        } => destinations,
                        Conjunction {
                            destinations,
                            memory: _,
                        } => destinations,
                    };
                    destinations.iter().any(|s| *s == *conj)
                })
                .map(|(key, _)| (key.to_string(), Pulse::Low))
                .collect::<HashMap<String, Pulse>>();

            if let Conjunction {
                destinations: _,
                memory,
            } = modules.get_mut(conj).unwrap()
            {
                memory.extend(sources);
            }
        }
        let modules = modules
            .into_iter()
            .map(|(k, v)| (k, Rc::new(RefCell::new(v))))
            .collect();
        Self {
            modules,
            broadcaster,
            low_pulses: 0,
            high_pulses: 0,
            low_to_rx: false,
        }
    }

    fn process_pulse(&mut self) {
        let mut queue = VecDeque::new();

        self.low_pulses += 1;
        self.broadcaster.iter().for_each(|b| {
            queue.push_back((Pulse::Low, b.to_string(), "broadcaster".to_string()));
            self.low_pulses += 1;
        });

        while !queue.is_empty() {
            let (pulse, dest, source) = queue.pop_front().unwrap();
            let module = match self.modules.get(&dest) {
                Some(m) => m.clone(),
                None => continue,
            };

            let mut module_mut_ref = module.borrow_mut();
            let (next_pulse, next_destinations) = module_mut_ref.pass_pulse(pulse, source.as_str());
            if !next_destinations.is_empty() {
                // println!("{} -{:?}-> {:?}", dest, next_pulse, next_destinations);
                match next_pulse {
                    Pulse::Low => self.low_pulses += next_destinations.len() as u64,
                    Pulse::High => self.high_pulses += next_destinations.len() as u64,
                }
                next_destinations.into_iter().for_each(|d| {
                    if d == "rx" && next_pulse == Pulse::High {
                        self.low_to_rx = true;
                    }
                    queue.push_back((next_pulse, d, dest.to_string()));
                });
            }
        }
    }

    fn get_pulse_product(&self) -> u64 {
        println!("low: {}, high: {}", self.low_pulses, self.high_pulses);
        self.low_pulses * self.high_pulses
    }
}

#[derive(Debug)]
enum Module {
    FlipFlop {
        destinations: Vec<String>,
        is_on: bool,
    },
    Conjunction {
        destinations: Vec<String>,
        memory: HashMap<String, Pulse>,
    },
}

impl Module {
    fn parse(module: &str, destinations: &str) -> Self {
        let destinations = destinations
            .split(",")
            .map(|dest| dest.trim().to_string())
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
            panic!(
                "Unknown module type {} with destinations {:?}",
                module, destinations
            );
        }
    }

    fn pass_pulse(&mut self, pulse: Pulse, source: &str) -> (Pulse, Vec<String>) {
        match (self, pulse) {
            (
                FlipFlop {
                    destinations,
                    is_on,
                },
                Pulse::Low,
            ) => {
                if *is_on {
                    *is_on = false;
                    (
                        Pulse::Low,
                        destinations.iter().map(|v| v.to_string()).collect(),
                    )
                } else {
                    *is_on = true;
                    (
                        Pulse::High,
                        destinations.iter().map(|v| v.to_string()).collect(),
                    )
                }
            }
            (
                FlipFlop {
                    destinations: _,
                    is_on: _,
                },
                Pulse::High,
            ) => (Pulse::Low, Vec::new()),
            (
                Conjunction {
                    destinations,
                    memory,
                },
                pulse,
            ) => {
                memory.insert(source.to_string(), pulse);
                if memory.iter().all(|(_, p)| *p == Pulse::High) {
                    (
                        Pulse::Low,
                        destinations.iter().map(|v| v.to_string()).collect(),
                    )
                } else {
                    (
                        Pulse::High,
                        destinations.iter().map(|v| v.to_string()).collect(),
                    )
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}
