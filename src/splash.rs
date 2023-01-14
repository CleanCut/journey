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

#[derive(Component)]
struct Continue;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D camera
    commands.spawn(Camera2dBundle::default());

    // Animated planet
    spawn_planet(&mut commands, &asset_server);

    // Continue "button"
    spawn_continue(&mut commands, &asset_server);

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
    mut query: Query<&mut TileTextureIndex, With<Continue>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for _ in button_inputs.get_just_pressed() {
        let _ = game_state.set(GameState::Game);
    }
    let mut tti = query.single_mut();
    for ev in cursor_moved_events.iter() {
        let mouse_pos = ev.position - Vec2::new(1280.0, 720.0) * 0.5;
        let continue_pos = Vec2::new(300.0, 180.0);
        if ((mouse_pos.x - continue_pos.x).abs() < (continue_pos.x))
            & ((mouse_pos.y - continue_pos.y).abs() < (continue_pos.y))
        {
            tti.0 = 1;
            if mouse_button_input.just_released(MouseButton::Left) {
                let _ = game_state.set(GameState::Game);
            }
        } else {
            tti.0 = 0;
        }
    }
}

fn splash_cleanup(mut commands: Commands, query: Query<Entity, With<SplashScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_continue(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("continue.png");
    let tilemap_entity = commands.spawn_empty().id();
    let size = TilemapSize { x: 1, y: 1 };
    let grid_size = TilemapGridSize { x: 1.0, y: 2.0 };
    let mut tile_storage = TileStorage::empty(size);
    let tile_entity = commands
        .spawn(TileBundle {
            position: TilePos::new(0, 0),
            tilemap_id: TilemapId(tilemap_entity),
            texture_index: TileTextureIndex(0),
            ..Default::default()
        })
        .insert(SplashScreen)
        .insert(Continue)
        .id();

    tile_storage.set(&TilePos::new(0, 0), tile_entity);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type: TilemapType::Square,
        size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TilemapTileSize { x: 128.0, y: 64.0 },
        transform: Transform {
            translation: Vec3::new(300.0, 180.0, 1.0),
            scale: Vec3::splat(4.0),
            ..Default::default()
        },
        ..Default::default()
    });
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
