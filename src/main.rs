use std::f32::consts::PI;

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

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
            ),
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
    // This way to get around bevy system param limit
    (
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
        mut smoke_bomb_materials,
    ): (
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
        ResMut<Assets<SmokeBombMaterial>>,
    ),
    (mut vertex_material, mut ripple_material, mut jackpot_material): (
        ResMut<Assets<VertexTest>>,
        ResMut<Assets<RippleMaterial>>,
        ResMut<Assets<Jackpot>>,
    ),
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        Transform::from_xyz(0.0, -1.0, 0.0),
        MeshMaterial3d(fresnel_materials.add(FresnelMaterial { sharpness: 4.0 })),
        Rotate,
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        Transform::from_xyz(0.0, 1.0, 0.0),
        MeshMaterial3d(fresnel_materials.add(FresnelMaterial { sharpness: 2.0 })),
        Rotate,
    ));

    // camera
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            Visibility::default(),
        ))
        .with_children(|cb| {
            // Active

            // Column 3
            let cylinder_mesh = Cylinder::new(0.125, 0.25).mesh().without_caps().build();
            let cylinder_rotation = Quat::from_euler(EulerRot::XZX, PI / 4.0, 0.0, 0.0);
            cb.spawn((
                Mesh3d(meshes.add(cylinder_mesh.clone())),
                Transform::from_xyz(-0.75, -0.25, -2.0).with_rotation(cylinder_rotation),
                MeshMaterial3d(jackpot_material.add(Jackpot {})),
            ));

            cb.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(0.25, 0.25).subdivisions(20))),
                Transform::from_xyz(-0.75, 0.0, -2.0).with_rotation(Quat::from_euler(
                    EulerRot::XZX,
                    PI / 4.0,
                    0.0,
                    0.0,
                )),
                MeshMaterial3d(ripple_material.add(RippleMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-0.75, 0.25, -2.0),
                MeshMaterial3d(vertex_material.add(VertexTest {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-0.75, 0.75, -2.0),
                MeshMaterial3d(sparks_materials.add(SparksMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-0.75, 0.5, -2.0),
                MeshMaterial3d(smoke_bomb_materials.add(SmokeBombMaterial {})),
            ));

            // Column 2
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.0, 0.75, -2.0),
                MeshMaterial3d(rocks_materials.add(RocksMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.0, -0.5, -2.0),
                MeshMaterial3d(ripple_ring_materials.add(RippleRingMaterial {
                    edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                    base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
                    duration: 0.7,
                    ring_thickness: 0.05,
                })),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.0, -0.25, -2.0),
                MeshMaterial3d(line_field_materials.add(LineFieldMaterial {
                    edge_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                    base_color: LinearRgba::rgb(0.3, 1.0, 0.4),
                    speed: 1.0,
                    angle: 0.0,
                    line_thickness: 0.01,
                    layer_count: 7,
                })),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.0, 0.0, -2.0),
                MeshMaterial3d(explosion_materials.add(HitSparkMaterial {
                    edge_color: LinearRgba::rgb(1.0, 0.2, 0.05),
                    mid_color: LinearRgba::rgb(1.0, 1.0, 0.1),
                    base_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                })),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.0, 0.25, -2.0),
                MeshMaterial3d(block_materials.add(BlockMaterial {
                    edge_color: LinearRgba::rgb(0.1, 0.2, 1.0),
                    base_color: LinearRgba::rgb(1.0, 1.0, 1.0),
                    speed: 1.0,
                })),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.0, 0.5, -2.0),
                MeshMaterial3d(clink_materials.add(ClinkMaterial {
                    edge_color: LinearRgba::rgb(0.9, 0.1, 0.9),
                    base_color: LinearRgba::rgb(1.0, 0.5, 1.0),
                    speed: 1.2,
                })),
            ));

            // Column 1
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.25, 0.75, -2.0),
                MeshMaterial3d(burst_materials.add(BurstMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.25, 0.5, -2.0),
                MeshMaterial3d(edge_slash_materials.add(EdgeSlashMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.25, 0.25, -2.0),
                MeshMaterial3d(corner_slash_materials.add(CornerSlashMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.25, 0.0, -2.0),
                MeshMaterial3d(lightning_materials.add(LightningMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.25, -0.25, -2.0),
                MeshMaterial3d(spinner_materials.add(SpinnerMaterial {})),
            ));
            cb.spawn((
                Mesh3d(meshes.add(Rectangle::new(0.25, 0.25))),
                Transform::from_xyz(-1.25, -0.5, -2.0),
                MeshMaterial3d(focal_line_materials.add(FocalLineMaterial {})),
            ));
        });
}

fn rotate_meshes(mut mesh_query: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut tf in &mut mesh_query {
        tf.rotate_y(time.delta_secs());
        tf.rotate_x(0.5 * time.delta_secs());
    }
}

fn flicker_sizes(mut mesh_query: Query<&mut Transform, With<Flicker>>, time: Res<Time>) {
    for mut tf in &mut mesh_query {
        let mut scale = f32::sin(time.elapsed_secs() * 15.0).abs() + 0.3;
        if scale < 1.0 {
            scale = 0.0;
        }
        tf.scale = Vec3::ONE * scale;
    }
}

fn rotate_camera(mut cam_query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut cam_tf = cam_query.single_mut();
    let angle = time.elapsed_secs() * 0.01;
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
struct Jackpot {}

impl Material for Jackpot {
    fn vertex_shader() -> ShaderRef {
        "shaders/jackpot.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/jackpot.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct RippleMaterial {}

impl Material for RippleMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/ripple.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ripple.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SmokeBombMaterial {}

impl Material for SmokeBombMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/smoke_bomb.wgsl".into()
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct VertexTest {}

impl Material for VertexTest {
    fn vertex_shader() -> ShaderRef {
        "shaders/vertex.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/vertex.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
