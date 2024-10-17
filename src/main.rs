use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
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
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_meshes, rotate_camera, flicker_sizes))
        .run();
}

#[derive(Debug, Component)]
struct Rotate;

#[derive(Debug, Component)]
struct Flicker;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fresnel_materials: ResMut<Assets<FresnelMaterial>>,
    mut ripple_ring_materials: ResMut<Assets<RippleRingMaterial>>,
    mut explosion_materials: ResMut<Assets<HitSparkMaterial>>,
    mut block_materials: ResMut<Assets<BlockMaterial>>,
    mut clink_materials: ResMut<Assets<ClinkMaterial>>,
    mut line_field_materials: ResMut<Assets<LineFieldMaterial>>,
    mut spinner_materials: ResMut<Assets<SpinnerMaterial>>,
    mut focal_line_materials: ResMut<Assets<FocalLineMaterial>>,
    mut lightning_materials: ResMut<Assets<LightningMaterial>>,
    mut corner_slash_materials: ResMut<Assets<CornerSlashMaterial>>,
    mut edge_slash_materials: ResMut<Assets<EdgeSlashMaterial>>,
    mut burst_materials: ResMut<Assets<BurstMaterial>>,
    mut rocks_materials: ResMut<Assets<RocksMaterial>>,
    mut sparks_materials: ResMut<Assets<SparksMaterial>>,
) {
    // cube
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Sphere::default()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            material: fresnel_materials.add(FresnelMaterial { sharpness: 4.0 }),
            ..default()
        },
        Rotate,
    ));

    // cube
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Cuboid::default()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            material: fresnel_materials.add(FresnelMaterial { sharpness: 2.0 }),
            ..default()
        },
        Rotate,
    ));

    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            VisibilityBundle::default(),
        ))
        .with_children(|cb| {
            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(1.0, 1.0)),
                transform: Transform::from_xyz(1.0, 0.0, -2.0),
                material: sparks_materials.add(SparksMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.25, 0.75, -2.0),
                material: burst_materials.add(BurstMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.25, 0.5, -2.0),
                material: edge_slash_materials.add(EdgeSlashMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.25, 0.25, -2.0),
                material: corner_slash_materials.add(CornerSlashMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.25, 0.0, -2.0),
                material: lightning_materials.add(LightningMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.25, -0.25, -2.0),
                material: spinner_materials.add(SpinnerMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.25, -0.5, -2.0),
                material: focal_line_materials.add(FocalLineMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.00, 0.75, -2.0),
                material: rocks_materials.add(RocksMaterial {}),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.0, -0.5, -2.0),
                material: ripple_ring_materials.add(RippleRingMaterial {
                    edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                    base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
                    duration: 0.7,
                    ring_thickness: 0.05,
                }),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.0, -0.25, -2.0),
                material: line_field_materials.add(LineFieldMaterial {
                    edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                    base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
                    speed: 1.0,
                    angle: 0.0,
                    line_thickness: 0.01,
                    layer_count: 7,
                }),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.0, 0.0, -2.0),
                material: explosion_materials.add(HitSparkMaterial {
                    edge_color: LinearRgba::rgb(1.0, 0.2, 0.05),
                    mid_color: LinearRgba::rgb(1.0, 1.0, 0.1),
                    base_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                }),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.0, 0.25, -2.0),
                material: block_materials.add(BlockMaterial {
                    edge_color: LinearRgba::rgb(0.1, 0.2, 1.0),
                    base_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                    speed: 1.0,
                }),
                ..default()
            });

            cb.spawn(MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(0.25, 0.25)),
                transform: Transform::from_xyz(-1.0, 0.5, -2.0),
                material: clink_materials.add(ClinkMaterial {
                    edge_color: LinearRgba::rgb(0.9, 0.1, 0.9),
                    base_color: LinearRgba::rgb(1.0, 0.5, 1.0),
                    speed: 1.2,
                }),
                ..default()
            });
        });
}

fn rotate_meshes(mut mesh_query: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut tf in &mut mesh_query {
        tf.rotate_y(time.delta_seconds());
        tf.rotate_x(0.5 * time.delta_seconds());
    }
}

fn flicker_sizes(mut mesh_query: Query<&mut Transform, With<Flicker>>, time: Res<Time>) {
    for mut tf in &mut mesh_query {
        let mut scale = f32::sin(time.elapsed_seconds() * 15.0).abs() + 0.3;
        if scale < 1.0 {
            scale = 0.0;
        }
        tf.scale = Vec3::ONE * scale;
    }
}

fn rotate_camera(mut cam_query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut cam_tf = cam_query.single_mut();
    let angle = time.elapsed_seconds() * 0.01;
    let distance = 5.0;
    *cam_tf = Transform::from_xyz(0.0, distance * f32::sin(angle), distance * f32::cos(angle))
        .looking_at(Vec3::ZERO, Vec3::Y);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FresnelMaterial {
    #[uniform(0)]
    sharpness: f32,
}

impl Material for FresnelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fresnel.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct LineFieldMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    edge_color: LinearRgba,
    #[uniform(2)]
    speed: f32,
    #[uniform(3)]
    angle: f32,
    #[uniform(4)]
    line_thickness: f32,
    #[uniform(5)]
    layer_count: i32,
}

impl Material for LineFieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_field.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct RippleRingMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    edge_color: LinearRgba,
    #[uniform(2)]
    duration: f32,
    #[uniform(3)]
    ring_thickness: f32,
}

impl Material for RippleRingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ripple_ring.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct HitSparkMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    mid_color: LinearRgba,
    #[uniform(2)]
    edge_color: LinearRgba,
}

impl Material for HitSparkMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hitspark.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct BlockMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    edge_color: LinearRgba,
    #[uniform(2)]
    speed: f32,
}

impl Material for BlockMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blocking.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ClinkMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    edge_color: LinearRgba,
    #[uniform(2)]
    speed: f32,
}

impl Material for ClinkMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/clink.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SpinnerMaterial {}

impl Material for SpinnerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/spinner.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FocalLineMaterial {}

impl Material for FocalLineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/focal_lines.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct EdgeSlashMaterial {}

impl Material for EdgeSlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/edge_slash.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct BurstMaterial {}

impl Material for BurstMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/burst.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CornerSlashMaterial {}

impl Material for CornerSlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/corner_slash.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct LightningMaterial {}

impl Material for LightningMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lightning.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct RocksMaterial {}

impl Material for RocksMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rocks.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SparksMaterial {}

impl Material for SparksMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sparks.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
