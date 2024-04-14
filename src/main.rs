mod game_engine;

pub use crate::game_engine::game_engine as game_lib;

use macroquad::prelude::*;

const WIDTH: f32 = 1240.0;
const HEIGHT: f32 = 650.0;

#[macroquad::main("Solitare")]
async fn main() {
    let solitare = game_lib::Game::new().await;
    let mut selected: Option<usize> = None;

    
    loop {
        request_new_screen_size(WIDTH, HEIGHT);

        if is_key_pressed(KeyCode::Escape) {
            break
        } 

        if is_mouse_button_pressed(MouseButton::Left) {
            selected = Some(mouse_position().0 as usize/150);
            println!("{:?}", selected);
        }

        clear_background(RED);

        draw_cards(&solitare).await;

        next_frame().await
    }
}

async fn draw_cards(game: &game_lib::Game) {
    for pile in 0..game.stack_piles.len() {
        for card in 0..game.stack_piles[pile].len() {

            if card == game.stack_piles[pile].len() - 1 {
                draw_texture(
                    &game.stack_piles[pile][card].texture,
                    pile as f32 * 150.0,
                    card as f32 * 30.0,
                    WHITE
                );
            } else {

                draw_texture(
                    &game.stack_piles[pile][card].back_texture,
                    pile as f32 * 150.0,
                    card as f32 * 30.0,
                    WHITE
                );
            }
            
        }
    }

}
