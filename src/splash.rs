use crate::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Splash)
            .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(splash_countdown))
            .add_system_set(SystemSet::on_exit(GameState::Splash).with_system(splash_cleanup));
    }
}

#[derive(Component)]
struct SplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D camera
    commands.spawn(Camera2dBundle::default());

    // Animated planet
    spawn_planet(&mut commands, &asset_server);

    // starry background
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("title_screen_background.png"),
            transform: Transform::from_scale(Vec3::new(4.0, 4.0, 4.0)),
            ..Default::default()
        })
        .insert(SplashScreen);
}

fn splash_countdown(
    button_inputs: Res<Input<GamepadButton>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _ in button_inputs.get_just_pressed() {
        game_state.set(GameState::Game).unwrap();
    }
}

fn splash_cleanup(mut commands: Commands, query: Query<Entity, With<SplashScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_planet(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("Planet.png");

    let tilemap_entity = commands.spawn_empty().id();

    let size = TilemapSize { x: 1, y: 1 };
    let grid_size = TilemapGridSize { x: 16.0, y: 14.0 };
    let mut tile_storage = TileStorage::empty(size);

    let tile_entity = commands
        .spawn(TileBundle {
            position: TilePos::new(0, 0),
            tilemap_id: TilemapId(tilemap_entity),
            texture_index: TileTextureIndex(0),
            ..Default::default()
        })
        .insert(SplashScreen)
        .id();

    tile_storage.set(&TilePos::new(0, 0), tile_entity);

    commands.entity(tile_entity).insert(AnimatedTile {
        start: 0,
        end: 212,
        speed: 0.075,
    });

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type: TilemapType::Square,
        size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TilemapTileSize { x: 112.0, y: 112.0 },
        transform: Transform {
            translation: Vec3::new(-325.0, 75.0, 1.0),
            scale: Vec3::splat(4.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
