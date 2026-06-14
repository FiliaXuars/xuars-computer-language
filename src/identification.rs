#[allow(dead_code)]
pub mod f
{
    use crate::UsefulData;
    pub fn words(program: &String) -> Vec<String>
    {
        let words: Vec<&str> = program.split(['\n', ';']).collect();
        let mut new_words: Vec<String> = vec![];
        for word in words
        {
            new_words.push(word.to_string());
        }
        new_words
    }

    pub fn statement(data: &mut UsefulData) -> (UsefulData, usize)
    {
		let ignored_keys = 
		[
			"is",
			"then",
			"do",
			"if",
			"set",
			"to",
			"do",
			"store",
			"at",
			"	",
			" ",
			"",
		];
		let filtered_statement_parts: Vec<&str> = data.word.split(' ').collect();
		let mut filtered_statement: String = "".to_string();
		for part in 0..filtered_statement_parts.len()
		{
			filtered_statement = filtered_statement + "\t" + filtered_statement_parts[part];
		}
		let mut filtered_statement_parts: Vec<&str> = filtered_statement.split('\t').collect();
		for key in 0..ignored_keys.len()
		{
			let _: Vec<&str> = filtered_statement_parts.extract_if(.., |word| (word == &ignored_keys[key]) == true).collect();
		}
        let mut word_components: Vec<String> = vec![];
        for word in filtered_statement_parts
        {
            word_components.push(word.to_string());
        }

        let mut done_searching = false;
        let mut iteration = 0;
        let mut word_type;
        while !done_searching
        {
            if iteration >= word_components.len()
            {
                data.operator = "ignored".to_string();
                return (data.clone(), 0)
            }
            if word_components[iteration] != ""
            {
                done_searching = true;
            }
            else
            {
                iteration += 1;
            }
        }

        word_type = match word_components[iteration].as_str()
        {
            "#"     => "comment",
			"noop"	=> "noop",
            "jumpu" => "jumpu",
            "jumpc" => "jumpc",
            "jumpp" => "jumpp",
            "take"  => "take",
            "place" => "place",
            _       => "variable"
        };
        if word_type == "variable"
        {
            let first_char = word_components.clone()[iteration].chars().nth(0);
            if first_char.is_some()
            {
                let first_char = first_char.unwrap();
                let mut numbers: [char; 10] = ['0'; 10];
                for iteration in 1..=9
                {
                    numbers[iteration] = format!("{}", iteration).chars().nth(0).expect("Huh thats weird...");
                }
                for character in numbers
                {
                    if character == first_char
                    {
                        word_type = "arithmatic";
                    }
                }
            }

            if word_type == "variable"
            {
                for character in word_components.clone()[iteration].chars()
                {
                    if character == '('
                    {
                        word_type = "function";
                    }
                    if character == '['
                    {
                        word_type = "array";
                    }
                }
            }
            if word_type == "array"
            {
                let name = crate::variables::f::array_extract(&word_components[iteration]);
                for variable in 0..name.1
                {
                    let variable_name = crate::variables::f::array_extract(&word_components[iteration].replace(name.1.to_string().as_str(), variable.to_string().as_str()));
                    data.variables.insert(variable_name.0, (data.variable_counter.to_string(), "0".to_string()));
                    data.variable_counter = data.variable_counter + 1;
                }
            }
        }
        if word_type == "arithmatic"
        {
            if word_components.len() > iteration + 1
            {
                word_type = match word_components[iteration + 1].as_str()
                {
                    "gthan" => "gthan",
                    "lthan" => "lthan",
                    "and"   => "and",
                    "or"    => "or",
                    "xor"   => "xor",
                    "nor"   => "nor",
                    "add"   => "add",
                    "sub"   => "sub",
                    "sll"   => "sll",
					"shiftl" => "sll",
					"shiftr" => "srl",
                    "srl"   => "srl",
                    _       => "ignored"
                };
            }
            else
            {
                word_type = "ignored";
            }
        }
        data.operator = word_type.to_string();
        data.words = word_components;
        (data.clone(), iteration)
    }
}
