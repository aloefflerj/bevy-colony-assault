use crate::collider::components::*;
use crate::config::resources::*;
use bevy::prelude::*;

pub fn load_colliders(
    query: Query<(Entity, &Transform, &ColliderConfig), With<ColliderConfig>>,
    debug_options: Res<DebugOptions>,
    mut commands: Commands,
) {
    let collider_debug_view_color = Srgba::hex("51afe6").unwrap();
    let collider_debug_view_color = Color::srgba(
        collider_debug_view_color.red,
        collider_debug_view_color.green,
        collider_debug_view_color.blue,
        0.1,
    );

    for (entity, transform, collider_config) in query.iter() {
        let sprite_config = if debug_options.collider {
            Some(Sprite {
                color: collider_debug_view_color,
                ..default()
            })
        } else {
            None
        };

        let transform = Transform {
            translation: Vec3::new(transform.translation.x, transform.translation.y, -1000.),
            scale: Vec3::new(collider_config.size.x, collider_config.size.y, 0.),
            ..default()
        };

        let sprite = match sprite_config {
            Some(sprite) => sprite,
            None => Sprite::default(),
        };

        let sprite_bundle = SpriteBundle {
            transform,
            sprite,
            ..default()
        };

        let collider_object = ColliderObject {
            attached_to_entity_id: entity.index(),
        };

        if collider_config.collider_type == ColliderType::Actor {
            commands.spawn((sprite_bundle, collider_object, Actor));
        } else if collider_config.collider_type == ColliderType::Solid {
            commands.spawn((sprite_bundle, collider_object, Solid));
        } else {
            commands.spawn((sprite_bundle, collider_object));
        }
    }
}

pub fn stick_solid_colliders_to_entities(
    mut collider_query: Query<(&mut Transform, &ColliderObject), With<Solid>>,
    transform_query: Query<(Entity, &Transform), (With<Transform>, Without<Solid>)>,
) {
    for (mut collider_transform, collider) in collider_query.iter_mut() {
        for (entity, entity_transform) in transform_query.iter() {
            if collider.attached_to_entity_id == entity.index() {
                collider_transform.translation = entity_transform.translation;
            }
        }
    }
}

pub fn stick_actor_colliders_to_entities(
    mut collider_query: Query<(&mut Transform, &ColliderObject), With<Actor>>,
    transform_query: Query<(Entity, &Transform), (With<Transform>, Without<Actor>)>,
) {
    for (mut collider_transform, collider) in collider_query.iter_mut() {
        for (entity, entity_transform) in transform_query.iter() {
            if collider.attached_to_entity_id == entity.index() {
                collider_transform.translation = entity_transform.translation;
            }
        }
    }
}
