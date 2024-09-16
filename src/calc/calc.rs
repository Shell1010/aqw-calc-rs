#[allow(dead_code)]

mod entities;

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
                self.skill.calculate(wdps) * (self.me.all_out * self.me.phy_out ) * (self.enemy.all_in * self.enemy.phy_in ),
            FuncType::Magical => 
                self.skill.calculate(wdps) * (self.me.all_out * self.me.mag_out ) * (self.enemy.all_in * self.enemy.mag_in ),
            FuncType::Physical => 
                self.skill.calculate(wdps) * (self.me.all_out * self.me.phy_out ) * (self.enemy.all_in * self.enemy.phy_in )

        };
        if self.skill.is_heal {
            (val * self.me.heal_out) .round() as i32
        } else {
            val.round() as i32
        }
        
    }
}