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

mod process;
mod math;
mod useful_data;

mod file;
mod identification;
mod manipulation;
mod variables;

use process::f::*;
use useful_data::f::UsefulData;

fn main() {
    let mut data = UsefulData::new();
    data.debug_log = "#! /usr/bin/env more\nxuars-computer-language\n\n".to_string();
    data.architecture = "x4c".to_string();

    data = manipulation::f::argument(&mut data);

    data.debug_log = data.debug_log + "\nArchitecture: " + &data.architecture + "\n";
    let debug_file = std::fs::File::create("out.log");
    if debug_file.is_ok()
    {
        let _ = std::io::Write::write(&mut debug_file.unwrap(), &data.debug_log.as_bytes());
    }
    else
    {
        error("compilation successful! however creating a log \"out.log\" was not...", 0);
    }
}
