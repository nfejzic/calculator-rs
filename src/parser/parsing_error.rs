use std::fmt;

pub struct ParsingError {
    pub text: String,
    pub index: usize,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding = String::from_utf8(vec![b' '; self.index + 1]).unwrap();

        f.write_fmt(format_args!("{:?}\n", self.text))?;
        f.write_fmt(format_args!("{}|\n", padding))?;
        f.write_fmt(format_args!(
            "{}Syntax Error, unexpected character!\n",
            padding
        ))?;
        f.write_fmt(format_args!(
            "{}Please use only decimal numbers and one of the following operations: \n",
            padding,
        ))?;
        f.write_fmt(format_args!(
            "{}+ (Add), - (Subtract), * (Multiply), / (Divide), % (Modulo)",
            padding
        ))?;

        Ok(())
    }
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        println!("{}", self);
        Ok(())
    }
}
