use vgl::renderer::VglRenderer;
use vgl::renderer::core::parameters::VglRendererParameters;

use vgl::object::vertex::Vertex;

fn system(renderer: &mut VglRenderer) {
    let mut square = vec!
        [
        Vertex { position: [ 0.0,  0.0] },
        ];

    let mut sizes = vec![0.1];

    renderer.add_squares(&mut square, &mut sizes);
}

fn main() {
    VglRenderer::new(VglRendererParameters::default())
        .add_system_setup(system)
        .run();
}
