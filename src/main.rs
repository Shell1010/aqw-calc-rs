mod strucs;
use strucs::{Skill, Function, Character};
fn main() {
    let me = Character::new(
        1.2, 1.1,
        1.0, 1.0
    );
    let auto = Skill::new(
        "Some Auto",
        1.0,
        Function::AP2 { ap: 850.0, w_range: 1.0 },
        false
    );

    println!("{}", auto.calculate(85.0));
}
