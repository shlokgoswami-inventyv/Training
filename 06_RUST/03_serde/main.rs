use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Position{
	x:u32,
	y:u32,
	z:u32
}
#[derive(Serialize, Deserialize, Debug)]
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

fn main() {
    let p = Player {
        id: 1,
        name: "Hero".to_string(),
		health:100,
		position:Position { x:10, y: 5, z:4 }
    };

    let json = serde_json::to_string(&p).unwrap();
    println!("JSON: {}", json);

    let back: Player = serde_json::from_str(&json).unwrap();
    println!("Back: {:?}", back);

	let raw_string=r#"{"id": 1,"name": "Hero","health": 100,"position": {"x": 10,"y": 20,"z": 30}}"#;

	let p: Player = serde_json::from_str(raw_string).unwrap();
    println!("From raw string: {:?}", p);
	
}
