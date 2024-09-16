mod calc;

use calc::{
    skills::{Function, Skill},
    entities::{Character, Enemy},
    calc::Calculator
};

fn main() {
    let me = Character::new(
        1.2, 1.1,
        1.0, 1.0
    );
    let auto = Skill::new(
        "Some Auto",
        1.0,
        Function::AP2 { ap: 850.0, w_range: 1.0 }
    );

    let val = Calculator::new(
        me,
        Enemy::new(1.0, 1.0, 1.0),
        auto
    ).calculate(85.0);
    println!("{val}");
}
