#[cfg(test)]
#[derive(Debug, PartialEq)]
pub struct Error1(String);
impl Error1 {
    pub fn new(msg: &str) -> Error1 {
        Error1(msg.to_string())
    }
}
impl From<Error1> for Error2 {
    fn from(value: Error1) -> Self {
        Error2("Converted Error1(".to_string() + &value.0 + ")")
    }
}

#[cfg(test)]
#[derive(Debug, PartialEq)]
pub struct Error2(String);
impl Error2 {
    pub fn new(msg: &str) -> Error2 {
        Error2(msg.to_string())
    }
}
impl From<Error2> for Error1 {
    fn from(value: Error2) -> Self {
        Error1("Converted Error2(".to_string() + &value.0 + ")")
    }
}
