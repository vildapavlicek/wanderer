use bevy::ecs::prelude::Query;
use big_brain::choices::Choice;
use big_brain::pickers::Picker;
use big_brain::prelude::Score;

#[derive(Debug, Clone, Default)]
pub struct Below {
    threshold: f32,
}

impl Below {
    pub fn new(threshold: f32) -> Self {
        Below { threshold }
    }
}

impl Picker for Below {
    fn pick(&self, choices: &[Choice], scores: &Query<&Score>) -> Option<Choice> {
        for choice in choices {
            let value = choice.calculate(scores);
            if value < self.threshold {
                return Some(choice.clone());
            }
        }
        None
    }
}
