// this program interprets a custom language of code and returns data qualified for a few platforms
// namely:
//  - x4c
//  - x4cb
//
// a program offset is required
// # global variable scope
// structs(for writer clarity) `struct {}`
// comments `# comment`
// defining and calling variables, simply type its name `variable` use `&variables` to reference
//  its address. assiging can also be done using `variable is x`
// defining functions `function() {}` use `function()` to call and `&function()` to reference
// # modifying data in a register is done by:
//  - `a operator b is c`
//  - `operation a <address>` if <address> is not provided it will be assumed as zero
//
// # modifying data at an address is done by:
//  - `x is <value>`
//
// ARF!!! - Filia <3
// I LOVE YOU - Owner <3
//

#[allow(dead_code)]
#[ignore = "unused"]

mod math;
mod file;
mod process;
mod identification;
mod manipulation;
mod useful_data;

use math::f::*;
use process::f::*;
use useful_data::f::UsefulData;

use file::f::*;

fn main() {
    let mut data = UsefulData::new();
    data.debug_log = "#! /usr/bin/env more\nxuars-computer-language\n\n".to_string();
    data.architecture = "x4c".to_string();
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
                        data = process(file_string, &mut data);
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

    data.debug_log = data.debug_log + "\nArchitecture: " + &data.architecture + "\n";
    let debug_file = std::fs::File::create("out.log");
    if debug_file.is_ok()
    {
        std::io::Write::write(&mut debug_file.unwrap(), &data.debug_log.as_bytes());
    }
    else
    {
        error("compilation successful! however creating a log \"out.log\" was not...", 0);
    }
}
