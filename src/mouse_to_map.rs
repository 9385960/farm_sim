use bevy::{prelude::*, render::camera::Projection};

pub fn handle_mouse_clicks(mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>,camera : Query<&Camera>, camera_transform : Query<&GlobalTransform,With<Camera>>) {
    let win = windows.get_primary().expect("no primary window");
    let camera = camera.get_single().expect("Too many cameras");
    let camera_transform = camera_transform.get_single().expect("EEEE");
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("click at {:?}", win.cursor_position());
        let mouse_position = match win.cursor_position() {
            Some(vec) => vec,
            None => panic!("Help"),
        }; 
        let x = 2.0 * (mouse_position[0] / win.width() as f32) - 1.0;
        let y = 2.0 * (mouse_position[1] / win.height() as f32) - 1.0;

        let camera_inverse_matrix =
           camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let near = camera_inverse_matrix * Vec3::new(x, y, 0.0).extend(1.0);
        let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

       let near = near.truncate() / near.w;
        println!("camera found, width : {}",near);
    }
}
