use crate::rock_paper_scissors::Action::{PAPER, ROCK, SCISSORS};



struct Player {
    points:i32,
    name:String,
}

#[derive(PartialEq, Debug)]
enum Action {
    ROCK,
    PAPER,
    SCISSORS
}

impl Action {
    fn beats(self, action_b: Action) -> bool {
        match(self, action_b){
            (ROCK,SCISSORS) | (SCISSORS, PAPER) | (PAPER,ROCK) => true,
            _ => false
        }
    }
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            points:0,
        }
    }

    fn fight(&mut self, action_a:Action, action_b:Action, player_b: &mut Player) {


        println!("{:?} vs {:?} !",action_a,action_b);;
        if action_a == action_b {
            println!("Egalité !");
            println!("Scores : {} / {} !",self.points,player_b.points);;
            return;
        }

        if action_a.beats(action_b) {
            println!("{} a battu {} !",self.name,player_b.name);
            self.points += 1;
            println!("Scores : {} / {} !",self.points,player_b.points);;
            return;
        }

        println!("{} a battu {} !",player_b.name,self.name);
        player_b.points += 1;
        println!("Scores : {} / {} !",self.points,player_b.points);

    }


}

pub(crate) fn play() {

    let mut player_a : Player = Player::new("Yro");
    let mut player_b : Player = Player::new("Proutix");

    player_a.fight(SCISSORS,ROCK, &mut player_b);
    player_a.fight(SCISSORS,PAPER, &mut player_b);

}