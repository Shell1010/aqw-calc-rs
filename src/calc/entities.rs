#[allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Character {
    all_out: f32,
    phy_out: f32,
    mag_out: f32,
    dot_out: f32,
    heal_out: f32,
    weapon_boost: f32,
}


impl Character {
    pub fn new(all_out: f32, phy_out: f32, mag_out: f32, dot_out: f32, heal_out: f32, weapon_boost: f32) -> Self {
        Self {
            all_out,
            phy_out,
            mag_out,
            dot_out,
            heal_out,
            weapon_boost
        }
    }

    pub fn all_out(&self) -> f32 {
        self.all_out
    }

    pub fn phy_out(&self) -> f32 {
        self.phy_out
    }

    pub fn mag_out(&self) -> f32 {
        self.mag_out
    }

    pub fn heal_out(&self) -> f32 {
        self.heal_out
    }

    pub fn dot_out(&self) -> f32 {
        self.dot_out
    }

    pub fn weapon_boost(&self) -> f32 {
        self.weapon_boost
    }

    pub fn change_all_out(&mut self, n: f32) -> f32 {
        self.all_out += n;
        self.all_out
    }

    pub fn change_phy_out(&mut self, n: f32) -> f32 {
        self.phy_out += n;
        self.phy_out
    }

    pub fn change_mag_out(&mut self, n: f32) -> f32 {
        self.mag_out += n;
        self.mag_out
    }

    pub fn change_heal_out(&mut self, n: f32) -> f32 {
        self.heal_out += n;
        self.heal_out
    }

    pub fn change_dot_out(&mut self, n: f32) -> f32 {
        self.dot_out += n;
        self.dot_out
    }

    pub fn change_weapon_boost(&mut self, n: f32) -> f32 {
        self.weapon_boost += n;
        self.weapon_boost
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    all_in: f32,
    phy_in: f32,
    mag_in: f32,
    dot_in: f32
}

impl Enemy {
    pub fn new(all_in: f32, phy_in: f32, mag_in: f32, dot_in: f32) -> Self {
        Self { all_in, phy_in, mag_in, dot_in }
    }

    pub fn all_in(&self) -> f32 {
        self.all_in
    }

    pub fn phy_in(&self) -> f32 {
        self.phy_in
    }

    pub fn mag_in(&self) -> f32 {
        self.mag_in
    }

    pub fn dot_in(&self) -> f32 {
        self.dot_in
    }

    pub fn change_all_in(&mut self, n: f32) -> f32 {
        self.all_in += n;
        self.all_in
    }

    pub fn change_phy_in(&mut self, n: f32) -> f32 {
        self.phy_in+= n;
        self.phy_in
    }

    pub fn change_mag_in(&mut self, n: f32) -> f32 {
        self.mag_in += n;
        self.mag_in
    }

    pub fn change_dot_in(&mut self, n: f32) -> f32 {
        self.dot_in += n;
        self.dot_in
    }
}