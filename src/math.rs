#[allow(dead_code)]
pub mod f
{
	pub fn into_hex(number: u32) -> String
	{
		let mut hex_value: String = "".to_string();
		for character in 0..32/4
		{
			match (number.wrapping_shl(character*4) & 0xf0000000).wrapping_shr(28)
			{
				0 => hex_value += "0",
				1 => hex_value += "1",
				2 => hex_value += "2",
				3 => hex_value += "3",
				4 => hex_value += "4",
				5 => hex_value += "5",
				6 => hex_value += "6",
				7 => hex_value += "7",
				8 => hex_value += "8",
				9 => hex_value += "9",
				10 => hex_value += "a",
				11 => hex_value += "b",
				12 => hex_value += "c",
				13 => hex_value += "d",
				14 => hex_value += "e",
				15 => hex_value += "f",
				_ => ()
			}
		}
		hex_value
	}

    pub fn base10tobase2(mut number: u32) -> String
    {
        let mut value = "".to_string();
        while number >= 1
        {
            value = "".to_string() + (number % 2).to_string().as_str() + &value;
            number = number / 2;
        }
        while value.len() < 32
        {
            value = "".to_string() + "0" + &value;
        }
        value
    }
}
