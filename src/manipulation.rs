#[allow(dead_code)]
pub mod f
{
    use crate::UsefulData;
    use crate::*;

    pub fn argument(data: &mut UsefulData) -> UsefulData
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2
        {
            error("usage", 1);
        }
        let mut program_name_ignored = false;
        let mut type_flag = "argument";
        for arg in &args
        {
            data.debug_log = "".to_string() + &data.debug_log + "\n" + arg;
            match type_flag
            {
                "argument" =>
                {
                    if program_name_ignored == true
                    {

                        if arg.len() > 2 && arg.clone().chars().nth(1).unwrap() == '-'
                        {
                            match arg.clone().as_str()
                            {
                                "--arch"    => type_flag = "architecture",
                                _                   => error("usage", 1)
                            }
                        }
                        else
                        {
                            let file_string = std::fs::read_to_string(arg.as_str());
                            if file_string.is_err()
                            {
                                error("invalid file", 1);
                            }
                            let file_string = file_string.unwrap();
                            *data = crate::file::f::process(file_string, data);
                        }
                    }
                },
                "architecture" =>
                {
                    data.architecture = arg.clone();
                    type_flag = "argument";
                },
                _ => error("oopsie", 1)

            }
            program_name_ignored = true;
        }
        data.clone()
    }

    pub fn get_platform_code(data: &mut UsefulData) -> UsefulData
    {
        let mut operations: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let opcode_keys: [&str; 16] = ["noop", "jumpu", "jumpc", "jumpp", "take", "place", "gthan", "lthan", "and", "or", "xor", "nor", "add", "sub", "sll", "srl"];
        let opcode_value: [usize; 16] = match data.architecture.as_str()
        {
            "x4c" | "x4cb" =>
            {
                let mut new_array: [usize; 16] = [0; 16];
                for iteration in 0..16
                {
                    new_array[iteration] = iteration << 28 & 0xf0000000;
                }
                new_array
            },
            _ =>
            {
                    error(format!("unsupported architecture {}", data.architecture).as_str(), 1);
                    [0; 16]
            }
        };
        for key in 0..16
        {
            operations.insert(opcode_keys[key], opcode_value[key]);
        }
        data.operation = operations.get(data.operator.as_str()).expect("huh... ummmmmmm... that not suppored to happen...").to_string();
        *data = word_arguments(data);
        data.clone()
    }

    pub fn statement(data: &mut UsefulData, start_offset: usize) -> (usize, String, String)
    {
        // should we add to <>?
        let mut pc = 0;
        let mut vars: String = "none".to_string();
        for _ in 0..3
        {
            data.words.push("0".to_string());
        }
        let code: String = match data.operator.as_str()
        {
            "jumpu" | "jumpc" | "jumpp" | "take" | "place" | "gthan" | "lthan" | "and" | "or" | "xor" | "nor" | "add" | "sub" | "sll" | "srl"  =>
                {
                    pc = 1;
                    data.word = 
                        "".to_string() + 
                        &data.words[start_offset] + " " + 
                        &data.words[start_offset+1] + " " + 
                        &data.words[start_offset+2] + " " + 
                        &data.words[start_offset+3];
                    *data = get_platform_code(data);
                    data.operation.clone()

                },
            "variable" => { vars = data.words[start_offset].to_string(); "".to_string() },
            "array" => { vars = data.words[start_offset].to_string(); "".to_string() },
            "function" => { vars = data.words[start_offset].to_string(); "".to_string() },
            _ => { error(format!("invalid statement passed {}", data.operator).as_str(), 1); "".to_string() }
        };
        (pc, vars, code)
    }

    pub fn word_arguments(data: &mut UsefulData) -> UsefulData
    {
        let mut operation = data.operation.parse::<u32>().expect("function get_platorm_code() passed an invalid value");
        match data.architecture.as_str()
        {
            "x4c" | "x4cb" =>
            {
                let mut instruction_format = "";
                let word_components: Vec<&str> = data.word.split(' ').collect();
                let mut word_arguments = [""; 3];
                match data.operator.as_str()
                {
                    "noop" | "jumpu" | "jumpc" | "jumpp" | "take" | "place" =>
                    {
                        for iteration in 0..3
                        {
                            word_arguments[iteration] = word_components[iteration+1];
                            instruction_format = "icmmmmmm";
                        }
                    },
                    "gthan" | "lthan" | "and" | "or" | "xor" | "nor" | "add" | "sub" | "sll" | "srl" =>
                    {
                        word_arguments[0] = word_components[0];
                        word_arguments[1] = word_components[2];
                        word_arguments[2] = word_components[3];
                        instruction_format = "iccc"
                    }
                    _ => ()
                }
                for iteration in 0..3
                {
                    let mut format_spec: [(bool, u32, u32); 3] = [(false, 0, 0); 3];
                    let value = word_arguments[iteration].parse::<u32>();
                    match instruction_format
                    {
                        "icmmmmmm" =>
                        {
                            format_spec[0] = (true, 26, (1<<2)-1);
                            format_spec[1] = (true, 0, (1<<26)-1);
                        },
                        "iccc" =>
                        {
                            format_spec[0] = (true, 26, (1<<2)-1);
                            format_spec[1] = (true, 24, (1<<2)-1);
                            format_spec[2] = (true, 22, (1<<2)-1);
                        }
                        _ => error("invalid instruction format", 1)
                    }
                    if value.is_ok()
                    {
                        if format_spec[iteration].0
                        {
                            operation = operation + ((value.unwrap() & format_spec[iteration].2) << format_spec[iteration].1);
                        }
                    }
                    else
                    {
                        link_variables();
                    }
                }
            }
            _ => ()
        }
        data.operation = operation.to_string();
        data.clone()
    }

    pub fn link_variables() -> u32
    {
        0
    }
}
