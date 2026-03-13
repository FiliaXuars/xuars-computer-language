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
        let word_components_as_str: Vec<&str> = data.word.split(' ').collect();
        let mut word_components: Vec<String> = vec![];
        for word in word_components_as_str
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
