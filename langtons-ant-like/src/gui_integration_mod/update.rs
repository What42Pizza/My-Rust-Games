use crate::prelude::*;
use crate::gui_mod::data::KeyboardData;



pub fn update_gui(app: &mut App, program_data: &mut ProgramData) {
	let last_screen_size = program_data.last_screen_size;
	
	
	
	let initial_area = RealArea::new(last_screen_size);
	let keyboard_data = get_keyboard_data(&app.keyboard);
	update_gui_elements(&mut program_data.gui, initial_area, &keyboard_data);
	
}





pub fn get_keyboard_data(keyboard: &Keyboard) -> KeyboardData {
	let shift = keyboard.shift();
	let ctrl = keyboard.ctrl();
	let alt = keyboard.alt();
	
	let mut typed_text = String::new();
	let mut control_keys_pressed = vec!();
	for key in &keyboard.pressed {
		match key {
			
			KeyCode::Up => control_keys_pressed.push(ControlKey::Up),
			KeyCode::Down => control_keys_pressed.push(ControlKey::Down),
			KeyCode::Left => control_keys_pressed.push(ControlKey::Left),
			KeyCode::Right => control_keys_pressed.push(ControlKey::Right),
			
			KeyCode::Return => control_keys_pressed.push(ControlKey::Return),
			KeyCode::Delete => control_keys_pressed.push(ControlKey::Delete),
			KeyCode::Back => control_keys_pressed.push(ControlKey::Backspace),
			KeyCode::End => control_keys_pressed.push(ControlKey::End),
			KeyCode::Escape => control_keys_pressed.push(ControlKey::Esc),
			
			// prevent typing while ctrl or alt are pressed
			_ if ctrl || alt => {}
			
			KeyCode::A => typed_text.push(if shift {'A'} else {'a'}),
			KeyCode::B => typed_text.push(if shift {'B'} else {'b'}),
			KeyCode::C => typed_text.push(if shift {'C'} else {'c'}),
			KeyCode::D => typed_text.push(if shift {'D'} else {'d'}),
			KeyCode::E => typed_text.push(if shift {'E'} else {'e'}),
			KeyCode::F => typed_text.push(if shift {'F'} else {'f'}),
			KeyCode::G => typed_text.push(if shift {'G'} else {'g'}),
			KeyCode::H => typed_text.push(if shift {'H'} else {'h'}),
			KeyCode::I => typed_text.push(if shift {'I'} else {'i'}),
			KeyCode::J => typed_text.push(if shift {'J'} else {'j'}),
			KeyCode::K => typed_text.push(if shift {'K'} else {'k'}),
			KeyCode::L => typed_text.push(if shift {'L'} else {'l'}),
			KeyCode::M => typed_text.push(if shift {'M'} else {'m'}),
			KeyCode::N => typed_text.push(if shift {'N'} else {'n'}),
			KeyCode::O => typed_text.push(if shift {'O'} else {'o'}),
			KeyCode::P => typed_text.push(if shift {'P'} else {'p'}),
			KeyCode::Q => typed_text.push(if shift {'Q'} else {'q'}),
			KeyCode::R => typed_text.push(if shift {'R'} else {'r'}),
			KeyCode::S => typed_text.push(if shift {'S'} else {'s'}),
			KeyCode::T => typed_text.push(if shift {'T'} else {'t'}),
			KeyCode::U => typed_text.push(if shift {'U'} else {'u'}),
			KeyCode::V => typed_text.push(if shift {'V'} else {'v'}),
			KeyCode::W => typed_text.push(if shift {'W'} else {'w'}),
			KeyCode::X => typed_text.push(if shift {'X'} else {'x'}),
			KeyCode::Y => typed_text.push(if shift {'Y'} else {'y'}),
			KeyCode::Z => typed_text.push(if shift {'Z'} else {'z'}),
			
			KeyCode::Key1 => typed_text.push(if shift {'!'} else {'1'}),
			KeyCode::Key2 => typed_text.push(if shift {'@'} else {'2'}),
			KeyCode::Key3 => typed_text.push(if shift {'#'} else {'3'}),
			KeyCode::Key4 => typed_text.push(if shift {'$'} else {'4'}),
			KeyCode::Key5 => typed_text.push(if shift {'%'} else {'5'}),
			KeyCode::Key6 => typed_text.push(if shift {'^'} else {'6'}),
			KeyCode::Key7 => typed_text.push(if shift {'&'} else {'7'}),
			KeyCode::Key8 => typed_text.push(if shift {'*'} else {'8'}),
			KeyCode::Key9 => typed_text.push(if shift {'('} else {'9'}),
			KeyCode::Key0 => typed_text.push(if shift {')'} else {'0'}),
			
			KeyCode::RBracket => typed_text.push(if shift {'{'} else {'['}),
			KeyCode::LBracket => typed_text.push(if shift {'}'} else {']'}),
			KeyCode::Backslash => typed_text.push(if shift {'|'} else {'\\'}),
			KeyCode::Semicolon => typed_text.push(if shift {';'} else {':'}),
			KeyCode::Apostrophe => typed_text.push(if shift {'\''} else {'"'}),
			KeyCode::Comma => typed_text.push(if shift {','} else {'<'}),
			KeyCode::Period => typed_text.push(if shift {'.'} else {'>'}),
			KeyCode::Slash => typed_text.push(if shift {'?'} else {'/'}),
			KeyCode::Grave => typed_text.push(if shift {'`'} else {'~'}),
			
			KeyCode::Space => typed_text.push(' '),
			KeyCode::Tab => typed_text.push('\t'),
			
			KeyCode::LShift | KeyCode::RShift
			| KeyCode::LControl | KeyCode::RControl
			| KeyCode::LAlt | KeyCode::RAlt
			| KeyCode::LWin | KeyCode::RWin => {}
			
			_ => println!("Warning: unknown key: {key:?}"),
		}
	}
	
	KeyboardData {
		shift_pressed: shift,
		control_pressed: ctrl,
		alt_pressed: alt,
		typed_text,
		control_keys_pressed,
	}
}
