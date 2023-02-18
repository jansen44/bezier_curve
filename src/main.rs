use raylib::prelude::*;

const WIN_NAME: &str = "Bezier editor";
const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;
const WIN_FPS: u32 = 120;

const HANDLE_SIZE: f32 = 10.0;
const HANDLE_COLOR: Color = Color::RED;
const MARKER_SIZE: f32 = 2.0;
const MARKER_COLOR: Color = Color::BLUE;
const BEZIER_SIZE: f32 = 10.0;
const BEZIER_COLOR: Color = Color::GREEN;
const BEZIER_STEP: f32 = 0.005;

trait DrawCallsExt {
    fn draw_bezier_handle_v(&mut self, pos: Vector2);
    fn draw_bezier_item_v(&mut self, pos: Vector2);
    fn draw_bezier_lines(&mut self, pos: &[Vector2; 4]);
    fn draw_bezier(&mut self, poss: &[Vector2; 4]);

    fn debug_bezier_handlers(&mut self, rects: &[Vector2; 4]);
}

fn get_center(i: Vector2, size: f32) -> Vector2 {
    Vector2::new(i.x + size / 2.0, i.y + size / 2.0)
}

impl DrawCallsExt for RaylibDrawHandle<'_> {
    fn draw_bezier_handle_v(&mut self, pos: Vector2) {
        self.draw_rectangle_v(pos, Vector2::new(HANDLE_SIZE, HANDLE_SIZE), HANDLE_COLOR);
    }

    fn draw_bezier_item_v(&mut self, pos: Vector2) {
        self.draw_rectangle_v(pos, Vector2::new(BEZIER_SIZE, BEZIER_SIZE), BEZIER_COLOR);
    }

    fn draw_bezier_lines(&mut self, poss: &[Vector2; 4]) {
        self.draw_line_ex(
            get_center(poss[0], HANDLE_SIZE),
            get_center(poss[1], HANDLE_SIZE),
            MARKER_SIZE,
            MARKER_COLOR,
        );
        self.draw_line_ex(
            get_center(poss[2], HANDLE_SIZE),
            get_center(poss[3], HANDLE_SIZE),
            MARKER_SIZE,
            MARKER_COLOR,
        );
    }

    fn draw_bezier(&mut self, poss: &[Vector2; 4]) {
        if poss.len() < 4 {
            return;
        }

        self.draw_bezier_lines(poss);

        let mut step: f32 = 0.0;
        while step <= 10.0 {
            let i = poss[0];
            let j = poss[1];
            let k = poss[3];
            let l = poss[2];

            let normalized_step = (step.sin() + 1.0) / 2.0;
            let m1 = i.lerp(j, normalized_step);

            let m2 = j.lerp(k, normalized_step);
            let m3 = k.lerp(l, normalized_step);

            let bezier = m1.lerp(m2, normalized_step);
            let bezier = bezier.lerp(m3, normalized_step);

            self.draw_bezier_item_v(bezier);

            step += BEZIER_STEP;
        }
    }

    fn debug_bezier_handlers(&mut self, rects: &[Vector2; 4]) {
        self.draw_text("Try moving any red square", 10, 10, 20, Color::WHITE);
        for (i, r) in rects.iter().enumerate() {
            self.draw_text(
                &format!("0: {:?}", r),
                10,
                32 + (15 * i) as i32,
                15,
                Color::from_hex("999999").unwrap(),
            );
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title(WIN_NAME)
        .build();

    rl.set_target_fps(WIN_FPS);

    let mut rects = [
        Vector2::new(100.0, WIN_HEIGHT as f32 / 2.0),
        Vector2::new(100.0, WIN_HEIGHT as f32 / 2.0),
        Vector2::new(WIN_WIDTH as f32 - 100.0, WIN_HEIGHT as f32 / 2.0),
        Vector2::new(WIN_WIDTH as f32 - 100.0, WIN_HEIGHT as f32 / 2.0),
    ];
    let mut moving = -1;

    while !rl.window_should_close() {
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) && moving == -1 {
            let pos = rl.get_mouse_position();
            for (i, rect) in rects.iter().enumerate() {
                if pos.x >= rect.x
                    && pos.x <= rect.x + HANDLE_SIZE
                    && pos.y >= rect.y
                    && pos.y <= rect.y + HANDLE_SIZE
                {
                    moving = i as i32;
                }
            }
        }
        if rl.is_mouse_button_up(MouseButton::MOUSE_LEFT_BUTTON) {
            moving = -1;
        }
        if moving != -1 {
            let pos = rl.get_mouse_position();
            rects[moving as usize] =
                Vector2::new(pos.x - HANDLE_SIZE / 2.0, pos.y - HANDLE_SIZE / 2.0);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_bezier(&rects);
        rects.iter().for_each(|&pos| d.draw_bezier_handle_v(pos));
        d.debug_bezier_handlers(&rects);
    }
}
