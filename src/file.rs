#[allow(dead_code)]
pub mod f
{
    use crate::UsefulData;

    pub fn process( file: String, data: &mut UsefulData, _filename: String ) -> UsefulData
    {
        let mut words_string = "".to_string();
 
		data.words = crate::identification::f::words(&file);
        for word in data.words.clone()
        {
            data.word = word.to_string();
            let offset: usize;
            (*data, offset) = crate::identification::f::statement(data);
            if data.operator != "ignored" && data.operator != "comment"
            {
                *data = crate::manipulation::f::statement(data, offset, "size");
            }
        }

 		
		data.statement_counter = 0;
		data.words = crate::identification::f::words(&file);
        for word in data.words.clone()
        {
            data.word = word.to_string();
            let offset: usize;
            (*data, offset) = crate::identification::f::statement(data);
            if data.operator != "ignored" && data.operator != "comment"
            {
                *data = crate::manipulation::f::statement(data, offset, "functions");
            }
        }

		data.statement_counter = 0;
		data.words = crate::identification::f::words(&file);
        for word in data.words.clone()
        {
            data.word = word.to_string();
            let offset: usize;
            (*data, offset) = crate::identification::f::statement(data);
            if data.operator != "ignored" && data.operator != "comment"
            {
                *data = crate::manipulation::f::statement(data, offset, "variables");
            }
        }
        *data = crate::variables::f::append_data_address(data);

		data.statement_counter = 0;
		data.words = crate::identification::f::words(&file);
		for word in data.words.clone()
		{
			data.word = word.to_string();
            let offset: usize;
            (*data, offset) = crate::identification::f::statement(data);
            if data.operator != "ignored" && data.operator != "comment"
            {
                words_string = words_string + "\n\n    " + &data.word + " ;; identified as \"" + &data.operator + "\"";
                *data = crate::manipulation::f::statement(data, offset, "main");

                match data.operator.as_str()
                {
                    "noop" | "jumpu" | "jumpc" | "jumpp" | "take" | "place" | "gthan" | "lthan" | "and" | "or" | "xor" | "nor" | "add" | "sub" | "sll" | "srl"  =>
                    {
                        let binary_value = data.operation.clone().parse::<u32>();
                        if binary_value.is_ok()
                        {
                            words_string =
                                words_string +
                                "\n        " + binary_value.clone().unwrap().to_string().as_str() +
                                " | " + crate::math::f::base10tobase2(binary_value.clone().unwrap()).as_str() + "\n";
                            data.final_binary = data.final_binary.clone() + " " +binary_value.unwrap().to_string().as_str();
                        }
                        else
                        {
                            crate::error(format!("compiler failed to convert binary {:?} for log.. continuing however", data.operator).as_str(), 1);
                        }
                    }
                    _ => ()
                }
            }

		}

		*data = crate::variables::f::place(data);
        data.debug_log = format!(
"{}

statement count: {}
variable count: {}
Variables:
{:?}

Words:
[\"
{words_string}

\"]
", 
            data.statement_counter, 
            data.variable_counter, 
            data.debug_log,
            data.variables
        );
        let out_file = std::fs::File::create("0x".to_string() + &data.data_offset + ".rom");
        if out_file.is_ok()
        {
            let mut final_binary_u8: Vec<u8> = vec![];
            let numeric_words = data.final_binary.split(" ");
            for word in numeric_words
            {
                let word_parts = word.parse::<u32>();
                if word_parts.is_ok()
                {
                    let word_parts = word_parts.expect("oops...").to_be_bytes();
                    for parts in word_parts
                    {
                        final_binary_u8.push(parts);
                    }
                }
            }
            let _ = std::io::Write::write_all(&mut out_file.unwrap(), &final_binary_u8);
        }
        else
        {
            crate::error("compilation failed!", 1);
        }
        data.operator = "".to_string();
        data.operation = "".to_string();
        data.word = "".to_string();
        data.final_binary = "".to_string();
        data.words = vec![];
        data.variables = std::collections::HashMap::new();
        data.statement_counter = 0;
        data.variable_counter = 0;
        data.clone()
    }
}
