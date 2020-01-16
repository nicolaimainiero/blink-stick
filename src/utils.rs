pub fn map_hex(match_char: &str) -> u8 {
	let upper_char = match_char.to_uppercase();
	let match_number = match upper_char.as_str() {
		"A" => 10,
		"B" => 11,
		"C" => 12,
		"D" => 13,
		"E" => 14,
		"F" => 15,
		"1" => 1,
		"2" => 2,
		"3" => 3,
		"4" => 4,
		"5" => 5,
		"6" => 6,
		"7" => 7,
		"8" => 8,
		"9" => 9,
		"0" => 0,
		_ => panic!("[color-convert] map_hex not match match_char value.")
	};
	match_number
}