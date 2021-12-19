use crate::pokemon::Pokemon;

pub fn start() {
    let mut decidueye = Pokemon{
        name: String::from("decidueye"),
        level: 0,
        buff : String::new(),
        skills: vec![String::from("point"), String::from("shoot")],

    };

    println!("Pokemon {} Created!", decidueye.name);

    println!("Leveling Up ...");

    decidueye.level_up();

    println!("{} is now level {}", decidueye.name, decidueye.level);
    
    for skill in decidueye.skills {
        println!("You have Skill : {}, ", skill);
    }

}