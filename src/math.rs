#[allow(dead_code)]
pub mod f
{
    use crate::UsefulData;
    pub fn base10tobase2(mut number: u32) -> String
    {
        let mut iteration = 0;
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
