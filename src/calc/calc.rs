#[allow(dead_code)]

use crate::calc::{
    entities::{Character, Enemy},
    skills::{Skill, FuncType}
};

#[derive(Debug, Clone)]
pub struct Calculator {
    me: Character,
    enemy: Enemy,
    skill: Skill,
}

impl Calculator {
    pub fn new ( me: Character, enemy: Enemy, skill: Skill ) -> Calculator {
        Self { me, enemy, skill }
    }


    pub fn calculate(&self, wdps: f32, duration: Option<f32>, tick_interval: Option<i32>) -> f32 {

        let func_type = self.skill.function.get_type();

        let val = match func_type {
            FuncType::Hybrid => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() ),
            FuncType::Magical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.mag_out() ) * (self.enemy.all_in() * self.enemy.mag_in() ),
            FuncType::Physical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() ),
            FuncType::TrueDamage =>
                // For some reason only weapon boost needed for reg damage
                if self.skill.is_dot {
                    self.skill.calculate(wdps) * self.me.all_out()
                } else {
                    self.skill.calculate(wdps)
                }
        };
        let mut before: f32;
        if self.skill.is_heal {
            before = val * self.me.heal_out();
        } else {
            before = val;
        }

        if !self.skill.is_dot {
            before = before * self.me.weapon_boost();
        }


        if self.skill.is_dot {
            // For a single tick of DoT
            if let (Some(duration), Some(tick_interval)) = (duration, tick_interval) {
                (before * 
                    (duration/tick_interval as f32) *
                    self.me.all_out() *
                    self.enemy.dot_in() *
                    self.me.dot_out() * 
                    self.me.mag_out() 
                ) / (duration / tick_interval as f32)
            } else {
                0.0
            }
            
        } else {
            before
        }
    }


}