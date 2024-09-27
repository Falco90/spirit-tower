use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
struct MouseCoords(Vec2);

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn update_mouse_coords(
    mut mouse_coords: ResMut<MouseCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();

    if let Some(world_position) = window.cursor_position()
    .and_then(|cursor_coords| camera.viewport_to_world(camera_transform, cursor_coords)).map(|ray| ray.origin.truncate()) {
        mouse_coords.0 = world_position;
    }
}

fn main() {
    App::new().init_resource::<MouseCoords>().add_plugins(DefaultPlugins).add_systems(Startup, spawn_camera).add_systems(Update, update_mouse_coords).run();
}
