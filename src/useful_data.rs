pub mod f
{   
    #[derive(Debug, Clone)]
    pub struct UsefulData
    {
        pub operator:       String,
        pub operation:      String,
        pub word:           String,
        pub architecture:   String,
        pub debug_log:      String,
        pub words:          Vec<String>,
        pub variables:      std::collections::HashMap<String, (String, String)>
    }

    impl UsefulData
    {
        pub fn new() -> UsefulData
        {
            UsefulData
            {
                operator:       "".to_string(),
                operation:      "".to_string(),
                word:           "".to_string(),
                architecture:   "".to_string(),
                debug_log:      "".to_string(),
                words:          vec![],
                variables:      std::collections::HashMap::new()
            }
        }
    }
}

