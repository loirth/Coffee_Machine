#[allow(unused_imports)]
use std::io::{self, Write};

#[allow(unused_imports)]
use std::{thread, time};
use std::process;


fn main() {
    ascii_coffee_machine(); // Introduction

    loop {
        choose_coffee();
    }
}

#[allow(dead_code)]
fn leave() {
    println!("Goodbye, see you later!\n");
    process::exit(0);
}


fn clear_entire_screen() {
    print!("\x1b[2J");
    io::stdout().flush().unwrap();
}


#[allow(dead_code)]
fn choose_coffee() {
    loop {
        let mut user_choice: String = String::new();
        
        println!("\nChoose a coffee for coffee machine above, see coffee machine or leave");

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        match user_choice.as_str().to_lowercase().as_str().trim() {
            "leave" => leave(),
            "coffee machine" => ascii_coffee_machine(),
            "latte" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "americano" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "capuchino" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "robusta" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "espresso" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "tchibo" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "3 in 1" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "3in1" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "3x1" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            "doppio" => {
                print_coffee(user_choice.to_lowercase());
                break;
            },
            _ => {
                println!("Invalid function. Please try again!");
                continue;
            },
        }
    }
}



// ASCII Arts

fn ascii_coffee() {
    let raw_str_1 = r"                        (";
    let raw_str_2 = r"                          )     (";
    let raw_str_3 = r"                   ___...(-------)-....___";
    let raw_str_4 = r#"               .-""       )    (          ""-."#;
    let raw_str_5 = r"         .-'``'|-._             )         _.-|";
    let raw_str_6 = r#"        /  .--.|   `""---...........---""`   |"#;
    let raw_str_7 = r"       /  /    |                             |";
    let raw_str_8 = r"       |  |    |                             |";
    let raw_str_9 = r"        \  \   |                             |";
    let raw_str_10 = r"         `\ `\ |                             |";
    let raw_str_11 = r"           `\ `|                             |";
    let raw_str_12 = r"           _/ /\                             /";
    let raw_str_13 = r"          (__/  \                           /";
    let raw_str_14 = r#"       _..---""` \                         /`""---.._"#;
    let raw_str_15 = r"    .-'           \                       /          \'-.";
    let raw_str_16 = r"   :               `-.__             __.-'              :";
    let raw_str_17 = r#"   :                  ) ""---...---"" (                 :"#;
    let raw_str_18 = r#"    '._               `"--...___...--"`              _.'"#;
    let raw_str_19 = r#"      \""--..__                              __..--""/"#;
    let raw_str_20 = r#"       '._     """----.....______.....----"""     _.'"#;
    let raw_str_21 = r#"          `""--..,,_____            _____,,..--""`"#;
    let raw_str_22 = r#"                        `"""----"""`"#;

    println!("{}", raw_str_1);
    println!("{}", raw_str_2);
    println!("{}", raw_str_3);
    println!("{}", raw_str_4);
    println!("{}", raw_str_5);
    println!("{}", raw_str_6);
    println!("{}", raw_str_7);
    println!("{}", raw_str_8);
    println!("{}", raw_str_9);
    println!("{}", raw_str_10);
    println!("{}", raw_str_11);
    println!("{}", raw_str_12);
    println!("{}", raw_str_13);
    println!("{}", raw_str_14);
    println!("{}", raw_str_15);
    println!("{}", raw_str_16);
    println!("{}", raw_str_17);
    println!("{}", raw_str_18);
    println!("{}", raw_str_19);
    println!("{}", raw_str_20);
    println!("{}", raw_str_21);
    println!("{}", raw_str_22);
}

#[allow(dead_code)]
fn print_coffee(coffee: String) {
    ascii_coffee();
    println!("\nHere's your {}. Bon appetit!", coffee);
    thread::sleep(time::Duration::from_millis(5000));
    clear_entire_screen();
    ascii_coffee_machine();
}


fn ascii_coffee_machine() { // I make it in different variables to simplify modifications in future and better readability
    let raw_str_1 = r"  _________________________________________";
    let raw_str_2 = r" (___________   _________________________  |";
    let raw_str_3 = r"   [XXXXX]   | | Coffee machine by Vlad  | |";
    let raw_str_4 = r"             |  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾  |";
    let raw_str_5 = r"             |    _________    ________    |";
    let raw_str_6 = r"    ( (      |   |  latte  |  |espresso|   |";
    let raw_str_7 = r"     ) )     |   |americano|  | tchibo |   |";
    let raw_str_8 = r"  .........  |   |capuchino|  | 3 in 1 |   |";
    let raw_str_9 = r"  |       |] |   | robusta |  | doppio |   |";
    let raw_str_10 = r"  \       /  |    ‾‾‾‾‾‾‾‾‾    ‾‾‾‾‾‾‾‾    |";
    let raw_str_11 = r" ‗‗`-----'‗‗‗|_____________________________|";
    let raw_str_12 = r"|__________________________________________|";

    println!("{}", raw_str_1);
    println!("{}", raw_str_2);
    println!("{}", raw_str_3);
    println!("{}", raw_str_4);
    println!("{}", raw_str_5);
    println!("{}", raw_str_6);
    println!("{}", raw_str_7);
    println!("{}", raw_str_8);
    println!("{}", raw_str_9);
    println!("{}", raw_str_10);
    println!("{}", raw_str_11);
    println!("{}", raw_str_12);
}



/*
fn ascii_name() {
    let raw_str_1 = r" _______  _______  _______  _______  _______  _______    _______  _______  _______          _________ _        _______ ";
    let raw_str_2 = r"(  ____ \(  ___  )(  ____ \(  ____ \(  ____ \(  ____ \  (       )(  ___  )(  ____ \|\     /|\__   __/( (    /|(  ____ \";
    let raw_str_3 = r"| (    \/| (   ) || (    \/| (    \/| (    \/| (    \/  | () () || (   ) || (    \/| )   ( |   ) (   |  \  ( || (    \/";
    let raw_str_4 = r"| |      | |   | || (__    | (__    | (__    | (__      | || || || (___) || |      | (___) |   | |   |   \ | || (__    ";
    let raw_str_5 = r"| |      | |   | ||  __)   |  __)   |  __)   |  __)     | |(_)| ||  ___  || |      |  ___  |   | |   | (\ \) ||  __)   ";
    let raw_str_6 = r"| |      | |   | || (      | (      | (      | (        | |   | || (   ) || |      | (   ) |   | |   | | \   || (      ";
    let raw_str_7 = r"| (____/\| (___) || )      | )      | (____/\| (____/\  | )   ( || )   ( || (____/\| )   ( |___) (___| )  \  || (____/\";
    let raw_str_8 = r"(_______/(_______)|/       |/       (_______/(_______/  |/     \||/     \|(_______/|/     \|\_______/|/    )_)(_______/";
    println!("{}", raw_str_1);
    println!("{}", raw_str_2);
    println!("{}", raw_str_3);
    println!("{}", raw_str_4);
    println!("{}", raw_str_5);
    println!("{}", raw_str_6);
    println!("{}", raw_str_7);
    println!("{}", raw_str_8);
}


fn loading_animation() {
    println!("Loading...");
    for i in 0..101 {
        thread::sleep(time::Duration::from_millis(100));
        print!("\x1b[1000D {}%", i.to_string());
        io::stdout().flush().unwrap();
    }
    print!("\x1b[1A \x1b[1000D Loading completed!");
}
*/