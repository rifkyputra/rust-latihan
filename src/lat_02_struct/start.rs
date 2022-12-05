use crate::pokemon::Pokemon;

pub fn start() {
    let mut decidueye = Pokemon::new(
        String::from("decidueye"),
        vec![String::from("point"), String::from("shoot")],
    );

    {
        println!("Pokemon {} Created!", decidueye.name);

        println!("Leveling Up ...");

        decidueye.level_up();

        let impostor: &Pokemon = &decidueye;

        println!(" the impostor is now level {}", impostor.level);

        print_status(&decidueye);
    }

    decidueye.level_up();

    let impostor: &Pokemon = &decidueye;

    println!(" the impostor is now level {}", impostor.level);

    for skill in decidueye.skills {
        println!("You have Skill : {}, ", skill);
    }
}

pub fn print_status(poke: &Pokemon) {
    println!("{} is now level {}", poke.name, poke.level);
}
