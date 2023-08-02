#[allow(unused_imports)]
use std::io::{self, Write};

#[allow(unused_imports)]
use std::{thread, time};
use std::process;


fn main() {
	call_machine_animation(0);

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

fn move_cursor_up(layers: i16) {
	print!("\x1b[{}A", layers);
	io::stdout().flush().unwrap();
}


#[allow(dead_code)]
fn choose_coffee() {
	loop {
		let mut user_choice: String = String::new();

		call_machine_animation(1);
		
		println!("\nChoose a coffee for coffee machine above, see coffee machine or leave");

		io::stdin()
			.read_line(&mut user_choice)
			.expect("Failed to read line");

		match user_choice.as_str().to_lowercase().as_str().trim() {
			"leave" => leave(),
			"coffee machine" => coffee_machine_animation(0),
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
	coffee_machine_animation(0);
}


fn ascii_coffee_machine(frame: &str) {
	let raw_str_1 = r"  _________________________________________";
	let raw_str_2 = r" (___________   _________________________  |";
	let raw_str_3 = format!("   [XXXXX]   | {} |", frame);
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


fn coffee_machine_animation(number: i8) {

	let animation_frames: Vec<String> = create_animation_frames(call_machine_animation(number));

	let fps: u64 = 1000 / 60;
	let mut repeat = 3;

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
	}
}


fn create_animation_frames(text: Vec<char>) -> Vec<String> {
	let mut animation_frames: Vec<String> = Vec::new();
	
	for frames in 0..27 {
		let mut frame: String = String::new();
		String::push(&mut frame, '|');
		let mut i = frames;
		while i < text.len() - 1 {
			String::push(&mut frame, *text.get(i).unwrap());
			i += 1;
		}
		i = frames;
		#[allow(unused_comparisons)]
		while i >= 0 {
			String::push(&mut frame, *text.get(i).unwrap());
			i -= 1;
		}
		String::push(&mut frame, '|');
		Vec::push(&mut animation_frames, frame.clone());
		if frames == 26 {Vec::push(&mut animation_frames, frame)}
	}
	let empty_str = String::from("|                         |");
	
	let mut normal_str = String::new();

	String::push(&mut normal_str, '|');
	
	let i = 0;
	while i < text.len() - 1 {
		String::push(&mut normal_str, *text.get(i).unwrap())
	}
	for frame in 0..18 {
		if frame % 3 == 0 {
			Vec::push(&mut animation_frames, empty_str.clone());
		} else {Vec::push(&mut animation_frames, normal_str.clone());}
	}

	String::push(&mut normal_str, '|');

	return animation_frames;
}


fn call_machine_animation(number: i8) -> Vec<char> {
	match number {
		0 => {
			return vec!(
				' ',
				'C',
				'o',
				'f',
				'f',
				'e',
				'e',
				' ',
				'm',
				'a',
				'c',
				'h',
				'i',
				'n',
				'e',
				' ',
				'b',
				'y',
				' ',
				'V',
				'l',
				'a',
				'd',
				' ',
				' ',
				);
		},
		1 => {
			return vec!(
				' ',
				' ',
				' ',
				' ',
				'C',
				'h',
				'o',
				'o',
				's',
				'e',
				' ',
				'a',
				' ',
				'C',
				'o',
				'f',
				'f',
				'e',
				'e',
				' ',
				' ',
				' ',
				' ',
				' ',
				' ',
				);
		},
		_ => {
			println!("Invalid number, recalled with defauld value");
			call_machine_animation(0)
		},
	}
}

/*
	let name_animation_frames: Vec<&str> = vec!(
		"| Coffee machine by Vlad  |",
		"|Coffee machine by Vlad   |",
		"|offee machine by Vlad   C|",
		"|ffee machine by Vlad   Co|",
		"|fee machine by Vlad   Cof|",
		"|ee machine by Vlad   Coff|",
		"|e machine by Vlad   Coffe|",
		"| machine by Vlad   Coffee|",
		"|machine by Vlad   Coffee |",
		"|achine by Vlad   Coffee m|",
		"|chine by Vlad   Coffee ma|",
		"|hine by Vlad   Coffee mac|",
		"|ine by Vlad   Coffee mach|",
		"|ne by Vlad   Coffee machi|",
		"|e by Vlad   Coffee machin|",
		"| by Vlad   Coffee machine|",
		"|by Vlad   Coffee machine |",
		"|y Vlad   Coffee machine b|",
		"| Vlad   Coffee machine by|",
		"|Vlad   Coffee machine by |",
		"|lad   Coffee machine by V|",
		"|ad   Coffee machine by Vl|",
		"|d   Coffee machine by Vla|",
		"|   Coffee machine by Vlad|",
		"|  Coffee machine by Vlad |",
		"| Coffee machine by Vlad  |",
		"| Coffee machine by Vlad  |",
		"|                         |",
		"|                         |",
		"|                         |",
		"| Coffee machine by Vlad  |",
		"| Coffee machine by Vlad  |",
		"|                         |",
		"|                         |",
		"|                         |",
		"| Coffee machine by Vlad  |",
		"| Coffee machine by Vlad  |",
		"| Coffee machine by Vlad  |",
		"|                         |",
		"|                         |",
		"|                         |",
		"| Coffee machine by Vlad  |",
		"| Coffee machine by Vlad  |",
		);
*/