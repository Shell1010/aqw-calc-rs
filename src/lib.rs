pub mod calc;


#[cfg(test)]
mod tests {
    use super::*;

    use calc::{
        skills::{Function, Skill},
        entities::{Character, Enemy},
        calc::Calculator
    };

    #[test]
    pub fn damage_test() {
        let me = Character::new(1.3, 1.00, 1.00, 1.00, 1.00);
        let enemy = Enemy::new(1.0, 1.0, 1.0);
        let skill = Skill::new(
            "Auto Attack", 1.0, Function::AP2 { ap: 834.0, w_range: 1.0 }
        );
        let calc = Calculator::new(me, enemy, skill);
        println!("{:#?}", calc);
        println!("{}", calc.calculate(85.0));
        println!("{}", calc.unround_calculate(85.0));
    }
}
