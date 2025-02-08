use std::env::Args;

pub struct ArgParse {
    pub args: Vec<String>,
}

impl ArgParse {
    pub fn init(input_args: Args) -> Self {
        ArgParse {
            args: input_args.collect(),
        }
    }

    /// Returns true if the given arg exists in the arg list.
    pub fn arg(&self, arg: &str, arg_full: Option<&str>) -> bool {
        self.args.iter().any(|a| a == arg)
            || arg_full.map_or(false, |full| self.args.iter().any(|a| a == full))
    }

    /// Returns the option given for an arg
    ///
    /// Returns 1 if arg is not called, or returns 2 if no value is given
    pub fn get_arg_option(&self, arg: &str, arg_full: Option<&str>) -> Result<String, i32> {
        let mut iter = self.args.iter().enumerate();

        while let Some((_, current_arg)) = iter.next() {
            if current_arg == arg || current_arg == arg_full.unwrap_or_default() {
                if let Some((_, next_arg)) = iter.next() {
                    return Ok(next_arg.clone());
                } else {
                    return Err(2); /* No value given */
                }
            }
        }

        Err(1) /* No arg was found */
    }
}
