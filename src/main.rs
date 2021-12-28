use vgl::renderer::VglRenderer;
use vgl::renderer::core::parameters::VglRendererParameters;

use vgl::object::vertex::Vertex;

fn system(renderer: &mut VglRenderer) {
    let mut rectangle = vec!
        [
        Vertex{ position: [-0.5, -0.5] },
        Vertex{ position: [ 0.5, -0.5] },
        Vertex{ position: [ 0.5,  0.5] },
        Vertex{ position: [-0.5,  0.5] },
        ];

    let mut indices = vec![0, 1, 2, 2, 3, 0];

    renderer.add_rectangles(&mut rectangle, &mut indices);
}

fn main() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(system)
        .run();
}
