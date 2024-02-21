use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let (rules, parts) = parse_input(input);
    parts
        .iter()
        .filter(|part| is_part_accepted(&rules, part))
        .map(|part| part.sum_values())
        .sum()
}

fn is_part_accepted(rules: &HashMap<&str, Rule>, part: &Part) -> bool {
    let mut current_rule = "in";
    loop {
        let rule = rules.get(current_rule).unwrap();
        current_rule = rule.get_next_rule(part);
        if current_rule == "A" {
            return true;
        } else if current_rule == "R" {
            return false;
        }
    }
}

fn second_part(input: &str) -> u128 {
    let (rules, _) = parse_input(input);
    rec_compute_number_of_possible_accepted(&rules, "in", ConditionCalculator::new())
}

fn rec_compute_number_of_possible_accepted(
    rules: &HashMap<&str, Rule>,
    current_rule: &str,
    mut cc: ConditionCalculator,
) -> u128 {
    let mut sum = 0;
    let rule = rules.get(current_rule).unwrap();
    for (category, condition, next_rule) in rule.sub_rules.iter() {
        let mut new_cc = cc.clone();
        cc.add_condition(*category, condition.negate());
        new_cc.add_condition(*category, *condition);
        if next_rule == "A" {
            sum += new_cc.compute_possible_variations();
        } else if next_rule == "R" {
            continue;
        } else {
            sum += rec_compute_number_of_possible_accepted(rules, next_rule, new_cc.clone());
        }
    }
    match rule.last_resort.as_str() {
        "A" => {
            sum += cc.compute_possible_variations();
        }
        "R" => {}
        next => sum += rec_compute_number_of_possible_accepted(rules, next, cc),
    }
    sum
}

