#[allow(dead_code)]
pub mod f
{
    use crate::UsefulData;

    pub fn process( file: String, data: &mut UsefulData ) -> UsefulData
    {
        let mut statement_counter = 0;
        let mut variable_counter = 0;

        let mut words_string = "".to_string();
        data.words = crate::identification::f::words(&file);
        for word in data.words.clone()
        {
            data.word = word.to_string();
            let mut offset: usize;
            (*data, offset) = crate::identification::f::statement(data);
            if data.operator != "ignored" && data.operator != "comment"
            {
                words_string = words_string + "\n\n    " + &data.word + " ;; identified as \"" + &data.operator + "\"";
                // vars arch 012
                let processed_statement = crate::manipulation::f::statement(data, offset);

                statement_counter += processed_statement.0;
                if processed_statement.1 != "none"
                {
                    data.variables.push(processed_statement.1);
                    variable_counter += 1;
                }
                if processed_statement.2.len() > 0
                {
                    let mut binary_value = processed_statement.2.clone().parse::<u32>();
                    if binary_value.is_ok()
                    {
                        words_string =
                            words_string +
                            "\n        " + binary_value.clone().unwrap().to_string().as_str() +
                            " | " + crate::math::f::base10tobase2(binary_value.unwrap()).as_str() + "\n";
                    }
                    else
                    {
                        println!("compiler failed to convert binary {:?} for log.. continuing however", processed_statement.2);
                    }
                }
            }
        }
        data.debug_log = format!(
"{}

statement count: {statement_counter}
variable count: {variable_counter}
Variables:
{:?}

Words:
[\"
{words_string}

\"]
", data.debug_log ,data.variables);
        data.clone()
    }
}
