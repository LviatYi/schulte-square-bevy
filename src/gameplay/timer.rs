use bevy::prelude::{
    App, Commands, Component, Node, Plugin, Query, Res, Text, Time, With, percent,
};
use bevy::ui::Display;
use bevy::utils::default;
use std::time::{Duration, Instant};

#[derive(Default, Debug, Copy, Clone)]
pub enum TimerState {
    #[default]
    Running,
    Paused,
    Stopped,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Timer {
    state: TimerState,
    elapsed: Duration,
    last_update: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            state: TimerState::Paused,
            elapsed: Duration::ZERO,
            last_update: Instant::now(),
        }
    }
}

impl Timer {
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
}

pub struct TimerViewPlugin;

impl Plugin for TimerViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy::prelude::Startup, build_timer_view);
        app.add_systems(bevy::prelude::Update, update_timer_view);
    }
}

#[derive(Component)]
pub struct TimerView;

fn build_timer_view(mut commands: Commands) {
    commands
        .spawn(Node {
            display: Display::Block,
            height: percent(10),
            ..default()
        })
        .with_child((TimerView, Text::new("")));
    commands.spawn(Timer::default());
}

fn update_timer_view(
    time_res: Res<Time>,
    mut timer_query: Query<&mut Timer>,
    mut query: Query<&mut Text, With<TimerView>>,
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

    for mut label in query.iter_mut() {
        label.0 = time_string.clone();
    }
}
