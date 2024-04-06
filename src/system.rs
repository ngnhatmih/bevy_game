use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_ecs_ldtk::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {

    let window = window.single();
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = (536./2.0)/window.width(); 
    let scale = camera.projection.scale;
    camera.transform.translation.x += window.width() / 2.0 * scale;
    camera.transform.translation.y -= 536. - window.height() / 2.0 * scale;
    
    commands.spawn(camera);
    let ldtk_handle = asset_server.load("newtile.ldtk");
    
    let mut ldtk_world_bundle = LdtkWorldBundle::default();
    ldtk_world_bundle.ldtk_handle = ldtk_handle;
    
    commands.spawn(ldtk_world_bundle);
}


pub fn on_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut camera: Query<(&OrthographicProjection, &mut Transform), With<Camera2d>>
) {
    let (projection, mut transform) = camera.single_mut();
    for e in resize_reader.read() {
        let scale = projection.scale;
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        transform.translation.x += e.width / 2.0 * scale;
        transform.translation.y -= 536. - e.height / 2.0 * scale;
    }
}