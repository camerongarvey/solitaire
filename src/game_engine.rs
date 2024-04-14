pub mod game_engine {
    use macroquad::texture::load_texture;
    use rand::seq::SliceRandom;

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Card {
        pub value: String,
        pub suit: String,
        pub id: u8, 
        pub texture: macroquad::texture::Texture2D,
        pub back_texture: macroquad::texture::Texture2D,
    }

    pub struct Deck {
        pub deck: Vec<Card>,
    }

    pub struct Game {
        pub sorted_piles: Vec<Vec<Card>>,
        pub draw_pile: Vec<Vec<Card>>,
        pub stack_piles: Vec<Vec<Card>>,
    }
    
    impl Card {
        pub async fn new(value: String, suit: String, id: u8) -> Card {
 
            let texture = load_texture(("src/assets/".to_owned()+&suit+&value+".png").as_str()).await.unwrap();
            let back_texture = load_texture("src/assets/Back_green4.png").await.unwrap();

            Card {value, suit, id, texture, back_texture}
        }
    }

    impl Deck {
        pub async fn new() -> Deck {
            let suits = ["Spades", "Hearts", "Clubs", "Diamonds"];
            let values = ["A","2","3","4","5","6","7","8","9","10","J","Q","K"];
            let mut deck:Vec<Card> = Vec::new();
            for suit in 0..4 {
                for card in 0..13 {
                    deck.push(Card::new(values[card].to_string(), suits[suit].to_string(), (suit*13+card) as u8).await);
                }
            }

            Deck { deck: deck }

        }
    
        pub fn shuffle(&mut self) {
            self.deck.shuffle(&mut rand::thread_rng());
        }
    
    }

    impl Game{
        pub async fn new() -> Game {
            let mut deck = Deck::new().await;
            deck.shuffle();

            let mut object = Game {draw_pile: Vec::new(), sorted_piles: Vec::new(), stack_piles:Vec::new()};

            object.stack_piles = object.deal_cards(deck);

            object
            

            
        }
        fn deal_cards(&mut self, mut deck:Deck) -> Vec<Vec<Card>>{
            let mut dealt_cards = Vec::new();

            for i in 1..8 {
                let mut pile = Vec::new();
                for _ in 0..i {
                    pile.push(deck.deck.pop().unwrap())
                }
                dealt_cards.push(pile)
            }

            dealt_cards
        } 
    }
}