macro_rules! stringify_args {
    ($($arg:expr),*) => {
        {
            let mut s = String::new();
            $(
                s.push_str(format!("{} ", $arg).as_str()); 
            )*
            s 
        }
    }
}

macro_rules! info {
    ($($msg:expr),*) => {
        eprintln!("\u{2139} \x1b[38;2;194;208;252m {}\x1b[m", stringify_args!($($msg),*));
    }
}

macro_rules! warn {
    ($($msg:expr),*) => {
        eprintln!("\u{26a0} \x1b[33m{}\x1b[m", stringify_args!($($msg),*));
    }
}

macro_rules! error {
    ($($msg:expr),*) => {
        eprintln!("\u{274c} \x1b[31;1m{}\x1b[m", stringify_args!($($msg),*));
    }
}

macro_rules! success {
    ($($msg:expr),*) => {
        println!("\u{2705} \x1b[32;1m{}\x1b[m", stringify_args!($($msg),*));
    }
}

