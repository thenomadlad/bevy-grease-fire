//! A splash screen that plays briefly at startup.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{AppSystems, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen);

    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSystems::Computations),
            check_splash_timer.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    app.add_systems(
        Update,
        enter_title_screen
            .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::Splash))),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;

fn spawn_splash_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Splash Screen"),
        BackgroundColor(SPLASH_BACKGROUND_COLOR),
        DespawnOnExit(Screen::Splash),
    ));
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(timer: ResMut<SplashTimer>, mut next_screen: ResMut<NextState<Screen>>) {
    if timer.0.just_finished() {
        next_screen.set(Screen::Title);
    }
}

fn enter_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
