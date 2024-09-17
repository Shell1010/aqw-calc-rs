/*
DoTs require additional all out multiplier
Implement when I'm not lazy
*/

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub coef: f32,
    pub function: Function,
    pub is_dot: bool,
    pub is_heal: bool
}

impl Skill {
    pub fn new(name: &str, coef: f32, is_dot: bool, function: Function) -> Skill {
        Self { 
            name: name.to_string(),
            coef, function, is_dot,
            is_heal: coef.is_sign_negative()
        }
    }

    pub fn calculate(&self, wdps: f32) -> f32 {
        self.coef * &self.function.calculate(wdps)
    }
}

#[derive(Debug, Clone)]
pub enum FuncType {
    Physical,
    Magical,
    Hybrid
}

#[derive(Debug, Clone)]
pub enum Function {
    AP1 { ap: f32,  },
    SP1 { sp: f32,  },
    AP2 { ap: f32, w_range: f32,  },
    SP2 { sp: f32, w_range: f32,  },
}

impl Function {
    pub fn calculate(&self, wdps: f32 ) -> f32 {
        match self {
            Function::AP1 { ap, } => wdps + (0.1 * ap),
            Function::SP1 { sp, } => wdps + (0.1 * sp),

            Function::AP2 { ap, w_range, } => {
                let ap1 = wdps + (0.1 * ap);
                2.0 * ap1 * w_range
            }

            Function::SP2 { sp, w_range, } => {
                let sp1 = wdps + (0.1 * sp);
                2.0 * sp1 * w_range
            }
        }
    }

    pub fn get_type(&self) -> FuncType {

        match self {
            Function::AP1 { ap: _, } => FuncType::Physical,
            Function::SP1 { sp: _, } => FuncType::Magical,
            Function::SP2 { sp: _, w_range: _, } => FuncType::Magical,
            Function::AP2 { ap: _, w_range: _, } => FuncType::Physical,
        }
    }
}


