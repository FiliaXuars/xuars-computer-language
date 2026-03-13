pub mod f
{
    use crate::*;

    pub fn link(data: &UsefulData, operand: &str) -> u32
    {
        if data.operator == "array"
        {
            return 0
        }
        if (operand.chars().nth(0) == Some('&')) | (data.operator == "function")
        {
            let value = data.variables.get(operand).expect("could not find a variable!");
            let value = value.0.parse::<u32>();
            if value.is_ok()
            {
                return value.unwrap()
            }
        }
        else
        {
            let value = data.variables.get(operand).expect("could not find a variable!");
            let value = value.1.parse::<u32>();
            if value.is_ok()
            {
                return value.unwrap()
            }
        }
        0
    } 
    
}
