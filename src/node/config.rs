
pub trait TConfig{
     fn new() -> Config;
     fn setId(&mut self, id:&str);
     fn setAddress(&mut self, address:&str);
     fn setPort(&mut self, port:u16);
     fn setDelay(&mut self, delay: u16);

     fn getId(&self) -> String;
     fn getAddress(&self) -> String;
     fn getPort(&self) -> u16;
     fn getDelay(&self) -> u16;
}

#[derive(Clone)]
pub struct Config{
    id: String,
    address: String,
    port:u16,
    delay:u16,
}

impl TConfig for Config{
    fn new() -> Config{
        Config{id:String::from(""),address:String::from(""),port:0, delay:0}
    }
    fn setId(&mut self, id:&str){
        self.id = String::from(id);
    }
    fn setAddress(&mut self, address:&str){
        self.address = String::from(address);
    }
    fn setPort(&mut self, port:u16){
        self.port = port;
    }
    fn setDelay(&mut self, delay: u16){
        self.delay = delay;
    }

    fn getId(&self) -> String{
        self.id.clone()
    }
    fn getAddress(&self) -> String{
        self.address.clone()
    }
    fn getPort(&self) -> u16{
        self.port
    }
    fn getDelay(&self) -> u16{
        self.delay
    }  
}
