use bevy::prelude::*;

#[derive(Component)]
pub struct AlterTransformOnce {
    pub target: Transform,
}

pub fn init_translation(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &AlterTransformOnce)>,
) {
    for (entity, mut transform, rm) in q.iter_mut() {
        *transform = rm.target;
        commands.entity(entity).remove::<AlterTransformOnce>();
    }
}
