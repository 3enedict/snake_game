use vgl::renderer::VglRenderer;

use vgl::objects::triangle::VglTriangle;
use vgl::objects::vertex::Vertex;

fn system(renderer: &mut VglRenderer) {
    let triangle = VglTriangle::new(
        [
        Vertex { position: [ 0.0, -0.5] },
        Vertex { position: [ 0.5,  0.5] },
        Vertex { position: [-0.5,  0.5] },
        ],
    );

    renderer.add_triangle(triangle);
}

fn main() {
    VglRenderer::new()
        .add_system_setup(system)
        .run();
}
