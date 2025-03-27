use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64::consts::PI;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Game {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    player: Player,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
    score: u32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Game {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let player = Player::new(
            canvas.width() as f64 / 2.0,
            canvas.height() as f64 / 2.0,
        );

        let mut asteroids = Vec::new();
        for _ in 0..5 {
            asteroids.push(Asteroid::new(
                rand::random::<f64>() * canvas.width() as f64,
                rand::random::<f64>() * canvas.height() as f64,
            ));
        }

        Game {
            canvas,
            context,
            player,
            asteroids,
            bullets: Vec::new(),
            score: 0,
        }
    }

    pub fn update(&mut self) {
        self.player.update();
        
        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
        }
        
        // Update asteroids
        for asteroid in &mut self.asteroids {
            asteroid.update();
        }

        // Remove bullets that are off screen
        self.bullets.retain(|bullet| {
            bullet.x >= 0.0
                && bullet.x <= self.canvas.width() as f64
                && bullet.y >= 0.0
                && bullet.y <= self.canvas.height() as f64
        });

        // Check collisions
        self.check_collisions();
    }

    pub fn render(&self) {
        // Clear canvas
        self.context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        
        // Draw player
        self.player.draw(&self.context);
        
        // Draw asteroids
        for asteroid in &self.asteroids {
            asteroid.draw(&self.context);
        }
        
        // Draw bullets
        for bullet in &self.bullets {
            bullet.draw(&self.context);
        }
    }

    pub fn shoot(&mut self) {
        self.bullets.push(self.player.shoot());
    }

    pub fn rotate_left(&mut self) {
        self.player.rotate(-0.1);
    }

    pub fn rotate_right(&mut self) {
        self.player.rotate(0.1);
    }

    pub fn thrust(&mut self) {
        self.player.thrust();
    }

    fn check_collisions(&mut self) {
        // Check bullet-asteroid collisions
        let mut i = 0;
        while i < self.bullets.len() {
            let mut j = 0;
            while j < self.asteroids.len() {
                if self.bullets[i].collides_with(&self.asteroids[j]) {
                    self.bullets.remove(i);
                    self.asteroids.remove(j);
                    self.score += 100;
                    break;
                }
                j += 1;
            }
            if i < self.bullets.len() {
                i += 1;
            }
        }
    }
}

struct Player {
    x: f64,
    y: f64,
    angle: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Player {
    fn new(x: f64, y: f64) -> Player {
        Player {
            x,
            y,
            angle: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }

    fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.save();
        context.translate(self.x, self.y).unwrap();
        context.rotate(self.angle).unwrap();
        
        context.begin_path();
        context.move_to(0.0, -20.0);
        context.line_to(10.0, 10.0);
        context.line_to(-10.0, 10.0);
        context.close_path();
        
        context.set_stroke_style(&JsValue::from_str("white"));
        context.stroke();
        
        context.restore();
    }

    fn rotate(&mut self, angle: f64) {
        self.angle += angle;
    }

    fn thrust(&mut self) {
        self.velocity_x += self.angle.sin() * 0.5;
        self.velocity_y -= self.angle.cos() * 0.5;
    }

    fn shoot(&self) -> Bullet {
        Bullet {
            x: self.x + self.angle.sin() * 20.0,
            y: self.y - self.angle.cos() * 20.0,
            velocity_x: self.angle.sin() * 10.0,
            velocity_y: -self.angle.cos() * 10.0,
        }
    }
}

struct Asteroid {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    size: f64,
}

impl Asteroid {
    fn new(x: f64, y: f64) -> Asteroid {
        Asteroid {
            x,
            y,
            velocity_x: (rand::random::<f64>() - 0.5) * 2.0,
            velocity_y: (rand::random::<f64>() - 0.5) * 2.0,
            size: 20.0,
        }
    }

    fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.arc(self.x, self.y, self.size, 0.0, PI * 2.0).unwrap();
        context.set_stroke_style(&JsValue::from_str("white"));
        context.stroke();
    }
}

struct Bullet {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Bullet {
    fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.arc(self.x, self.y, 2.0, 0.0, PI * 2.0).unwrap();
        context.set_fill_style(&JsValue::from_str("white"));
        context.fill();
    }

    fn collides_with(&self, asteroid: &Asteroid) -> bool {
        let dx = self.x - asteroid.x;
        let dy = self.y - asteroid.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < asteroid.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_player_creation() {
        let player = Player::new(100.0, 100.0);
        assert_eq!(player.x, 100.0);
        assert_eq!(player.y, 100.0);
        assert_eq!(player.angle, 0.0);
        assert_eq!(player.velocity_x, 0.0);
        assert_eq!(player.velocity_y, 0.0);
    }

    #[wasm_bindgen_test]
    fn test_player_rotation() {
        let mut player = Player::new(100.0, 100.0);
        player.rotate(0.5);
        assert!((player.angle - 0.5).abs() < 0.0001);
    }

    #[wasm_bindgen_test]
    fn test_player_thrust() {
        let mut player = Player::new(100.0, 100.0);
        player.thrust();
        assert!(player.velocity_x != 0.0 || player.velocity_y != 0.0);
    }

    #[wasm_bindgen_test]
    fn test_asteroid_creation() {
        let asteroid = Asteroid::new(100.0, 100.0);
        assert_eq!(asteroid.x, 100.0);
        assert_eq!(asteroid.y, 100.0);
        assert_eq!(asteroid.size, 20.0);
        assert!(asteroid.velocity_x.abs() <= 1.0);
        assert!(asteroid.velocity_y.abs() <= 1.0);
    }

    #[wasm_bindgen_test]
    fn test_asteroid_movement() {
        let mut asteroid = Asteroid::new(100.0, 100.0);
        let initial_x = asteroid.x;
        let initial_y = asteroid.y;
        asteroid.update();
        assert!(asteroid.x != initial_x || asteroid.y != initial_y);
    }

    #[wasm_bindgen_test]
    fn test_bullet_creation() {
        let player = Player::new(100.0, 100.0);
        let bullet = player.shoot();
        assert!(bullet.x > 0.0);
        assert!(bullet.y > 0.0);
        assert!(bullet.velocity_x != 0.0 || bullet.velocity_y != 0.0);
    }

    #[wasm_bindgen_test]
    fn test_bullet_movement() {
        let mut bullet = Bullet {
            x: 100.0,
            y: 100.0,
            velocity_x: 1.0,
            velocity_y: 1.0,
        };
        let initial_x = bullet.x;
        let initial_y = bullet.y;
        bullet.update();
        assert_eq!(bullet.x, initial_x + 1.0);
        assert_eq!(bullet.y, initial_y + 1.0);
    }

    #[wasm_bindgen_test]
    fn test_collision_detection() {
        let bullet = Bullet {
            x: 100.0,
            y: 100.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        };
        let asteroid = Asteroid {
            x: 100.0,
            y: 100.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            size: 20.0,
        };
        assert!(bullet.collides_with(&asteroid));

        let bullet = Bullet {
            x: 150.0,
            y: 150.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        };
        assert!(!bullet.collides_with(&asteroid));
    }
}
