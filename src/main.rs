use std::f32::consts::PI;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

mod materials;
use materials::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            (
                MaterialPlugin::<FresnelMaterial>::default(),
                MaterialPlugin::<RippleRingMaterial>::default(),
                MaterialPlugin::<HitSparkMaterial>::default(),
                MaterialPlugin::<BlockMaterial>::default(),
                MaterialPlugin::<ClinkMaterial>::default(),
                MaterialPlugin::<LineFieldMaterial>::default(),
                MaterialPlugin::<SpinnerMaterial>::default(),
                MaterialPlugin::<FocalLineMaterial>::default(),
                MaterialPlugin::<LightningMaterial>::default(),
                MaterialPlugin::<CornerSlashMaterial>::default(),
                MaterialPlugin::<EdgeSlashMaterial>::default(),
                MaterialPlugin::<BurstMaterial>::default(),
                MaterialPlugin::<RocksMaterial>::default(),
                MaterialPlugin::<SparksMaterial>::default(),
                MaterialPlugin::<SmokeBombMaterial>::default(),
            ),
            (
                MaterialPlugin::<VertexTest>::default(),
                MaterialPlugin::<RippleMaterial>::default(),
                MaterialPlugin::<Jackpot>::default(),
                MaterialPlugin::<FireMaterial>::default(),
            ),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_meshes, update_selection))
        .run();
}

#[derive(Debug, Resource)]
struct Selected(usize);

#[derive(Debug, Component)]
struct Rotate;

