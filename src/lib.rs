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
        let me = Character::new(1.3, 1.00, 1.00, 1.00, 1.00, 1.00);
        let enemy = Enemy::new(1.0, 1.0, 1.0, 1.0);
        let skill = Skill::new(
            "Auto Attack", 1.0, false, Function::AP2 { ap: 834.0, w_range: 1.0 }
        );
        let calc = Calculator::new(me, enemy, skill);
        let in_game = 437;
        let val = calc.calculate(85.0, None, None);
        let diff = ((val as i32 - in_game) / in_game).abs() * 100;

        let tolerance = 2;

        assert!(
            diff <= tolerance,
            "Tolerance of 2%. In Game: {in_game}, Actual: {val}. Percentage Difference: {diff}%"
        );
    }


    // CSS 3 - 0.1 cHPm
    // SC 4 - 0.65 AP1
    // LR 4 - Effected by Wpn Range
    // I'm gonna raise tolerance to 5% just for DoTs cause seems to be effected by rounding more
    #[test]
    pub fn dot_test() {
        let me = Character::new(
            1.45, 1.0, 2.0108,
            1.00, 1.5, 1.51
        );
        let enemy = Enemy::new(1.0, 1.0, 1.0, 1.0);
        let skill = Skill::new(
            "LR 4 Skill I forgo", -0.275, true, Function::SP2 { sp: 1442.0, w_range: 1.0 }
        );
        
        let calc: Calculator = Calculator::new(me, enemy, skill);
        let in_game = -716;
        let val = calc.calculate(85.0, Some(10.0), Some(5));
        println!("{val}");
        let diff = ((val - in_game as f32) / in_game as f32).abs() * 100 as f32;
        let tolerance = 5;

        assert!(
            diff <= tolerance as f32,
            "Tolerance of 5%. In Game: {in_game:.1}, Actual: {val:.1}. Percentage Difference: {diff:.1}%"
        );
        
    }

    #[test]
    pub fn truedmg_test() {
        let me = Character::new(
            1.0, 1.27, 1.5981,
            1.00, 1.0, 1.51
        );
        let enemy = Enemy::new(1.0, 1.0, 1.0, 1.0);
        let skill = Skill::new(
            "SS 5 lol", 0.1, false, Function::cHPm { max_hp: 2910.0 } 
        );
        let calc = Calculator::new(me, enemy, skill);
        let in_game = 440;
        let val = calc.calculate(85.0, Some(8.0), Some(2));
        let diff = ((val - in_game as f32) / in_game as f32).abs() * 100 as f32;
        let tolerance = 2;

        assert!(
            diff <= tolerance as f32,
            "Tolerance of 2%. In Game: {in_game:.1}, Actual: {val:.1}. Percentage Difference: {diff:.1}%"
        );

    }
}
