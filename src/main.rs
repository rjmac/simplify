macro_rules! define_test {
    ($ver:ident) => {
        mod $ver {
            use ::$ver::*;

            pub fn go() {
                let unsimplified = line_string![
                    (x: 0.0, y: 0.0),
                    (x: 5.0, y: 4.0),
                    (x: 11.0, y: 5.5),
                    (x: 17.3, y: 3.2),
                    (x: 27.8, y: 0.1)
                ];
                let simplified = unsimplified.simplify(&30.0);
                println!("Before: {unsimplified:?}");
                println!("After: {simplified:?}");
            }
        }
    };
}

define_test!(geo23);
define_test!(geo24);

fn main() {
    geo23::go();
    geo24::go();
}
