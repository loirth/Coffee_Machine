use std::io::{self, Write};
use std::{thread, time};
use std::process;


const ASCII_COFFEE: [&str; 22] = [
    r"                        (",
    r"                          )     (",
    r"                   ___...(-------)-....___",
    r#"               .-""       )    (          ""-."#,
    r"         .-'``'|-._             )         _.-|",
    r#"        /  .--.|   `""---...........---""`   |"#,
    r"       /  /    |                             |",
    r"       |  |    |                             |",
    r"        \  \   |                             |",
    r"         `\ `\ |                             |",
    r"           `\ `|                             |",
    r"           _/ /\                             /",
    r"          (__/  \                           /",
    r#"       _..---""` \                         /`""---.._"#,
    r"    .-'           \                       /          \'-.",
    r"   :               `-.__             __.-'              :",
    r#"   :                  ) ""---...---"" (                 :"#,
    r#"    '._               `"--...___...--"`              _.'"#,
    r#"      \""--..__                              __..--""/"#,
    r#"       '._     """----.....______.....----"""     _.'"#,
    r#"          `""--..,,_____            _____,,..--""`"#,
    r#"                        `"""----"""`"#,
];

const ASCII_COFFEE_MACHINE: [&str; 12] = [
    r"  __________________________________________",
    r" (___________   __________________________  |",
    r"   [XXXXX]   | {frame} |",
    r"             |  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾  |",
    r"             |    _________     ________    |",
    r"    ( (      |   |  latte  |   |espresso|   |",
    r"     ) )     |   |americano|   | tchibo |   |",
    r"  .........  |   |capuchino|   | 3 in 1 |   |",
    r"  |       |] |   | robusta |   | doppio |   |",
    r"  \       /  |    ‾‾‾‾‾‾‾‾‾     ‾‾‾‾‾‾‾‾    |",
    r" ‗‗`-----'‗‗‗|______________________________|",
    r"|___________________________________________|",
];


fn main() {
    call_name_animation(2);

    loop {
        choose_coffee();
    }
}

fn leave() {
	clear_screen();
	move_cursor_up(1000);
    println!("Goodbye, see you later!\n");
    process::exit(0);
}

fn clear_screen() {
    print!("\x1b[2J");
    io::stdout().flush().unwrap();
}

fn move_cursor_up(layers: u16) {
	print!("\x1b[{}A", layers);
	io::stdout().flush().unwrap();
}

fn choose_coffee() {
	call_choose_coffee_animation(1);
    ascii_coffee_machine("| Choose a coffee || Leave |");
	println!("\n");
    loop {
        let mut user_choice: String = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        match user_choice.to_lowercase().trim() {
            "leave" => leave(),
            "coffee machine" => {
                call_name_animation(2);
                choose_coffee();
            },
            "latte" | "americano" | "capuchino" | "robusta" | "espresso" | "tchibo" | "3 in 1" | "doppio" => {
                print_coffee(user_choice.to_lowercase());
            },
            _ => {
                println!("We don't have this coffee. Please try again!");
                continue;
            },
        }
    }
}

fn print_ascii_coffee(art: &[&str]) {
    clear_screen();
	move_cursor_up(1000);

    for line in art {
        println!("{}", line);
    }
}

fn print_coffee(coffee: String) {
    print_ascii_coffee(&ASCII_COFFEE);
    println!("\nHere's your {}! Bon appetit!", coffee);
    thread::sleep(time::Duration::from_millis(4000));
    clear_screen();
    call_choose_coffee_animation(1);
    choose_coffee();
}

fn ascii_coffee_machine(frame: &str) {
    for line in ASCII_COFFEE_MACHINE.iter() {
        println!("{}", line.replace("{frame}", frame));
    }
}

fn coffee_machine_animation(mut repeat: i8, animation_frames: Vec<&str>) {
	let fps: u64 = 1000 / 60;

	while repeat > 0 {
		for frame in animation_frames.iter() {
			for _i in 0..4 {   // without that animation will be very fast, but if we change fps to 30 it will look bad
				ascii_coffee_machine(frame);
				thread::sleep(time::Duration::from_millis(fps));
				clear_screen();
				move_cursor_up(1000);
			};
		};

		repeat -= 1;
	};
}

