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

    pub fn calculate(&self, wdps: f32) -> i32 {

        let func_type = self.skill.function.get_type();

        let val = match func_type {
            FuncType::Hybrid => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() ),
            FuncType::Magical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.mag_out() ) * (self.enemy.all_in() * self.enemy.mag_in() ),
            FuncType::Physical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() )

        };
        if self.skill.is_heal {
            (val * self.me.heal_out() * self.me.weapon_boost() ) .round() as i32
        } else {
            (val * self.me.weapon_boost() ).round() as i32
        }   
    }

    pub fn unround_calculate(&self, wdps: f32) -> f32 {

        let func_type = self.skill.function.get_type();

        let val = match func_type {
            FuncType::Hybrid => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() ),
            FuncType::Magical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.mag_out() ) * (self.enemy.all_in() * self.enemy.mag_in() ),
            FuncType::Physical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() )

        };
        if self.skill.is_heal {
            val * self.me.heal_out() * self.me.weapon_boost() 
        } else {
            val * self.me.weapon_boost() 
        }   
    }

    pub fn calculate_dot(&self, wdps: f32, duration: f32, tick_interval: i32) -> f32 {
        let func_type = self.skill.function.get_type();

        let val = match func_type {
            FuncType::Hybrid => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() ),
            FuncType::Magical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.mag_out() ) * (self.enemy.all_in() * self.enemy.mag_in() ),
            FuncType::Physical => 
                self.skill.calculate(wdps) * (self.me.all_out() * self.me.phy_out() ) * (self.enemy.all_in() * self.enemy.phy_in() )

        };
        let before: f32;
        if self.skill.is_heal {
            before = val * self.me.heal_out() * self.me.weapon_boost();
        } else {
            before = val * self.me.weapon_boost();
        }

        (before * (duration/tick_interval as f32) * self.me.all_out() * self.enemy.dot_in() * self.me.dot_out() ) / (duration / tick_interval as f32)

    }
}