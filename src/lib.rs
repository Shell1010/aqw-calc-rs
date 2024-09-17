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
        let rounded = calc.calculate(85.0);
        let diff = ((rounded - in_game) / in_game).abs() * 100;
        let unrounded = calc.unround_calculate(85.0);
        let unrounded_diff = ((unrounded - in_game as f32) / in_game as f32).abs() * 100.0;
        let tolerance = 2;

        assert!(
            diff <= tolerance,
            "Tolerance of 2%. In Game: {in_game}, Actual: {rounded}. Percentage Difference: {diff}%"
        );

        assert!(
            unrounded_diff <= tolerance as f32,
            "Tolerance of 2%. In Game: {in_game}, Actual: {unrounded}. Percentage Difference: {diff}%"
        );
    }


    #[test]
    pub fn dot_test() {
        let me = Character::new(
            1.0, 0.8, 1.6210,
            1.00, 1.50, 1.00
        );
        let enemy = Enemy::new(1.0, 1.0, 1.0, 1.0);
        let skill = Skill::new(
            "SC 4 Skill I forgo", 1.0, false, Function::AP2 { ap: 358.0, w_range: 1.0}
        );
        let calc = Calculator::new(me, enemy, skill);
        let in_game = 192;
        let rounded = calc.calculate_dot(85.0, 8.0, 2);
        let diff = ((rounded - in_game as f32) / in_game as f32).abs() * 100 as f32;
        let tolerance = 2;

        assert!(
            diff <= tolerance as f32,
            "Tolerance of 2%. In Game: {in_game:.1}, Actual: {rounded:.1}. Percentage Difference: {diff:.1}%"
        );
        println!("Tolerance of 2%. In Game: {in_game:.1}, Actual: {rounded:.1}. Percentage Difference: {diff:.1}%")
    }
}
