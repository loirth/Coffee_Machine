use std::io::{self, Write};
use std::{thread, time};
use std::process;


fn main() {
    call_name_animation(3);

    loop {
        choose_coffee();
    }
}


fn leave() {
	clear_entire_screen();
	move_cursor_up(1000);
    println!("Goodbye, see you later!\n");
    process::exit(0);
}


fn clear_entire_screen() {
    print!("\x1b[2J");
    io::stdout().flush().unwrap();
}


fn move_cursor_up(layers: u16) {
	print!("\x1b[{}A", layers);
	io::stdout().flush().unwrap();
}


fn choose_coffee() {
	call_test_choose_coffee_animation(1);
    ascii_coffee_machine("| Choose a coffee || Leave |");
	println!("\n");
    loop {
        let mut user_choice: String = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        match user_choice.as_str().to_lowercase().as_str().trim() {
            "leave" => leave(),
            "coffee machine" => call_name_animation(2),
            "latte" => print_coffee(user_choice.to_lowercase()),
            "americano" => print_coffee(user_choice.to_lowercase()),
            "capuchino" => print_coffee(user_choice.to_lowercase()),
            "robusta" => print_coffee(user_choice.to_lowercase()),
            "espresso" => print_coffee(user_choice.to_lowercase()),
            "tchibo" => print_coffee(user_choice.to_lowercase()),
            "3 in 1" => print_coffee(user_choice.to_lowercase()),
            "doppio" => print_coffee(user_choice.to_lowercase()),
            _ => {
                println!("We don't have this coffee. Please try again!");
                continue;
            },
        }
    }
}



// ASCII Arts

fn ascii_coffee() {
	clear_entire_screen();
	move_cursor_up(1000);
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
    println!("\nHere's your {}! Bon appetit!", coffee);
    thread::sleep(time::Duration::from_millis(5000));
    clear_entire_screen();
    call_test_choose_coffee_animation(1);
    choose_coffee();
}


fn ascii_coffee_machine(frame: &str) {
    let raw_str_1 = r"  __________________________________________";
    let raw_str_2 = r" (___________   __________________________  |";
    let raw_str_3 = format!("   [XXXXX]   | {} |", frame);
    let raw_str_4 = r"             |  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾  |";
    let raw_str_5 = r"             |    _________     ________    |";
    let raw_str_6 = r"    ( (      |   |  latte  |   |espresso|   |";
    let raw_str_7 = r"     ) )     |   |americano|   | tchibo |   |";
    let raw_str_8 = r"  .........  |   |capuchino|   | 3 in 1 |   |";
    let raw_str_9 = r"  |       |] |   | robusta |   | doppio |   |";
    let raw_str_10 = r"  \       /  |    ‾‾‾‾‾‾‾‾‾     ‾‾‾‾‾‾‾‾    |";
    let raw_str_11 = r" ‗‗`-----'‗‗‗|______________________________|";
    let raw_str_12 = r"|___________________________________________|";

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

fn coffee_machine_animation(mut repeat: i8, animation_frames: Vec<&str>) {
	let fps: u64 = 1000 / 60;

	while repeat > 0 {
		for frame in animation_frames.iter() {
			for _i in 0..4 {   // without that animation will be very fast, but if we change fps to 30 it will look bad
				ascii_coffee_machine(frame);
				thread::sleep(time::Duration::from_millis(fps));
				clear_entire_screen();
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


/*fn call_choose_coffee_animation(repeat: i8) {
    let animation_frames: Vec<&str> = vec!(
        "| Choose a coffee | leave |",
        "|Choose a coffee | leave  |",
        "|hoose a coffee | leave  C|",
        "|oose a coffee | leave  Ch|",
        "|ose a coffee | leave  Cho|",
        "|se a coffee | leave  Choo|",
        "|e a coffee | leave  Choos|",
        "| a coffee | leave  Choose|",
        "|a coffee | leave  Choose |",
        "| coffee | leave  Choose a|",
        "|coffee | leave  Choose a |",
        "|offee | leave  Choose a c|",
        "|ffee | leave  Choose a co|",
        "|fee | leave  Choose a cof|",
        "|ee | leave  Choose a coff|",
        "|e | leave  Choose a coffe|",
        "| | leave  Choose a coffee|",
        "|| leave  Choose a coffee |",
        "| leave  Choose a coffee ||",
        "|leave  Choose a coffee | |",
        "|eave  Choose a coffee | l|",
        "|ave  Choose a coffee | le|",
        "|ve  Choose a coffee | lea|",
        "|e  Choose a coffee | leav|",
        "|  Choose a coffee | leave|",
        "| Choose a coffee | leave |",
        "| Choose a coffee | leave |",
        "| Choose a coffee | leave |",
        "|                         |",
        "|                         |",
        "|                         |",
        "| Choose a coffee | leave |",
        "| Choose a coffee | leave |",
        "|                         |",
        "|                         |",
        "|                         |",
        "| Choose a coffee | leave |",
        "| Choose a coffee | leave |",
        "| Choose a coffee | leave |",
        "|                         |",
        "|                         |",
        "|                         |",
        "| Choose a coffee | leave |",
        "| Choose a coffee | leave |",
        );

    coffee_machine_animation(repeat, animation_frames)
} */

fn call_test_choose_coffee_animation(repeat: i8) {
    let animation_frames: Vec<&str> = vec!(
        "| Choose a coffee || Leave |",
        "|  Choose a coffee  Leave  |",
        "|   Choose a coffeLeavee   |",
        "|   Choose a coffLeaveee   |",
        "|   Choose a cofLeavefee   |",
        "|   Choose a coLeaveffee   |",
        "|   Choose a cLeaveoffee   |",
        "|   Choose a Leavecoffee   |",
        "|   Choose aLeave coffee   |",
        "|   Choose Leavea coffee   |",
        "|   ChooseLeave a coffee   |",
        "|   ChoosLeavee a coffee   |",
        "|   ChooLeavese a coffee   |",
        "|   ChoLeaveose a coffee   |",
        "|   ChLeaveoose a coffee   |",
        "|   CLeavehoose a coffee   |",
        "|   LeaveChoose a coffee   |",
        "|   LeaveChoose a coffee   |",
        "|  Leave  Choose a coffee  |",
        "| Leave   Choose a coffee  |",
        "| Leave || Choose a coffee |",
        "| Leave || Choose a coffee |",
        "| Leave   Choose a coffee  |",
        "|  Leave  Choose a coffee  |",
        "|   LeaveChoose a coffee   |",
        "|   LeaveChoose a coffee   |",
        "|   CLeavehoose a coffee   |",
        "|   ChLeaveoose a coffee   |",
        "|   ChoLeaveose a coffee   |",
        "|   ChooLeavese a coffee   |",
        "|   ChoosLeavee a coffee   |",
        "|   ChooseLeave a coffee   |",
        "|   Choose Leavea coffee   |",
        "|   Choose aLeave coffee   |",
        "|   Choose a Leavecoffee   |",
        "|   Choose a cLeaveoffee   |",
        "|   Choose a coLeaveffee   |",
        "|   Choose a cofLeavefee   |",
        "|   Choose a coffLeaveee   |",
        "|   Choose a coffeLeavee   |",
        "|  Choose a coffee  Leave  |",
        "| Choose a coffee || Leave |",
        );

    coffee_machine_animation(repeat, animation_frames)
}