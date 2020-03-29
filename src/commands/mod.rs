pub mod init;
pub mod new;

pub trait Exec {
    fn exec(&self); 
}

