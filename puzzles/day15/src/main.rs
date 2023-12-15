fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    input.split(',')
        .map(compute_hash)
        .sum()
}

fn second_part(input: &str) -> usize {
    let mut lens_boxes = Vec::with_capacity(256);
    for _ in 0..256 {
        lens_boxes.push(LensBox::new());
    }

    input.split(',')
        .for_each(|cmd| {
            if cmd.contains('=') {
                let (label, value) = cmd.split_once('=').expect(&format!("Cannot split command: {}", cmd));
                let index = compute_hash(label);
                let value = value.parse::<u8>().expect(&format!("Cannot parse value: {}", value));
                lens_boxes[index].replace_or_add_lens(label.to_string(), value);
            } else {
                let label = cmd.get(0..cmd.len() - 1).expect(&format!("Cannot get label: {}", cmd));
                let index= compute_hash(label);
                lens_boxes[index].remove_lens(label);
            }
        });
    lens_boxes.iter().enumerate()
        .map(|(index, lens_box)| lens_box.get_focusing_power(index))
        .sum()
}

fn compute_hash(input: &str) -> usize {
    let mut hash = 0;
    input.bytes().for_each(|byte| {
        hash += byte as usize;
        hash *= 17;
        hash %= 256;
    });
    hash
}

struct LensBox {
    lenses: Vec<(String, u8)>
}

impl LensBox {
    fn new() -> Self {
        Self {
            lenses: Vec::new(),
        }
    }

    fn replace_or_add_lens(&mut self, label: String, value: u8) {
        if let Some(current) = self.lenses.iter_mut().find(|(name, _)| *name == label) {
            current.1 = value;
        } else {
            self.lenses.push((label, value));
        }
    }

    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|(lens_name, _)| lens_name != label);
    }

    fn get_focusing_power(&self, box_number: usize) -> usize {
        self.lenses.iter().enumerate()
            .map(|(index, (_, power))| *power as usize * (box_number + 1) * (index + 1))
            .sum()
    }
}