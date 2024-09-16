#[allow(dead_code)]
pub struct Character {
    all_out: f32,
    phy_out: f32,
    mag_out: f32,
    heal_out: f32,
}

impl Character {
    pub fn new(all_out: f32, phy_out: f32, mag_out: f32, heal_out: f32) -> Character {
        Self { all_out, phy_out, mag_out, heal_out}
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

    pub fn add(&mut self, all_in: f32, phy_in: f32, mag_in: f32) {
        self.all_in += all_in;
        self.phy_in += phy_in;
        self.mag_in += mag_in;
    }

    pub fn subtract(&mut self, all_in: f32, phy_in: f32, mag_in: f32) {
        self.all_in -= all_in;
        self.phy_in -= phy_in;
        self.mag_in -= mag_in;
    }
}
