#[macro_use]
extern crate gfx;

macro_rules! gfx_defines_t {
    ($(#[$attr:meta])* vertex # $name:ident {
            $( $field:ident : $ty:ty = $e:expr, )+
    }) => {
        gfx_vertex_struct_meta!($(#[$attr])* vertex_struct_meta $name {$($field:$ty = $e,)+});
    };
}

gfx_defines_t!{vertex#Vertex{pos:[f32;4]="a_Pos",tex_coord:[f32;2]="a_TexCoord",}}
