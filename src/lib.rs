pub mod basics;
pub mod contact;
pub mod forces;
pub mod shapes;

pub mod simulation {
    use macroquad::prelude::{WHITE, clear_background, next_frame};

    #[macroquad::main("Physics Simulation")]
    pub async fn main() {
        loop {
            clear_background(WHITE);

            next_frame().await;
        }
    }
}