fn parse_input(input: &str) -> (HashMap<&str, Rule>, Vec<Part>) {
    let (rules_input, parts_input) = input.split_once("\n\n").unwrap();

    let rules = rules_input
        .lines()
        .map(|line| {
            let (key, rule) = line.split_once("{").unwrap();
            let rule = rule.trim_end_matches("}");
            let rule = Rule::parse(rule);
            (key, rule)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts_input
        .lines()
        .map(|line| Part::parse(&line[1..line.len() - 1]))
        .collect::<Vec<_>>();

    (rules, parts)
}

#[derive(Debug, Clone)]
struct Rule {
    sub_rules: Vec<(Category, Condition, String)>,
    last_resort: String,
}

impl Rule {
    fn parse(input: &str) -> Self {
        let mut sub_rules = Vec::new();
        let mut last_resort = String::new();

        let mut rule_split = input.split(",").peekable();
        while rule_split.peek().is_some() {
            let split = rule_split.next().unwrap();
            if rule_split.peek().is_none() {
                last_resort = split.to_string();
                break;
            }
            let category = Category::parse(&split[..1]);
            let (condition, next_rule) = split.split_once(":").unwrap();
            let condition = Condition::parse(&condition[1..]);
            sub_rules.push((category, condition, next_rule.to_string()))
        }

        Self {
            sub_rules,
            last_resort,
        }
    }

    fn get_next_rule(&self, part: &Part) -> &str {
        for (category, condition, next_rule) in self.sub_rules.iter() {
            if let Some(val) = part.map.get(category) {
                if condition.is_valid(*val) {
                    return next_rule;
                }
            }
        }
        &self.last_resort
    }
}

#[derive(Debug, Copy, Clone)]
struct Condition {
    value: i64,
    operation: Operation,
}

impl Condition {
    fn parse(input: &str) -> Self {
        let operation = &input[..1];
        let operation = Operation::parse(operation);

        Self {
            value: input[1..].parse().unwrap(),
            operation,
        }
    }

    fn is_valid(&self, value: i64) -> bool {
        match self.operation {
            Operation::Lt => value < self.value,
            Operation::Gt => value > self.value,
        }
    }

    fn negate(&self) -> Self {
        match self.operation {
            Operation::Lt => Self {
                operation: Operation::Gt,
                value: self.value - 1,
            },
            Operation::Gt => Self {
                operation: Operation::Lt,
                value: self.value + 1,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn parse(input: &str) -> Self {
        match input {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Invalid category '{}'", input),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Lt,
    Gt,
}

impl Operation {
    fn parse(input: &str) -> Self {
        match input {
            "<" => Self::Lt,
            ">" => Self::Gt,
            _ => panic!("Invalid operation '{}'", input),
        }
    }
}

#[derive(Debug)]
struct Part {
    map: HashMap<Category, i64>,
}

impl Part {
    fn parse(input: &str) -> Self {
        let map = input
            .split(",")
            .map(|part| {
                let category = Category::parse(&part[..1]);
                let value = part[2..].parse().unwrap();
                (category, value)
            })
            .collect::<HashMap<_, _>>();

        Self { map }
    }

    fn sum_values(&self) -> usize {
        self.map.values().map(|value| *value as usize).sum()
    }
}

#[derive(Debug, Clone)]
struct ConditionCalculator {
    x_s: Vec<Condition>,
    m_s: Vec<Condition>,
    a_s: Vec<Condition>,
    s_s: Vec<Condition>,
}

impl ConditionCalculator {
    fn new() -> Self {
        Self {
            x_s: Vec::new(),
            m_s: Vec::new(),
            a_s: Vec::new(),
            s_s: Vec::new(),
        }
    }

    fn add_condition(&mut self, category: Category, condition: Condition) {
        match category {
            Category::X => self.x_s.push(condition),
            Category::M => self.m_s.push(condition),
            Category::A => self.a_s.push(condition),
            Category::S => self.s_s.push(condition),
        }
    }

    fn compute_possible_variations(&self) -> u128 {
        fn compute_variations(conditions: &[Condition]) -> u128 {
            let mut min = 1;
            let mut max = 4000;
            for condition in conditions {
                match condition.operation {
                    Operation::Lt => {
                        if condition.value < max {
                            max = condition.value - 1;
                        }
                    }
                    Operation::Gt => {
                        if condition.value > min {
                            min = condition.value + 1;
                        }
                    }
                }
            }
            (max - min) as u128 + 1
        }
        let x = compute_variations(&self.x_s);
        let m = compute_variations(&self.m_s);
        let a = compute_variations(&self.a_s);
        let s = compute_variations(&self.s_s);
        x * m * a * s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn condition_calculator_test() {
        let mut cc = ConditionCalculator::new();
        cc.add_condition(
            Category::X,
            Condition {
                value: 2,
                operation: Operation::Lt,
            },
        );
        cc.add_condition(
            Category::M,
            Condition {
                value: 2,
                operation: Operation::Lt,
            },
        );
        assert_eq!(cc.compute_possible_variations(), 4000 * 4000);

        let mut cc = ConditionCalculator::new();
        assert_eq!(cc.compute_possible_variations(), 4000 * 4000 * 4000 * 4000);

        let mut cc = ConditionCalculator::new();
        cc.add_condition(
            Category::X,
            Condition {
                value: 2,
                operation: Operation::Lt,
            },
        );
        cc.add_condition(
            Category::M,
            Condition {
                value: 2,
                operation: Operation::Lt,
            },
        );
        cc.add_condition(
            Category::A,
            Condition {
                value: 2,
                operation: Operation::Lt,
            },
        );
        cc.add_condition(
            Category::S,
            Condition {
                value: 2,
                operation: Operation::Lt,
            },
        );
        assert_eq!(cc.compute_possible_variations(), 1);

        let mut cc = ConditionCalculator::new();
        cc.add_condition(
            Category::S,
            Condition {
                value: 1351,
                operation: Operation::Lt,
            },
        );
        assert_eq!(cc.compute_possible_variations(), 86_400_000_000_000);

        let mut cc = ConditionCalculator::new();
        cc.add_condition(
            Category::S,
            Condition {
                value: 1351,
                operation: Operation::Lt,
            },
        );
        cc.add_condition(
            Category::M,
            Condition {
                value: 2090,
                operation: Operation::Gt,
            },
        );
        assert_eq!(cc.compute_possible_variations(), 4000 * 4000 * 1350 * 1910);
    }
}
