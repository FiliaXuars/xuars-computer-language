pub mod f
{   
    #[derive(Debug, Clone)]
    pub struct UsefulData
    {
        pub operator:           String,
        pub operation:          String,
        pub word:               String,
        pub architecture:       String,
        pub debug_log:          String,
        pub final_binary:       String,
        pub words:              Vec<String>,
        pub variables:          std::collections::HashMap<String, (String, String)>,
		pub data_offset:		String,
        pub statement_counter:  usize,
        pub variable_counter:   usize
    }

    impl UsefulData
    {
        pub fn new() -> UsefulData
        {
            UsefulData
            {
                operator:       	"".to_string(),
                operation:      	"".to_string(),
                word:           	"".to_string(),
                architecture:       "".to_string(),
                debug_log:          "".to_string(),
                final_binary:       "".to_string(),
                words:              vec![],
                variables:          std::collections::HashMap::new(),
				data_offset:		"".to_string(),
                statement_counter:  0,
                variable_counter:   0
            }
        }
    }
}