fn call_name_animation(repeat: i8) {
    let animation_frames: Vec<&str> = vec!(
        "| Coffee machine by loirth |",
        "|Coffee machine by loirth  |",
        "|offee machine by loirth  C|",
        "|ffee machine by loirth  Co|",
        "|fee machine by loirth  Cof|",
        "|ee machine by loirth  Coff|",
        "|e machine by loirth  Coffe|",
        "| machine by loirth  Coffee|",
        "|machine by loirth  Coffee |",
        "|achine by loirth  Coffee m|",
        "|chine by loirth  Coffee ma|",
        "|hine by loirth  Coffee mac|",
        "|ine by loirth  Coffee mach|",
        "|ne by loirth  Coffee machi|",
        "|e by loirth  Coffee machin|",
        "| by loirth  Coffee machine|",
        "|by loirth  Coffee machine |",
        "|y loirth  Coffee machine b|",
        "| loirth  Coffee machine by|",
        "|loirth  Coffee machine by |",
        "|oirth  Coffee machine by l|",
        "|irth  Coffee machine by lo|",
        "|rth  Coffee machine by loi|",
        "|th  Coffee machine by loir|",
        "|h  Coffee machine by loirt|",
        "|  Coffee machine by loirth|",
        "| Coffee machine by loirth |",
        "| Coffee machine by loirth |",
        "|                          |",
        "|                          |",
        "|                          |",
        "| Coffee machine by loirth |",
        "| Coffee machine by loirth |",
        "|                          |",
        "|                          |",
        "|                          |",
        "| Coffee machine by loirth |",
        "| Coffee machine by loirth |",
        "| Coffee machine by loirth |",
        "|                          |",
        "|                          |",
        "|                          |",
        "| Coffee machine by loirth |",
        "| Coffee machine by loirth |",
        );

    coffee_machine_animation(repeat, animation_frames)
}


fn call_choose_coffee_animation(repeat: i8) {
    let animation_frames: Vec<&str> = vec!(
        "| Choose a coffee || leave |",
        "|Choose a coffee || leave  |",
        "|hoose a coffee || leave  C|",
        "|oose a coffee || leave  Ch|",
        "|ose a coffee || leave  Cho|",
        "|se a coffee || leave  Choo|",
        "|e a coffee || leave  Choos|",
        "| a coffee || leave  Choose|",
        "|a coffee || leave  Choose |",
        "| coffee || leave  Choose a|",
        "|coffee || leave  Choose a |",
        "|offee || leave  Choose a c|",
        "|ffee || leave  Choose a co|",
        "|fee || leave  Choose a cof|",
        "|ee || leave  Choose a coff|",
        "|e || leave  Choose a coffe|",
        "| || leave  Choose a coffee|",
        "||| leave  Choose a coffee |",
        "|| leave  Choose a coffee ||",
        "|leave  Choose a coffee || |",
        "|eave  Choose a coffee || l|",
        "|ave  Choose a coffee || le|",
        "|ve  Choose a coffee || lea|",
        "|e  Choose a coffee || leav|",
        "|  Choose a coffee || leave|",
        "| Choose a coffee || leave |",
        "| Choose a coffee || leave |",
        "| Choose a coffee || leave |",
        "|                          |",
        "|                          |",
        "|                          |",
        "| Choose a coffee || leave |",
        "| Choose a coffee || leave |",
        "|                          |",
        "|                          |",
        "|                          |",
        "| Choose a coffee || leave |",
        "| Choose a coffee || leave |",
        "| Choose a coffee || leave |",
        "|                          |",
        "|                          |",
        "|                          |",
        "| Choose a coffee || leave |",
        "| Choose a coffee || leave |",
        );

    coffee_machine_animation(repeat, animation_frames)
}