use crate::{layer, HitPoints, TextureHandles};
use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update.system().before("enemy_death"));
    }
}

struct Bullet {
    target: Entity,
    damage: u32,
    speed: f32,
}

pub fn spawn(
    mut position: Vec3,
    target: Entity,
    damage: u32,
    speed: f32,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    texture_handles: &Res<TextureHandles>,
) {
    position.z = layer::BULLET;

    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handles.bullet_shuriken.clone().into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(Bullet {
            target,
            damage,
            speed,
        });
}

fn update(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Bullet)>,
    mut target_query: Query<(&mut Transform, &mut HitPoints)>,
) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        if let Ok((target_transform, mut hp)) = target_query.get_mut(bullet.target) {
            let dist = transform
                .translation
                .truncate()
                .distance(target_transform.translation.truncate());

            let delta = time.delta_seconds();
            let step = bullet.speed * delta;

            if step < dist {
                transform.translation.x +=
                    step / dist * (target_transform.translation.x - transform.translation.x);
                transform.translation.y +=
                    step / dist * (target_transform.translation.y - transform.translation.y);

                // ten radians per second, clockwise
                transform.rotate(Quat::from_rotation_z(-10.0 * delta));
            } else {
                hp.current = hp.current.saturating_sub(bullet.damage);

                commands.despawn_recursive(entity);
            }
        } else {
            commands.despawn_recursive(entity);
        }
    }
}
