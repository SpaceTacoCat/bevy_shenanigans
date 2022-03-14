use bevy::prelude::*;

pub struct AlterTransformOncePlugin;

#[derive(Component)]
pub struct AlterTransformOnce {
    pub target: Transform,
}

impl Plugin for AlterTransformOncePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_translation);
    }
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
