pub mod f
{
    use crate::*;

    pub fn get(data: &mut UsefulData, operand: &str, variable_type: usize) -> u32
    {
            let value = data.variables.get(operand);
            if value.is_none() { error(format!("Could not find variable \"{operand}\" out of {:?}", data.variables).as_str(), 1) }
            let value = match variable_type
            {
                1 => value.expect("this can't be....").1.parse::<u32>(),
                _ => value.expect("this can't be....").0.parse::<u32>()
            };
            if value.is_ok()
            {
                return value.unwrap()
            }
            0
    }

    pub fn array_extract(operand: &str) -> (String, u32)
    {
        let offset_str: Vec<&str> = operand.split("[").collect();
        let mut offset_string:Vec<String> = vec![];
        for argument in offset_str 
        {
            offset_string.push(argument.to_string());
        }
        if offset_string.len() < 2 { error(&format!("invalid variable name! arrays must be closed with ']';  {operand}").to_string(), 1) }
        let offset_string = offset_string[1].replace("]", "");
        let offset: u32;
        let offset_string = offset_string.parse::<u32>();
        if offset_string.is_ok()
        { 
            offset = offset_string.unwrap();
        }
        else
        {
            offset = 0;
        }

        let operand = operand.replace(">", (offset.to_string() + ">").as_str());
        (operand, offset)
    }

    pub fn link(data: &mut UsefulData, operand: &str) -> u32
    {
        let value: u32;
        if (operand.chars().nth(0) == Some('&')) || (data.operator == "function")
        {
            value = get(data, operand.replace("&", "").as_str(), 0);
        }
        else
        {
            value = get(data, operand, 1);
        }
        value
    }
    
    pub fn append_data_address( data: &mut UsefulData ) -> UsefulData
    {
        let data_address: u32 = ((data.statement_counter + 1) & 0xffffffff) as u32;
		let data_offset_variable: Option<&(String, String)> = data.variables.get("@data");
		let data_offset: u32;
		if data_offset_variable.is_some()
		{
			let data_offset_variable = data_offset_variable.unwrap();
			data_offset = data_offset_variable.1.parse::<u32>().expect("this never fails");
		}
		else
		{
			data_offset = 0;
		}
		data.data_offset = crate::math::f::into_hex(data_offset);
        for variable in data.variables.clone()
        {
            let original_address = variable.1.0.parse::<u32>();
            if original_address.is_ok()
            {
                let key = variable.0;
                let value = variable.1.1;
                let address = original_address.unwrap() + data_address + data_offset - 1;
                data.variables.remove_entry(&key);
                data.variables.insert(key, (address.to_string(), value));
            }
        }
        data.clone()
    }
 
	pub fn place(data: &mut UsefulData) -> UsefulData
	{
		let final_binary = data.final_binary.clone();
		let mut binary: Vec<&str> = final_binary.split(' ').collect();
		for _ in 0..data.variables.len()
		{
			binary.push("");
		}
		for variable in &data.variables
		{
			binary[variable.1.0.parse::<usize>().expect("uhhhhh that's not right") - usize::from_str_radix(&data.data_offset, 16).expect("shouldn't ever fail")] = variable.1.1.as_str();
		}
		data.final_binary = "".to_string();
		for word in 0..binary.len()
		{
			data.final_binary += &(" ".to_string() + binary[word]);
		}
		data.clone()
	}
}
