mod framebuffer;
mod texture;
mod color;
mod model;

use framebuffer::FrameBuffer;
use model::Model;

fn main() {
    let mut framebuffer = FrameBuffer::new(800, 600);
    let model = Model::load("ruta/del/modelo.obj");

    model.render(&mut framebuffer);
}

