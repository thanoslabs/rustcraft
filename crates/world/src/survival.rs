//! Deterministic health, hunger, and starvation rules for survival gameplay.

pub const MAX_HEALTH: u8 = 20;
pub const MAX_FOOD: u8 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurvivalStats {
    pub health: u8,
    pub food: u8,
    pub saturation: f32,
    pub exhaustion: f32,
}

impl Default for SurvivalStats {
    fn default() -> Self {
        Self {
            health: MAX_HEALTH,
            food: MAX_FOOD,
            saturation: 5.0,
            exhaustion: 0.0,
        }
    }
}

impl SurvivalStats {
    pub fn damage(&mut self, amount: u8) {
        self.health = self.health.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u8) {
        self.health = self.health.saturating_add(amount).min(MAX_HEALTH);
    }

    pub fn eat(&mut self, food: u8, saturation: f32) {
        self.food = self.food.saturating_add(food).min(MAX_FOOD);
        self.saturation = (self.saturation + saturation.max(0.0)).min(f32::from(self.food));
    }

    pub fn add_exhaustion(&mut self, amount: f32) {
        if amount.is_finite() && amount > 0.0 {
            self.exhaustion += amount;
        }
    }

    /// Consume exhaustion in discrete classic hunger units.
    pub fn update_hunger(&mut self) {
        while self.exhaustion >= 4.0 {
            self.exhaustion -= 4.0;
            if self.saturation > 0.0 {
                self.saturation = (self.saturation - 1.0).max(0.0);
            } else {
                self.food = self.food.saturating_sub(1);
            }
        }
    }

    /// Apply one periodic hunger tick. The caller controls the tick interval.
    pub fn periodic_tick(&mut self, difficulty: Difficulty) {
        if difficulty == Difficulty::Peaceful {
            self.food = MAX_FOOD;
            self.heal(1);
        } else if self.food == MAX_FOOD && self.health < MAX_HEALTH {
            self.heal(1);
            self.add_exhaustion(3.0);
        } else if self.food == 0 {
            let floor = match difficulty {
                Difficulty::Peaceful => MAX_HEALTH,
                Difficulty::Easy => 10,
                Difficulty::Normal => 1,
                Difficulty::Hard => 0,
            };
            if self.health > floor {
                self.damage(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturation_is_spent_before_food() {
        let mut stats = SurvivalStats::default();
        stats.add_exhaustion(24.0);
        stats.update_hunger();
        assert_eq!(stats.saturation, 0.0);
        assert_eq!(stats.food, 19);
    }

    #[test]
    fn starvation_respects_difficulty_floor() {
        let mut easy = SurvivalStats { food: 0, ..Default::default() };
        for _ in 0..30 { easy.periodic_tick(Difficulty::Easy); }
        assert_eq!(easy.health, 10);
        let mut hard = SurvivalStats { food: 0, ..Default::default() };
        for _ in 0..30 { hard.periodic_tick(Difficulty::Hard); }
        assert_eq!(hard.health, 0);
    }

    #[test]
    fn food_and_healing_are_bounded() {
        let mut stats = SurvivalStats::default();
        stats.damage(100);
        stats.heal(100);
        stats.eat(100, 100.0);
        assert_eq!(stats.health, MAX_HEALTH);
        assert_eq!(stats.food, MAX_FOOD);
        assert_eq!(stats.saturation, 20.0);
    }
}
