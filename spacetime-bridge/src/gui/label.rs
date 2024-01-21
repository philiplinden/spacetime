use bevy::prelude::*;

pub struct EntityLabelPlugin;
impl Plugin for EntityLabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, spawn_labels)
            .add_systems(Update, update_labels_position);
    }
}

#[derive(Component, Default)]
pub struct Labelled {
    pub style: TextStyle,
    pub offset: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct LabelEntity(Entity);

fn spawn_labels(
    mut commands: Commands,
    query_labelled: Query<(Entity, &Name, &Labelled), Added<Labelled>>,
) {
    for (entity, name, labelled) in &query_labelled {
        let id = commands
            .spawn(TextBundle::from_section(
                name.to_string(),
                labelled.style.clone(),
            ))
            .id();

        commands.entity(entity).insert(LabelEntity(id));
    }
}

fn update_labels_position(
    query_camera: Query<(&Camera, &GlobalTransform)>,
    query_labelled: Query<(&LabelEntity, &Labelled, &GlobalTransform)>,
    mut query_labels: Query<(&mut Style, &Node)>,
) {
    let (camera, camera_transform) = query_camera.single();

    for (entity, label, transform) in &query_labelled {
        let Ok((mut style, node)) = query_labels.get_mut(**entity) else {
            continue;
        };

        let rotation_matrix = Mat3::from_quat(camera_transform.to_scale_rotation_translation().1);
        let viewport_position = camera
            .world_to_viewport(
                camera_transform,
                transform.translation() + rotation_matrix.mul_vec3(label.offset.extend(0.0)),
            )
            .map(|position| position - node.size() / 2.0);

        if let Some(viewport_position) = viewport_position {
            style.position_type = PositionType::Absolute;
            style.left = Val::Px(viewport_position.x);
            style.top = Val::Px(viewport_position.y);
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}