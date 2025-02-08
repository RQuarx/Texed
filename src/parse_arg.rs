use std::{env::Args, path::PathBuf};

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

    /// Returns the inputed file path if there is one
    /// returns 1 if not found
    pub fn get_file_path(&self) -> Result<PathBuf, i32> {
        if PathBuf::from(&self.args.last().unwrap()).exists() {
            return {
                if PathBuf::from(&self.args.last().unwrap()).is_file() {
                    Ok(PathBuf::from(&self.args.last().unwrap()))
                } else {
                    Err(1)
                }
            }
        } else {
            let option = self.get_arg_option("-f", Some("--file"));
            if option.is_ok() {
                Ok(PathBuf::from(option.unwrap()))
            } else {
                Err(1)
            }
        }
    }
}
