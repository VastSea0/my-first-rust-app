use std::fmt::{Display, Formatter};
use crate::level::Level;

#[derive(Debug)]
pub struct Player<'a> {
    pub name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a>{
    pub fn new(name: &'a str, level: Level) -> Self{
        Self{
            name,
            level
        }
    }

    // pub fn to_string(&self) -> String {
    //     format!("{} | {:?} ", self.name, self.level)
    // }

}

impl Display for Player<'_>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {:?}",self.name,self.level)
    }
}