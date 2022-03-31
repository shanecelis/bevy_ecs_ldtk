// Roughly equivalent to the "basic" example, except it doesn't use the LdtkEntity convenience
// trait. As a result, you can run this example with --no-default-features

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(process_my_entity)
        .insert_resource(LevelSelection::Index(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

fn process_my_entity(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, entity_instance) in entity_query.iter() {
        if entity_instance.identifier == "MyEntityIdentifier".to_string() {
            let tileset = asset_server.load("atlas/MV Icons Complete Sheet Free - ALL.png");

            if let Some(tile) = &entity_instance.tile {
                let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
                    tileset.clone(),
                    Vec2::new(tile.w as f32, tile.h as f32),
                    16,
                    95,
                ));

                let sprite = TextureAtlasSprite {
                    index: (tile.y / tile.h) as usize * 16 + (tile.x / tile.w) as usize,
                    ..Default::default()
                };

                commands.entity(entity).insert_bundle(SpriteSheetBundle {
                    texture_atlas,
                    sprite,
                    transform: transform.clone(),
                    ..Default::default()
                });
            }
        }
    }
}
