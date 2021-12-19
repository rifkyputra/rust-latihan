use std::io;


pub fn prompt() {

    let name = "World";

    let mut buffer = String::new();

    println!("Enter Your Name : ");
    print!("> ");
    io::stdin().read_line(&mut buffer).unwrap();

    
    println!("Hello, {}!", name);
    println!("Hello, {}", buffer);

    println!("-----------");


    println!("Pakai Assistant apa ? /");
    println!("1. Google");
    println!("2. Siri");
    println!("3. Alexa");
    println!("4. Bixby");

    let mut select = String::new();
    // let mut assistant : Assistant = Assistant::Google;

    io::stdin().read_line(&mut select).unwrap();

    let something :Assistant = match select.trim().parse::<u8>() {
        Ok(n) => selection(n),
        Err(e) => {
            println!("{}", e);
            selection(0)
        },
        
    };

    match something {
        Assistant::Google => {
            println!("OK, Google!");
        }
        Assistant::Siri => hi_siri(),
        Assistant::Alexa => other(),
        Assistant::Bixby => other(),

    }


}

enum Assistant {
    Siri,
    Google,
    Bixby,
    Alexa,
}

fn selection(num: u8) -> Assistant {
    println!("{}", num);
    if num == 1 {
        return Assistant::Google;
    } else if num == 2 {
        return Assistant::Siri;
    } else if num == 3 {
        return Assistant::Alexa;
    }

    return Assistant::Bixby;
}


fn hi_siri() {

    println!("Hi, I'm Siri,  What Can i help ? ");

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    println!("Sorry i can't do {} right now", buffer);


}

fn other() {
    println!("Sorry, not supported");
}