use gfx;
use image;

use std::path::Path;

use gfx_app::ColorFormat;

use genmesh::generators::Cube;
use genmesh::{Quad, MapToVertices, Triangulate, Vertices};

use pso::Vertex;

pub fn gen_cube() -> Vec<Vertex> {
  Cube::new()
    .vertex(|(x, y, z)| Vertex::new([x, y, z, 1.], [0., 0.]))
    .map(|Quad { x: v0, y: v1, z: v2, w: v3 }| {
      Quad::new(Vertex::new(v0.pos, [0., 0.]),
                Vertex::new(v1.pos, [1., 0.]),
                Vertex::new(v2.pos, [1., 1.]),
                Vertex::new(v3.pos, [0., 1.]))
    })
    .triangulate()
    .vertices()
    .collect()
}

pub fn load_texture<R, F>(factory: &mut F, path: &str)
                          -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>
  where R: gfx::Resources, F: gfx::Factory<R> {
  use gfx::texture as t;
  
  let img = image::open(&Path::new(path)).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
  let (_, view) = factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&img]).unwrap();
  Ok(view)
}
