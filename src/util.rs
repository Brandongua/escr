macro_rules! ternary {
    ($truthy:expr, if ($comp:expr) else $falsy:expr) => {
        if $comp {
            $truthy
        } else {
            $falsy
        }
    };

}

macro_rules! str {
    ($text:literal) => {
        String::from($text)
    }
}

macro_rules! String_vec {
    ($($text:literal$(,)?)*) => {
        vec![$(str!($text),)*]
    }
}


pub(crate) use ternary;
pub(crate) use String_vec;
pub(crate) use str;
