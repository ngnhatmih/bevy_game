use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_ecs_ldtk::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    // tao window, camera

    let window = window.single();
    let mut camera = Camera2dBundle::default();
    // chinh camera scale de phong to tilemap
    camera.projection.scale = (536./2.0)/window.width(); 
    let scale = camera.projection.scale;
    // chinh vi tri camera de hien thi 1 phan goc tilemap
    camera.transform.translation.x += window.width() / 2.0 * scale;
    camera.transform.translation.y -= 536. - window.height() / 2.0 * scale;
    
    // them camera vao world
    commands.spawn(camera);

    // load ldtk tile 
    let ldtk_handle = asset_server.load("newtile.ldtk");
    
    // tao ldtk world va them ldtk file
    let mut ldtk_world_bundle = LdtkWorldBundle::default();
    ldtk_world_bundle.ldtk_handle = ldtk_handle;
    
    // them ldtk world vao world
    commands.spawn(ldtk_world_bundle);
}


pub fn on_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut camera: Query<(&OrthographicProjection, &mut Transform), With<Camera2d>>
) {
    // chinh vi tri camera khi resize window
    let (projection, mut transform) = camera.single_mut();
    for e in resize_reader.read() {
        let scale = projection.scale;
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        transform.translation.x += e.width / 2.0 * scale;
        transform.translation.y -= 536. - e.height / 2.0 * scale;
    }
}