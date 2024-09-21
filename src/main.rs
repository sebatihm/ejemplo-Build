pub mod prefix;
pub mod builders;

use builders::Builder;



use crate::prefix::*;
use crate::builders::Director;
fn main() {
    let mut builder = PrefixBuilder::default();
    Director::construct_cinnamon(&mut builder);
    let distro = builder.build();
    let _ = distro.save_prefix("texto.txt");

    
}



