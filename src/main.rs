#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate cgmath;
extern crate genmesh;
extern crate image;
extern crate tobj;

mod pso;
mod obj;
mod util;

use gfx::{Bundle, texture};

use cgmath::{Point3, Vector3, Transform, AffineMatrix3};

use pso::{Locals, pipe};

use util::{load_texture, gen_cube};

struct App<R: gfx::Resources> {
  bundle: Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
  fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend, window_targets: gfx_app::WindowTargets<R>) -> Self {
    use gfx::traits::FactoryExt;
    
    let vs = gfx_app::shade::Source {
      glsl_150: include_bytes!("shader/cube_150.glslv"),
      hlsl_40: include_bytes!("data/vertex.fx"),
      msl_11: include_bytes!("shader/cube_vertex.metal"),
      vulkan: include_bytes!("data/vert.spv"),
      ..gfx_app::shade::Source::empty()
    };
    let ps = gfx_app::shade::Source {
      glsl_150: include_bytes!("shader/cube_150.glslf"),
      hlsl_40: include_bytes!("data/pixel.fx"),
      msl_11: include_bytes!("shader/cube_frag.metal"),
      vulkan: include_bytes!("data/frag.spv"),
      ..gfx_app::shade::Source::empty()
    };
    
    let (vertex_data, index_data) = obj::load_obj("assets/vector.obj");
    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data as &[u32]);
    
    //    let vertex_data = gen_cube();
    //    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());
    
    let texture_view = load_texture(factory, "assets/VectorUVMap.png").unwrap();
    
    let sinfo = texture::SamplerInfo::new(
      texture::FilterMethod::Bilinear,
      texture::WrapMode::Clamp);
    
    let pso = factory.create_pipeline_simple(
      vs.select(backend).unwrap(),
      ps.select(backend).unwrap(),
      pipe::new()
    ).unwrap();
    
    let proj = cgmath::perspective(cgmath::deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);
    
    let data = pipe::Data {
      vbuf: vbuf,
      transform: (proj * default_view().mat).into(),
      locals: factory.create_constant_buffer(1),
      color: (texture_view, factory.create_sampler(sinfo)),
      out_color: window_targets.color,
      out_depth: window_targets.depth,
    };
    
    App {
      bundle: Bundle::new(slice, pso, data),
    }
  }
  
  fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
    let locals = Locals { transform: self.bundle.data.transform };
    encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
    encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
    encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
    self.bundle.encode(encoder);
  }
  
  fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
    self.bundle.data.out_color = window_targets.color;
    self.bundle.data.out_depth = window_targets.depth;
    
    // In this example the transform is static except for window resizes.
    let proj = cgmath::perspective(cgmath::deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);
    self.bundle.data.transform = (proj * default_view().mat).into();
  }
}

pub fn main() {
  use gfx_app::Application;
  App::launch_simple("Cube example");
}

fn default_view() -> AffineMatrix3<f32> {
  Transform::look_at(
    Point3::new(10.0, 0.0, 1.0),
    Point3::new(0f32, 0.0, 0.0),
    Vector3::unit_z(),
  )
}
