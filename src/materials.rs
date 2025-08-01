use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FresnelMaterial {
    #[uniform(0)]
    pub sharpness: Vec4, // Needed for WASM padding, only X is used
}

impl Material for FresnelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fresnel.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Debug, Clone, ShaderType)]
#[repr(C, align(16))]
pub struct LFPack {
    pub speed: f32,
    pub angle: f32,
    pub line_thickness: f32,
    pub layer_count: i32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LineFieldMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub edge_color: LinearRgba,
    #[uniform(2)]
    pub pack: LFPack,
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
pub struct MultiRippleRingMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub edge_color: LinearRgba,
}

impl Material for MultiRippleRingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/multi_ripple_ring.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RippleRingMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub edge_color: LinearRgba,
    #[uniform(2)]
    pub pack: Vec4,
    // duration = pack.x;
    // ring_thickness = pack.y;
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
pub struct HitSparkMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub mid_color: LinearRgba,
    #[uniform(2)]
    pub edge_color: LinearRgba,
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
pub struct BlockMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub edge_color: LinearRgba,
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
pub struct ClinkMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub edge_color: LinearRgba,
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
pub struct SpinnerMaterial {}

impl Material for SpinnerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/spinner.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FocalLineMaterial {}

impl Material for FocalLineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/focal_lines.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EdgeSlashMaterial {}

impl Material for EdgeSlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/edge_slash.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BurstMaterial {}

impl Material for BurstMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/burst.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CornerSlashMaterial {}

impl Material for CornerSlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/corner_slash.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LightningMaterial {}

impl Material for LightningMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lightning.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RocksMaterial {}

impl Material for RocksMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rocks.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Jackpot {}

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
pub struct RippleMaterial {}

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
pub struct FireMaterial {}

impl Material for FireMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fire.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SmokeBombMaterial {}

impl Material for SmokeBombMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/smoke_bomb.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SparksMaterial {}

impl Material for SparksMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sparks.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VertexTest {}

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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BezierMaterial {
    #[uniform(0)]
    pub control_points: [Vec4; 16],
    #[uniform(1)]
    pub curves: UVec4, // Padding for WASM, only X matters
    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,
}

impl Material for BezierMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bezier.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BezierSwooshMaterial {
    #[uniform(0)]
    pub control_points: [Vec4; 16],
    #[uniform(1)]
    pub curves: UVec4, // Padding for WASM, only X matters
}

impl Material for BezierSwooshMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bezier-swoosh.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NormalCubeMaterial {}

impl Material for NormalCubeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/normal-cube.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SugarCoatMaterial {}

impl Material for SugarCoatMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sugarcoat.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BillBurst {}

impl Material for BillBurst {
    fn fragment_shader() -> ShaderRef {
        "shaders/bill-burst.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
