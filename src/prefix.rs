use crate::builders::Builder;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct User{
    name: String,
    password: String,
    sudoer: bool
}

impl User{
    pub fn get_name(&self)->String{
        return self.name.clone();
    }

    pub fn get_password(&self)->String{
        return self.password.clone();
    }

    pub fn is_sudoer(&self) -> bool{
        return self.sudoer;
    }

    pub fn from(name:&str,password:&str,sudoer:bool) -> Self{
        Self { 
            name: String::from(name), 
            password: String::from(password), 
            sudoer: sudoer 
        }
    }
}

impl std::fmt::Debug for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{{\n name: {}\n sudoer: {} \n password: {} \n}}",self.name,self.is_sudoer(),self.password)?;
        Ok(())
    }
}

pub struct SystemPrefix{
    users: Vec<User>,
    keyboard_layout: String,
    language: String,
    swap: bool,
    hostname: String,
    profile: Vec<String>,
    optional_repositories: Vec<String>
}

impl SystemPrefix{
    

    pub fn new() ->Self{
        Self{
            users: vec![User::from("seba", "password", false)],
            keyboard_layout: String::from("es"),
            language: String::from("es"),
            swap: false,
            hostname: String::from("navi"),
            profile: vec![String::from("i3")],
            optional_repositories: vec![String::from("i3")]
        }
    }

    fn from(users: Vec<User>, keyboard_layout: String, language: String,swap: bool,hostname: String,profile: Vec<String>,repos: Vec<String>) -> Self{
        Self {
            users: users,
            keyboard_layout: keyboard_layout,
            language: language,
            swap: swap,
            hostname: hostname,
            profile: profile,
            optional_repositories: repos
        }

        
    }

    pub fn save_prefix(&self,filename: &str)-> std::io::Result<()>{
        
        let mut archivo = File::create(filename)?;
        let er = self.to_string();
        archivo.write_all(er.as_bytes())?;
        Ok(())
    }
}

impl std::fmt::Display for SystemPrefix{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"linux_system_config  {{")?;
        writeln!(f, "Users {{ {:?} }}" ,self.users)?;
        writeln!(f, "Keyboard layout: {}",self.keyboard_layout)?;
        writeln!(f,"Language: {}", self.language)?;
        writeln!(f,"Swap: {}", self.swap)?;
        writeln!(f,"Hostname: {}", self.hostname)?;
        writeln!(f,"Profiles {{ {:?} }}", self.profile)?;
        writeln!(f, "Optional Repositories: {:?}", self.optional_repositories)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct PrefixBuilder{
    users: Vec<User>,
    keyboard_layout: String,
    language: String,
    swap: bool,
    hostname: String,
    profile: Vec<String>,
    optional_repositories: Vec<String>
}

impl Builder for PrefixBuilder {
    type OutputType = SystemPrefix;

    fn set_users(&mut self,users:Vec<User>) {
        self.users = users;
    }

    fn set_keyboard_layout(&mut self,layout:&str) {
        self.keyboard_layout = String::from(layout);
    }

    fn set_language(&mut self,language:&str) {
        self.language = String::from(language);
    }

    fn set_swap(&mut self,swap: bool) {
        self.swap = swap;
    }

    fn set_hostname(&mut self,host: &str) {
        self.hostname = String::from(host);
    }

    fn set_profiles(&mut self,profiles: Vec<String>) {
        self.profile = profiles;
    }

    fn set_optional_repositories(&mut self,repos :Vec<String>) {
        self.optional_repositories = repos;
    }

    fn build(&self) -> Self::OutputType {
        SystemPrefix::from(self.users.clone(), 
            self.keyboard_layout.clone(),
            self.language.clone(),
            self.swap.clone(),
            self.hostname.clone(),
            self.profile.clone(),
            self.optional_repositories.clone())
    }
}
