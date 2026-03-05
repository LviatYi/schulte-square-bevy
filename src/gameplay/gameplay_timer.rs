use crate::gameplay::main_panel::SchulteMainPanel;
use bevy::prelude::{Commands, Component, Query, Res, Single, Text, Time, With};
use std::time::{Duration, Instant};

#[derive(Default, Debug, Copy, Clone)]
pub enum TimerState {
    #[default]
    Running,
    Paused,
    Stopped,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct GameplayTimer {
    state: TimerState,
    elapsed: Duration,
    last_update: Instant,
}

impl Default for GameplayTimer {
    fn default() -> Self {
        Self {
            state: TimerState::Paused,
            elapsed: Duration::ZERO,
            last_update: Instant::now(),
        }
    }
}

impl GameplayTimer {
    pub fn resume_at(&mut self, at: Instant) -> &mut Self {
        if let TimerState::Paused = self.state {
            self.state = TimerState::Running;
            self.last_update = at;
        }
        self
    }

    pub fn resume(&mut self) -> &mut Self {
        self.resume_at(Instant::now());
        self
    }

    pub fn pause(&mut self) -> &mut Self {
        if let TimerState::Running = self.state {
            self.state = TimerState::Paused;
        }
        self
    }

    pub fn stop(&mut self) -> &mut Self {
        self.state = TimerState::Stopped;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.state = TimerState::Paused;
        self.elapsed = Duration::ZERO;
        self
    }

    pub fn tick_at(&mut self, at: Instant) -> &mut Self {
        if let TimerState::Running = self.state {
            self.force_tick_at(at);
        }
        self
    }

    pub fn tick_duration(&mut self, dur: Duration) -> &mut Self {
        if let TimerState::Running = self.state {
            self.elapsed += dur;
            self.last_update += dur;
        }
        self
    }

    pub fn tick(&mut self) -> &mut Self {
        self.tick_at(Instant::now())
    }

    pub fn force_tick_duration(&mut self, dur: Duration) -> &mut Self {
        self.elapsed += dur;
        self.last_update += dur;
        self
    }

    pub fn force_tick_at(&mut self, at: Instant) -> &mut Self {
        let delta = at.duration_since(self.last_update);
        self.elapsed += delta;
        self.last_update = at;
        self
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

#[derive(Component)]
pub struct GameplayTimerView;

pub fn build_gameplay_timer_view(mut commands: Commands, main_panel: Single<&SchulteMainPanel>) {
    commands
        .entity(main_panel.timer_view_slot)
        .with_child((GameplayTimerView, Text::new("")));
    commands.spawn(GameplayTimer::default());
}

pub fn update_gameplay_timer_view(
    time_res: Res<Time>,
    mut timer_query: Query<&mut GameplayTimer>,
    mut view_query: Query<&mut Text, With<GameplayTimerView>>,
) {
    let elapsed = if let Ok(mut timer) = timer_query.single_mut() {
        timer.tick_duration(time_res.delta());
        timer.elapsed
    } else {
        Duration::ZERO
    };

    let seconds = elapsed.as_secs() % 60;
    let minutes = (elapsed.as_secs() / 60) % 60;
    let milliseconds = (elapsed.subsec_millis()) % 1000;

    let time_string = format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds);

    for mut label in view_query.iter_mut() {
        label.0 = time_string.clone();
    }
}
