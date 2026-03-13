#[allow(dead_code)]
pub mod f
{
    use crate::UsefulData;
    pub fn error(reason: &str, code: i32)
    {
        match reason
        {
            "usage" => usage(),
            _       => eprintln!("{} [ERROR] {}", std::env::current_exe().expect("progam has no name?....").display(), reason)
        }
        std::process::exit(code);
    }

    pub fn usage()
    {
        print!(
    "XCLc USAGE:
        {} <flags> <files>

        available flags:
            --arch <architecture>
                x4c, x86_64
    ", std::env::current_exe().expect("progam has no name?....").display());
    }
}
