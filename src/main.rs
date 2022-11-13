use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(WindowDescriptor {
            title: "Journey to Find (something?)".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(camera_follow)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Player
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("little_guy.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(4.0, 4.0, 4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(VirtualDPad::dpad(), Action::Move)
                .insert(VirtualDPad::arrow_keys(), Action::Move)
                .insert(VirtualDPad::wasd(), Action::Move)
                .build(),
        })
        .insert(Player);

    // Animated planet
    spawn_planet(&mut commands, &asset_server);
}

fn spawn_planet(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("Planet.png");

    let tilemap_entity = commands.spawn().id();

    let size = TilemapSize { x: 1, y: 1 };
    let grid_size = TilemapGridSize { x: 16.0, y: 14.0 };
    let mut tile_storage = TileStorage::empty(size);

    let tile_entity = commands
        .spawn()
        .insert_bundle(TileBundle {
            position: TilePos::new(0, 0),
            tilemap_id: TilemapId(tilemap_entity),
            texture: TileTexture(0),
            ..Default::default()
        })
        .id();

    tile_storage.set(&TilePos::new(0, 0), tile_entity);

    commands.entity(tile_entity).insert(AnimatedTile {
        start: 0,
        end: 212,
        speed: 0.075,
    });

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            map_type: TilemapType::Square {
                diagonal_neighbors: false,
            },
            size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TilemapTileSize { x: 112.0, y: 112.0 },
            transform: Transform {
                translation: Vec3::new(-325.0, 75.0, 0.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
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

fn camera_follow() {}
