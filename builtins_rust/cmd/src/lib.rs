use std::collections::HashMap;
use std::sync::Mutex;
#[macro_use]
extern crate lazy_static;

pub struct Cmd {
    name:String,
    enable:bool,
}
impl Cmd {
pub fn build_cmd(name:String, enable:bool) -> Cmd {
    Cmd {
        name,
        enable,
    }
}

pub fn set_enable(&mut self, is_enable:bool) -> &Cmd {
    self.enable = is_enable;
    self
}
pub fn get_enable(&self) -> bool {
    return self.enable
}
}

lazy_static! {
static ref CMD: Mutex<HashMap<String,Box<Cmd>>> = Mutex::new(HashMap::new());
}
fn init_cmd_table() {
}

pub fn insert_empty_cmd(cmd:String) -> bool {
    let it = Cmd::build_cmd(String::from(&cmd[..]),true);
    let mut cmd_table = CMD.lock().unwrap();
    cmd_table.insert(String::from(&cmd[..]), Box::new(it));
    return true;
}
pub fn insert_cmd(cmd:&str, item: Box<Cmd>) -> Option<Box<Cmd>>{
    let mut cmd_table = CMD.lock().unwrap();
    cmd_table.insert(String::from(cmd), item)
}

pub fn get_cmd_enable(cmd:String) -> Result<bool,String>{
    let mut cmd_table = CMD.lock().unwrap();
    let item = cmd_table.get(&cmd);
    match item {
        Some(c) => {
            println!("name:{}, is {}!", cmd, c.enable);

            return Ok(c.enable);
        }
        None => {
            return Err(cmd+" not found");
        }
    }
}

pub fn set_cmd_enable(cmd :String, is_enable:bool)->bool {
    let mut a = CMD.lock().unwrap();
    let v = a.get_mut(&cmd);
    match v {
        Some(c) => {
            c.enable = is_enable;
        }
        None => {
            return false;
        }
    }
    return true;
}
