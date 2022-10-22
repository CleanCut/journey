use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(WindowDescriptor {
            title: "Journey to Find (something?)".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Player
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("little_guy.png"),
        transform: Transform::from_scale(Vec3::new(4.0, 4.0, 4.0)),
        ..Default::default()
    });
}
