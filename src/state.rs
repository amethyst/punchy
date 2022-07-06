use bevy::prelude::{App, Component, CoreStage, Plugin, Query};
use serde::Deserialize;

use crate::animation::Animation;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, return_to_idle);
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(try_from = "String")]
pub enum State {
    Idle,
    Running,
    Attacking,
    KnockedLeft,
    KnockedRight,
    Hitstun,
    Dying,
}

impl TryFrom<String> for State {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "idle" => State::Idle,
            "running" => State::Running,
            "attacking" => State::Attacking,
            "knocked_left" => State::KnockedLeft,
            "knocked_right" => State::KnockedRight,
            "hitstun" => State::Hitstun,
            "dying" => State::Dying,
            _ => {
                return Err("invalid value");
            }
        })
    }
}

impl State {
    pub fn set(&mut self, state: State) {
        *self = state;
    }

    // pub fn is_knocked(&self) -> bool {
    //     matches!(self, State::KnockedLeft | State::KnockedRight)
    // }
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

fn return_to_idle(mut query: Query<(&mut State, &Animation)>) {
    for (mut state, animation) in query.iter_mut() {
        if !animation.is_repeating() && animation.is_finished() && *state != State::Dying {
            state.set(State::Idle);
        }
    }
}
