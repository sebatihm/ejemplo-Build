use crate::prefix::User;
pub trait Builder{
    type OutputType;
     fn set_users(&mut self,users:Vec<User>);
     fn set_keyboard_layout(&mut self,layout:&str);
     fn set_language(&mut self,language:&str);
     fn set_swap(&mut self,swap: bool);
     fn set_hostname(&mut self,host: &str);
     fn set_profiles(&mut self,profiles: Vec<String>);
     fn set_optional_repositories(&mut self,repos :Vec<String>);

     fn build(&self) -> Self::OutputType;
}

pub struct Director{}

impl Director{
    pub fn construct_i3(builder:&mut impl Builder){
        builder.set_users(vec![User::from("user","user",true)]);
        builder.set_keyboard_layout("en");
        builder.set_language("English");
        builder.set_swap(true);
        builder.set_hostname("Computer");
        builder.set_profiles(vec![String::from("i3")]);
        builder.set_optional_repositories(vec![String::from("git"),String::from("nano")]);
    }

    pub fn construct_cinnamon(builder:&mut impl Builder){
        builder.set_users(vec![User::from("usuario","usuario",true)]);
        builder.set_keyboard_layout("es");
        builder.set_language("Spanish");
        builder.set_swap(true);
        builder.set_hostname("Computer");
        builder.set_profiles(vec![String::from("Cinnamon")]);
        builder.set_optional_repositories(vec![String::from("git"),String::from("vim")]);
    }
}