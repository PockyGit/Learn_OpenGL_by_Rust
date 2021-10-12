#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{glutin, Surface};

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    //三角形列表
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //顶点着色器
    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    uniform mat4 matrix;

    void main() {
       gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#;

    //片元着色器
    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    //着色器程序
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    //定义计时器
    let mut t: f32 = -0.5;

    //事件循环
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        //更新计时器
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        //定义顶点
        let vertex1 = Vertex {
            position: [-0.5 + t, -0.5],
        };
        let vertex2 = Vertex {
            position: [0.0 + t, 0.5],
        };
        let vertex3 = Vertex {
            position: [0.5 + t, -0.25],
        };
        let shape = vec![vertex1, vertex2, vertex3];

        //顶点buffer
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

        //draw
        let mut target = display.draw();
        //背景颜色
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        //旋转矩阵
        let uniforms = uniform! {
            matrix: [
                [t.cos(), -t.sin(), 0.0, 0.0],
                [t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        //画三角形
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                //&glium::uniforms::EmptyUniforms,
                //&uniform! { t: t }, //uniform
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
