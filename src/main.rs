use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod systems;

fn main() {        
    // tao app
    // them plugins
    // them resources
    // them systems
    // chay app
    App::new()      
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy ECS LDtk".to_string(),
                    resolution: (683., 384.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(LdtkPlugin)
        .register_ldtk_entity::<systems::PlayerBundle>("Player")
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, systems::setup)
        .add_systems(Update, systems::on_resize)
        .run();
}
