#[derive(Debug)]
struct Position{
	x:u32,
	y:u32,
	z:u32
}
#[derive(Debug)]
struct Player{
    id:u32,
	name:String,
    health:u8,
	position:Position
}

impl Player {

	fn set_id(&mut self,id:u32){
		self.id=id;
	}

	fn set_name(&mut self,name:String){
		self.name=name;
	}

	fn set_health(&mut self,health:u8){
		self.health=health;
	}
	fn set_position(&mut self,a:Position){
		self.position=a;
	}

	fn get_name(&self) -> String{
		self.name.clone()
	}
	
	fn get_id(&self) -> u32 {
		self.id
	}
	
	fn get_health(&self) -> u8{
		self.health
	}
    fn get_player_wo_args(&self) -> String{
        format!("id is: {}\nName is: {}\nHealth is: {}",self.id,self.name,self.health)
    }
	
}

fn main(){
	
    let mut user1:Player = Player{
        id:1234,
        name:String::from("Shlok"),
        health:100,
		position:Position{
			x:0,
			y:0,
			z:0
		}
    };
    user1.set_id(32);
    user1.set_name(String::from("Shlok1"));
    user1.set_health(32);
	user1.set_position(Position { x: 1, y: 1, z: 1 });
    print!("using gettters :\n{}\n{}\n{}\n",user1.get_id(),user1.get_name(),user1.get_health());
	print!("using function without aurguments {}",user1.get_player_wo_args());
	println!("\nWithout arguments :\n{:#?}",user1);
    

}