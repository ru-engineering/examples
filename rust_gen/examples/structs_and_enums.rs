use std::io; //for input of name and stuff
use std::fmt; //allows us to print enums as messages
#[derive(PartialEq)] //alows doing enum == enum
#[derive(Copy, Clone)] //allows sending pointed enums to function, try commenting out to see why
enum Spells {
    Fireball,
    Heal,
    Thunder,
    None,
}

impl fmt::Display for Spells{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        match *self{
            Spells::Fireball => {write!(f,"Fireball")}
            Spells::Heal => {write!(f,"Heal")}
            Spells::Thunder => {write!(f,"Thunder")}
            Spells::None => {write!(f,"None")}
        }
    }
}

struct Player {
    name : String,
    hit_points : u8,
    spells_known : [Spells;3],
}

struct Monster{
    hit_points : u8,
    damage_per_turn : u8,
}


fn main(){
    let mut me = Player::new();
    let mut name = String::new();
    println!("Hello Adventurer, what is your name");
    io::stdin().read_line(&mut name).expect("failed to read name");
    me.set_name(&mut name).expect("naming failed");
    println!("Choose 2 spells
    Spells Avialable are\n
            1 : {}
            2 : {}
            3 : {}\n
    pick the first by typing the number and then enter",Spells::Fireball,Spells::Heal,Spells::Thunder);

    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).expect("failed to read spell");
    s1 = s1.trim_end_matches('\n').to_string();

    let s1_out : i8 = s1.parse().expect("Ummm you cant do that");
    match s1_out {
        1 =>{me.give_spell(Spells::Fireball).unwrap();},
        2 =>{me.give_spell(Spells::Heal).unwrap();},
        3 =>{me.give_spell(Spells::Thunder).unwrap();},
        _ => {println!("Umm thats not an option"); panic!("")},
    };
    println!("Great now pick the second spell");
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).expect("failed to read spell");
    s2 = s2.trim_end_matches('\n').to_string();

    let s2_out:u8 = s2.parse().expect("Ummm you cant do that");
    match s2_out {
         1=>{me.give_spell(Spells::Fireball).unwrap();},
         2=>{me.give_spell(Spells::Heal).unwrap();},
         3=>{me.give_spell(Spells::Thunder).unwrap();},
        _ => {println!("Umm thats not an option"); panic!("")},
    };
    println!("\n\n\n\n\n");
    println!("1: {} ; 2: {}",me.spells_known[0],me.spells_known[1]);
    println!("Oh no a monster appeard, use your spells to take care of it");
    let mut bat = Monster::new(15,10);
    println!("Cast a spell by typing the same number as above, good luck!");
    while me.hit_points > 0 && bat.hit_points > 0  {
        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("failed to read spell");
        number = number.trim_end_matches('\n').to_string();
        let number:u8 = number.parse().expect("Ummm you cant do that");
        match me.cast(me.spells_known[number as usize -1]){
            Ok(x) => {bat.damage(x)}
            Err(_) => {println!("you did nothing")}
        }
        if bat.hit_points !=0{
        println!("The monster attacks");
        me.damage(bat.damage_per_turn);
        }
    }
    if (bat.hit_points == 0) & (me.hit_points > 0) {println!("You won") }
    if (bat.hit_points > 0) & (me.hit_points == 0) {println!("You lost")}
}

impl Monster {
    fn new(hp:u8,dpt:u8) -> Monster{
    Monster{
        hit_points : hp,
        damage_per_turn : dpt,
    }
    }
}

impl Player {
    fn new()->Player{
        Player{
            name : String::from(""),
            hit_points : 20,
            spells_known : [Spells::None,Spells::None,Spells::None],
        }
    }
    fn give_spell(&mut self, new_spell : Spells)-> Result<(),()>{
        match self.empty_spell(){
            Ok(x) => {
                self.spells_known[x] = new_spell;
                Ok(())
            },
            Err(_) => {println!("ERROR SPELL KNOWN");Err(())},
        }
    }

    fn empty_spell(&self)->Result<usize,()> {
        for i in 0..2 {
            match self.spells_known[i] {
                Spells::None => { return Ok(i as usize)},
                _ => {}
            }
        }
        return Err(())
    }

    fn is_spell_known(&self, spell :Spells) ->bool{
        match spell{
            Spells::None =>{return false}
            _ =>{
                if spell == self.spells_known[0] ||
                spell == self.spells_known[1] ||
                spell == self.spells_known[2]{
                return true
                }
                return false
            }
        };

    }


    fn set_name(&mut self,new_name :& str)-> Result<(),()>{
    let length = new_name.len();
    if length > 0 &&  20 > length{
        let x: &[_] = &['\n','\r','.',','];
        let names =  new_name.trim_end_matches(x);
        self.name = names.to_string();
        return Ok(())
    }
    Err(())
    }

    fn cast(&mut self,spell : Spells)-> Result<u8,()>{
         if self.is_spell_known(spell) {
            match spell{
            Spells::Fireball => {println!("Fireball go! you do {} damage",10); Ok(10)}
            Spells::Heal => {println!("You Heal yourself, recovering {} damage",4);self.hit_points = self.hit_points+4; Ok(0)}
            Spells::Thunder => {println!("Thunder!! you do {} damage",7); Ok(7)},
            Spells::None => {println!("You casted.... nothing? this should be imposible");Err(())}
        }
    }else{
        println!("You dont know this spell");
        Err(())
    }
}
}
impl Player {
    fn damage(&mut self,hit:u8){
        match self.hit_points{
            x if x<=0 => {
                println!("{} is already dead",self.name);
                self.hit_points = 0},

            x if x<hit => {
                println!("Fatal blow hit points below zero, {} is now dead",self.name);
                self.hit_points = 0}
            _ => {self.hit_points = self.hit_points - hit;
                println!("{} took {} damage, hp is now {}",self.name,hit,self.hit_points)}
        }
    }
}

impl Monster{
    fn damage(&mut self,hit:u8){
        match self.hit_points{
            x if x<=0 => {self.hit_points = 0},
            x if x<hit => {self.hit_points = 0}
            _ => {self.hit_points = self.hit_points-hit;}
        }
    }
}
