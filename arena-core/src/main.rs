use std::collections::VecDeque;

trait Creature {
    fn update(&self);
    fn merge(&self);
    fn faire(&self);
}

fn init() {}

fn process(
    life_cycle: &mut VecDeque<Box<dyn Creature>>,
    creatures: &mut Vec<Box<dyn Creature>>,
) -> bool {
    while let Some(item) = life_cycle.pop_front() {
        item.update();
    }
    for item in &mut *creatures {
        item.merge();
    }
    // push new borns
    for item in creatures {
        item.faire();
    }
    false
}

fn main() {
    let mut life_cycle: VecDeque<Box<dyn Creature>> = VecDeque::new();
    let mut creatures: Vec<Box<dyn Creature>> = Vec::new();

    init();
    loop {
        if !process(&mut life_cycle, &mut creatures) {
            break;
        };
    }
}
