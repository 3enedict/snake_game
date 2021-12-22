use vgl::renderer::VglRenderer;
use vgl::renderer::core::parameters::VglRendererParameters;

use vgl::objects::vertex::Vertex;

fn system(renderer: &mut VglRenderer) {
    let mut triangles = vec!
        [
        Vertex { position: [ 0.55, -0.5 ] },
        Vertex { position: [ 0.55,  0.55] },
        Vertex { position: [-0.5 ,  0.55] },

        Vertex { position: [-0.55,  0.5 ] },
        Vertex { position: [-0.55, -0.55] },
        Vertex { position: [ 0.5 , -0.55] },
        ];

    renderer.add_triangles(&mut triangles);
}

fn main() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(system)
        .run();
}
