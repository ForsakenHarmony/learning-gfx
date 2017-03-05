use tobj;

use std::path::Path;

use pso::Vertex;

pub fn load_obj<'a>(path: &str) -> (Vec<Vertex>, Vec<u32>) {
  let vector_obj = tobj::load_obj(&Path::new(path));
  let (models, _) = vector_obj.unwrap();
  
  let mut vertex_data: Vec<Vertex> = Vec::new();
  
  let ref m: tobj::Model = models[5];
  let mesh: &tobj::Mesh = &m.mesh;
  
  let index_data = mesh.indices.clone();
  
  for v in 0..mesh.positions.len() / 3 {
    let (x, y, z) = (
      mesh.positions[3 * v],
      mesh.positions[3 * v + 1],
      mesh.positions[3 * v + 2]
    );
    
    let (tex_x, tex_y) = (
      mesh.texcoords[2 * v],
      1.0 - mesh.texcoords[2 * v + 1]
    );
    
    vertex_data.push(Vertex::new(
      [x, z, y, 1.0],
      [tex_x, tex_y],
    ));
  }
  
  (vertex_data, index_data)
}
