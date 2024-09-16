pub struct Character {
    all_out: f32,
    phy_out: f32,
    mag_out: f32,
}

impl Character {
    pub fn new(all_out: f32, phy_out: f32, mag_out: f32) -> Character {
        Self { all_out, phy_out, mag_out}
    }
}

pub struct Enemy {
    all_in: f32,
    phy_in: f32,
    mag_in: f32,
}

impl Enemy {
    pub fn new(all_in: f32, phy_in: f32, mag_in: f32) -> Enemy {
        Self { all_in, phy_in, mag_in }
    }
}


pub struct Skill {
    pub name: String,
    pub coef: f32,
    pub function: Function,
    pub is_heal: bool
}

impl Skill {
    pub fn new(name: &str, coef: f32, function: Function) -> Skill {
        Self { name: name.to_string() , coef, function, is_heal: coef.is_sign_negative() }
    }

    pub fn calculate(&self, wdps: f32) -> f32 {
        self.coef * &self.function.calculate(wdps)
    }
}


pub enum FuncType {
    Physical,
    Magical,
    Hybrid
}

pub enum Function {
    AP1 { ap: f32, functype: FuncType },
    SP1 { sp: f32, functype: FuncType },
    AP2 { ap: f32, w_range: f32, functype: FuncType },
    SP2 { sp: f32, w_range: f32, functype: FuncType },
}

impl Function {
    pub fn calculate(&self, wdps: f32 ) -> f32 {
        match self {
            Function::AP1 { ap, functype } => wdps + (0.1 * ap),
            Function::SP1 { sp, functype } => wdps + (0.1 * sp),

            Function::AP2 { ap, w_range, functype } => {
                let ap1 = wdps + (0.1 * ap);
                2.0 * ap1 * w_range
            }

            Function::SP2 { sp, w_range, functype } => {
                let sp1 = wdps + (0.1 * sp);
                2.0 * sp1 * w_range
            }
        }
    }

    pub fn get_type(&self) -> FuncType {
        match self {
            Function::AP1 { ap, functype } => FuncType::Physical,
            Function::SP1 { sp, functype } => FuncType::Magical,
            Function::SP2 { sp, w_range, functype } => FuncType::Magical,
            Function::AP2 { ap, w_range, functype } => FuncType::Physical,
        }
    }
}

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
        match func_type {
            FuncType::Hybrid => 
                (self.skill.calculate(wdps) * (self.me.all_out * self.me.phy_out ) * (self.enemy.all_in * self.enemy.phy_in )).round() as i32,
            FuncType::Magical => 
                (self.skill.calculate(wdps) * (self.me.all_out * self.me.phy_out ) * (self.enemy.all_in * self.enemy.phy_in )).round() as i32,
            FuncType::Physical => 
                (self.skill.calculate(wdps) * (self.me.all_out * self.me.phy_out ) * (self.enemy.all_in * self.enemy.phy_in )).round() as i32,

        }
    }
}
