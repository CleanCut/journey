use bevy::{input::gamepad::GamepadSettings, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use journey::{prelude::*, splash::SplashPlugin};
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                // Set the window title
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Journey to Find (something?)".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                // Prevent blurry sprites
                .set(ImagePlugin::default_nearest())
                // Enable asset hot loading
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(TilemapPlugin)
        .add_plugin(SplashPlugin)
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup)
                .with_system(configure_gamepads),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(movement.label("movement"))
                .with_system(camera_follow.after("movement")),
        )
        .run();
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation = player_transform.translation;

    // Code for letting the player get closer to the edge of the screen
    //
    // let x_dist = player_transform.translation.x - camera_transform.translation.x;
    // let x_max = 500.0;
    // if x_dist > x_max {
    //     camera_transform.translation.x = player_transform.translation.x - x_max;
    // } else if x_dist < -x_max {
    //     camera_transform.translation.x = player_transform.translation.x + x_max;
    // }
    // let y_dist = player_transform.translation.y - camera_transform.translation.y;
    // let y_may = 500.0;
    // if y_dist > y_may {
    //     camera_transform.translation.y = player_transform.translation.y - y_may;
    // } else if y_dist < -y_may {
    //     camera_transform.translation.y = player_transform.translation.y + y_may;
    // }
}

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("little_guy.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(4.0, 4.0, 4.0),
                ..Default::default()
            },
            ..Default::default()
        },
        InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(VirtualDPad::dpad(), Action::Move)
                .insert(VirtualDPad::arrow_keys(), Action::Move)
                .insert(VirtualDPad::wasd(), Action::Move)
                .build(),
        },
        Player,
    ));

    // grass
    commands.spawn(SpriteBundle {
        texture: asset_server.load("grass_square.png"),
        transform: Transform::from_scale(Vec3::new(4.0, 4.0, 4.0)),
        ..Default::default()
    });
}

const MOVE_SPEED: f32 = 200.0;

fn movement(
    time: Res<Time>,
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>,
) {
    let (action_state, mut transform) = query.single_mut();

    if action_state.pressed(Action::Move) {
        let axis_pair = action_state.clamped_axis_pair(Action::Move).unwrap();
        transform.translation.x += MOVE_SPEED * time.delta_seconds() * axis_pair.x();
        transform.translation.y += MOVE_SPEED * time.delta_seconds() * axis_pair.y();
    }
}

fn configure_gamepads(mut settings: ResMut<GamepadSettings>) {
    // add a larger default dead-zone to all axes (ignore small inputs, round to zero)
    settings
        .default_axis_settings
        .set_deadzone_lowerbound(-0.125);
    settings
        .default_axis_settings
        .set_deadzone_upperbound(0.125);
}