#[derive(Debug, Component)]
struct Blank;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // This way to get around bevy system param limit
    (
        mut standard_materials,
        mut fresnel_materials,
        mut ripple_ring_materials,
        mut explosion_materials,
        mut block_materials,
        mut clink_materials,
        mut line_field_materials,
        mut spinner_materials,
        mut focal_line_materials,
        mut lightning_materials,
        mut corner_slash_materials,
        mut edge_slash_materials,
        mut burst_materials,
        mut rocks_materials,
        mut sparks_materials,
    ): (
        ResMut<Assets<StandardMaterial>>,
        ResMut<Assets<FresnelMaterial>>,
        ResMut<Assets<RippleRingMaterial>>,
        ResMut<Assets<HitSparkMaterial>>,
        ResMut<Assets<BlockMaterial>>,
        ResMut<Assets<ClinkMaterial>>,
        ResMut<Assets<LineFieldMaterial>>,
        ResMut<Assets<SpinnerMaterial>>,
        ResMut<Assets<FocalLineMaterial>>,
        ResMut<Assets<LightningMaterial>>,
        ResMut<Assets<CornerSlashMaterial>>,
        ResMut<Assets<EdgeSlashMaterial>>,
        ResMut<Assets<BurstMaterial>>,
        ResMut<Assets<RocksMaterial>>,
        ResMut<Assets<SparksMaterial>>,
    ),
    (
        mut vertex_material,
        mut ripple_material,
        mut jackpot_material,
        mut fire_material,
        mut smoke_bomb_materials,
    ): (
        ResMut<Assets<VertexTest>>,
        ResMut<Assets<RippleMaterial>>,
        ResMut<Assets<Jackpot>>,
        ResMut<Assets<FireMaterial>>,
        ResMut<Assets<SmokeBombMaterial>>,
    ),
) {
    commands.insert_resource(Selected(0));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        Visibility::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.0 / 8.0))),
        MeshMaterial3d(fresnel_materials.add(FresnelMaterial { sharpness: 2.0 })),
        Rotate,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(fire_material.add(FireMaterial {})),
    ));

    let cylinder_mesh = Cylinder::new(0.125, 0.25).mesh().without_caps().build();
    let cylinder_rotation = Quat::from_euler(EulerRot::XZX, PI / 4.0, 0.0, 0.0);
    commands.spawn((
        Mesh3d(meshes.add(cylinder_mesh.clone())),
        Transform::from_rotation(cylinder_rotation),
        MeshMaterial3d(jackpot_material.add(Jackpot {})),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(0.25, 0.25).subdivisions(20))),
        Transform::from_rotation(Quat::from_euler(EulerRot::XZX, PI / 4.0, 0.0, 0.0)),
        MeshMaterial3d(ripple_material.add(RippleMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(vertex_material.add(VertexTest {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(sparks_materials.add(SparksMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(smoke_bomb_materials.add(SmokeBombMaterial {})),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(rocks_materials.add(RocksMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(ripple_ring_materials.add(RippleRingMaterial {
            edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
            base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
            duration: 0.7,
            ring_thickness: 0.05,
        })),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(line_field_materials.add(LineFieldMaterial {
            edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
            base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
            speed: 1.0,
            angle: 0.0,
            line_thickness: 0.01,
            layer_count: 7,
        })),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(explosion_materials.add(HitSparkMaterial {
            edge_color: LinearRgba::rgb(1.0, 0.2, 0.05),
            mid_color: LinearRgba::rgb(1.0, 1.0, 0.1),
            base_color: LinearRgba::rgb(1.0, 1.0, 1.0),
        })),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(block_materials.add(BlockMaterial {
            edge_color: LinearRgba::rgb(0.1, 0.2, 1.0),
            base_color: LinearRgba::rgb(1.0, 1.0, 1.0),
            speed: 1.0,
        })),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(clink_materials.add(ClinkMaterial {
            edge_color: LinearRgba::rgb(0.9, 0.1, 0.9),
            base_color: LinearRgba::rgb(1.0, 0.5, 1.0),
            speed: 1.2,
        })),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(burst_materials.add(BurstMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(edge_slash_materials.add(EdgeSlashMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(corner_slash_materials.add(CornerSlashMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(lightning_materials.add(LightningMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(spinner_materials.add(SpinnerMaterial {})),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(focal_line_materials.add(FocalLineMaterial {})),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(standard_materials.add(StandardMaterial::default())),
        Blank,
    ));
}

fn rotate_meshes(mut mesh_query: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut tf in &mut mesh_query {
        tf.rotate_y(time.delta_secs());
        tf.rotate_x(0.5 * time.delta_secs());
    }
}

const ROW_SIZE: usize = 8;
const SQUARE_EDGE: f32 = 0.25;
const POS0: Vec3 = Vec3::new(-2.0, 1.0, 0.0);

fn update_selection(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut selection: ResMut<Selected>,
    mut meshes: Query<&mut Transform, (With<Mesh3d>, Without<Blank>)>,
    mut blanks: Query<&mut Transform, With<Blank>>,
) {
    let selectables = meshes.iter().count();
    let Selected(selected_index) = *selection;
    for event in keyboard_input_events.read() {
        if event.repeat || event.state == ButtonState::Released {
            continue;
        }

        match event.key_code {
            KeyCode::KeyD if selected_index < (selectables - 1) => selection.0 += 1,
            KeyCode::KeyD => selection.0 = 0,
            KeyCode::KeyA if selected_index == 0 => selection.0 = selectables - 1,
            KeyCode::KeyA => selection.0 -= 1,
            KeyCode::KeyW if selected_index < ROW_SIZE => {
                // Last item or same column on last row (wrap
                let rows = selectables / ROW_SIZE;
                let last_row_first_index = rows * ROW_SIZE;
                let selected_col = selected_index % ROW_SIZE;
                let same_col_last_row = last_row_first_index + selected_col;
                if same_col_last_row < selectables {
                    selection.0 = same_col_last_row;
                } else {
                    selection.0 = same_col_last_row - ROW_SIZE;
                }
            }
            KeyCode::KeyW => selection.0 -= ROW_SIZE,
            KeyCode::KeyS if selected_index > (selectables - ROW_SIZE - 1) => {
                selection.0 = selected_index % ROW_SIZE
            }
            KeyCode::KeyS => selection.0 += ROW_SIZE,
            _ => {}
        }
    }

    let Selected(new_selection) = *selection;

    for (index, mut tf) in meshes.iter_mut().enumerate() {
        let row = (index / ROW_SIZE) as f32;
        let col = (index % ROW_SIZE) as f32;
        let pos = POS0 + SQUARE_EDGE * Vec3::new(col, -row, 0.0);

        if index == new_selection {
            tf.translation = Vec3::new(1.0, 0.0, 0.0);
            tf.scale = Vec3::splat(6.0);

            let mut blank_tf = blanks.single_mut().unwrap();
            blank_tf.translation = pos;
        } else {
            tf.translation = pos;
            tf.scale = Vec3::splat(1.0);
        }
    }
}
