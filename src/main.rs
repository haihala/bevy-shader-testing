use std::f32::consts::PI;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::WindowMode,
};

mod materials;
use materials::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
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
                MaterialPlugin::<MultiRippleRingMaterial>::default(),
                MaterialPlugin::<BezierMaterial>::default(),
                MaterialPlugin::<BezierSwooshMaterial>::default(),
                MaterialPlugin::<NormalCubeMaterial>::default(),
                MaterialPlugin::<SugarCoatMaterial>::default(),
                MaterialPlugin::<BillBurst>::default(),
            ),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                rotate_meshes,
                update_selection,
                keyboard_system,
                button_system,
            ),
        )
        .run();
}

#[derive(Debug, Resource)]
struct Selected(usize);

#[derive(Debug, Component)]
struct Rotate;

#[derive(Debug, Component)]
struct Blank;

#[allow(clippy::type_complexity)]
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
        mut smoke_bomb_material,
        mut multi_ripple_ring_material,
        mut bezier_material,
        mut bezier_swoosh_material,
        mut normal_cube_material,
        mut sugarcoat_material,
        mut bill_burst_material,
    ): (
        ResMut<Assets<VertexTest>>,
        ResMut<Assets<RippleMaterial>>,
        ResMut<Assets<Jackpot>>,
        ResMut<Assets<FireMaterial>>,
        ResMut<Assets<SmokeBombMaterial>>,
        ResMut<Assets<MultiRippleRingMaterial>>,
        ResMut<Assets<BezierMaterial>>,
        ResMut<Assets<BezierSwooshMaterial>>,
        ResMut<Assets<NormalCubeMaterial>>,
        ResMut<Assets<SugarCoatMaterial>>,
        ResMut<Assets<BillBurst>>,
    ),
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Selected(0));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(bill_burst_material.add(BillBurst {})),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(sugarcoat_material.add(SugarCoatMaterial {})),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(normal_cube_material.add(NormalCubeMaterial {})),
    ));

    // Even though we don't use the fourth dimension, Bevy wants them as 4d
    let bezier_swoosh_control_points = vec![
        Vec3::new(-0.7, 0.9, 2.0),
        Vec3::new(1.2, 1.0, 8.0),
        Vec3::new(-0.2, -0.9, 6.0),
        Vec3::new(-0.9, 0.7, 0.0),
    ];
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(
            bezier_swoosh_material.add(BezierSwooshMaterial {
                control_points: pad_to(bezier_swoosh_control_points, 16)
                    .as_slice()
                    .try_into()
                    .unwrap(),
                curves: UVec4::splat(1),
            }),
        ),
    ));

    // Even though we don't use the fourth dimension, Bevy wants them as 4d
    let bezier_control_points = vec![
        // Set 1
        Vec3::new(-0.9, -0.9, 2.0),
        Vec3::new(1.0, 1.0, 10.0),
        Vec3::new(-2.0, 1.5, 10.0),
        Vec3::new(-0.2, -0.2, 5.0),
        // Set 2 (first anchor is assumed to be the last point of previous curve)
        // Think of it as auto-continuity
        Vec3::new(1.6, -1.9, 10.0),
        Vec3::new(-3.0, 0.2, 10.0),
        Vec3::new(0.9, 0.0, 2.0),
    ];
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(
            bezier_material.add(BezierMaterial {
                control_points: pad_to(bezier_control_points, 16)
                    .as_slice()
                    .try_into()
                    .unwrap(),
                curves: UVec4::splat(2),

                texture: Some(asset_server.load("pictures/smiley.png")),
            }),
        ),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(multi_ripple_ring_material.add(MultiRippleRingMaterial {
            edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
            base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
        })),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.0 / 8.0))),
        MeshMaterial3d(fresnel_materials.add(FresnelMaterial {
            sharpness: Vec4::splat(2.0),
        })),
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
        MeshMaterial3d(smoke_bomb_material.add(SmokeBombMaterial {})),
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
            pack: Vec4::new(
                0.7,  // duration
                0.05, // ring_thickness
                0.0, 0.0, // Padding for WASM
            ),
        })),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(line_field_materials.add(LineFieldMaterial {
            edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
            base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
            pack: LFPack {
                speed: 1.0,
                angle: 0.0,
                line_thickness: 0.01,
                layer_count: 7,
            },
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
        })),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
        MeshMaterial3d(clink_materials.add(ClinkMaterial {
            edge_color: LinearRgba::rgb(0.9, 0.1, 0.9),
            base_color: LinearRgba::rgb(1.0, 0.5, 1.0),
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

    commands.spawn((
        Text::new("Select shader with W/A/S/D or the buttons"),
        Node {
            // Pad it out a bit
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Percent(1.0)),
            ..default()
        },
        children![button(-1, "previous"), button(1, "next")],
    ));
}

const BORDER_COLOR: Color = Color::linear_rgb(0.1, 0.1, 0.1);
const NORMAL_BUTTON: Color = Color::WHITE;
const HOVERED_BUTTON: Color = Color::linear_rgb(0.6, 0.6, 0.6);
const PRESSED_BUTTON: Color = Color::linear_rgb(0.4, 0.5, 0.8);

#[derive(Debug, Component)]
struct ButtonDelta(i32);

fn button(delta: i32, text: &'static str) -> impl Bundle + use<> {
    (
        Button,
        ButtonDelta(delta),
        Node {
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(3.0)),

            // Center the text
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BORDER_COLOR),
        BorderRadius::all(Val::Px(3.0)),
        children![Text::new(text), TextColor(NORMAL_BUTTON)],
    )
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

#[allow(clippy::type_complexity)]
fn button_system(
    interaction_query: Query<
        (&Interaction, &ButtonDelta, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_cols: Query<&mut TextColor>,
    mut selection: ResMut<Selected>,
    meshes: Query<&Transform, (With<Mesh3d>, Without<Blank>)>,
) {
    let selectables = meshes.iter().count();

    for (interaction, delta, children) in &interaction_query {
        let mut color = text_cols.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                let Selected(index) = *selection;
                *selection = Selected(((index as i32 + delta.0) % selectables as i32) as usize);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
fn keyboard_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut selection: ResMut<Selected>,
    meshes: Query<&Transform, (With<Mesh3d>, Without<Blank>)>,
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
}

fn update_selection(
    selection: ResMut<Selected>,
    mut meshes: Query<&mut Transform, (With<Mesh3d>, Without<Blank>)>,
    mut blanks: Query<&mut Transform, With<Blank>>,
) {
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

fn pad_to(input: Vec<Vec3>, desired_len: usize) -> Vec<Vec4> {
    let padding = std::iter::repeat_n(Vec3::default(), desired_len - input.len());
    input
        .into_iter()
        .chain(padding)
        .map(|v| v.extend(0.0))
        .collect::<Vec<_>>()
}
