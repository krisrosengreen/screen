use std::any::Any;

pub const SCREEN_SIZE_X: usize = 150;
pub const SCREEN_SIZE_Y: usize = 45;
pub const Y_SQUISH: f32 = 0.6;

pub const DELTA_TIME: f32 = 1.0 / 60.0;
pub const TIME_SPEED: f64 = 1.0;

pub fn screen_loop(mut screen: Screen, mut time: Time) {
    let time_delta: f64 = time.delta_time() as f64;

    #[cfg(feature = "fps")]
    let mut time_last = std::time::Instant::now();
    let mut time_last_sleep = std::time::Instant::now();

    loop {
        time.tick();
        screen.tick(&time);
        screen.draw();

        let calc_time = time_last_sleep.elapsed().as_secs_f64();

        std::thread::sleep(std::time::Duration::from_secs_f64(
            (time_delta - calc_time).max(0.0) * TIME_SPEED,
        ));
        
        screen.clear();
        time_last_sleep = std::time::Instant::now();

        #[cfg(feature = "fps")]
        {
        // For fps
        let difference = time_last.elapsed();
        time_last = std::time::Instant::now();
        println!("fps {}", 1000.0 / (difference.as_millis() as f32));
        }
    }
}

pub trait Drawable: Any {
    fn as_any(&self) -> &dyn Any;
    
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Handler {
    fn tick(&mut self, objs: &mut Vec<Box<dyn Drawable>>, time: &Time);

    fn get_screen(&mut self) -> [[char; SCREEN_SIZE_X]; SCREEN_SIZE_Y];

    fn clear(&mut self);
}

pub struct Time {
    pub current_time: f32,
}

impl Time {
    pub fn tick(&mut self) {
        self.current_time += self.delta_time();
    }

    pub fn delta_time(&self) -> f32 {
        DELTA_TIME
    }
}

pub struct Screen {
    pub position_x: f32,
    pub position_y: f32,
    pub scale: f32,
    objs: Vec<Box<dyn Drawable>>,
    handler: Box<dyn Handler>,
}

impl Screen {
    pub fn new(handler: Box<dyn Handler>) -> Self {
        Screen {
            objs: Vec::new(),
            position_x: 0.0,
            position_y: 0.0,
            scale: 1.0,
            handler: handler,
        }
    }

    pub fn draw(&mut self) {
        let screen = self.handler.get_screen();
        for y_i in 0..SCREEN_SIZE_Y {
            for x_i in 0..SCREEN_SIZE_X {
                print!("{}", screen[y_i][x_i]);
            }

            // Print new line
            println!("");
        }
    }

    pub fn clear(&mut self) {
        self.handler.clear();
        print!("\x1b[H");
    }

    pub fn add_body(&mut self, body: Box<dyn Drawable>) {
        self.objs.push(body);
    }

    fn tick(&mut self, time: &Time) {
        self.handler.tick(&mut self.objs, time);
    }
}
