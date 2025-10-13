use bevy::{
    app::MainScheduleOrder,
    ecs::schedule::{ScheduleBuildSettings, ScheduleLabel},
    prelude::*,
};

pub fn plugin(app: &mut App) {
    insert_schedule(app, First, InputSchedule);
    insert_schedule(app, InputSchedule, PreMovement);
    insert_schedule(app, PreMovement, Movement);
    insert_schedule(app, Movement, PostMovement);
}

fn insert_schedule(
    app: &mut App,
    after: impl ScheduleLabel,
    schedule_label: impl ScheduleLabel + Clone,
) {
    let mut schedule = Schedule::new(schedule_label.clone());
    schedule.set_build_settings(ScheduleBuildSettings {
        auto_insert_apply_deferred: false,
        ..Default::default()
    });
    app.add_schedule(schedule);

    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
    main_schedule_order.insert_after(after, schedule_label);
}

/// all user input is handled during this schedule
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSchedule;

/// the game state is prepared for the [`Movement`] stage during this schedule.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreMovement;

/// all "game active" entity movement (changes to [`Transform`]) happens during
/// this schedule (and in no other stage).
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Movement;

/// this schedule includes for example the update to spatial index of movable objects
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostMovement;
