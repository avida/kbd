#![allow(dead_code)]
use serde::Deserialize;
use toml::Table;

mod parser;

#[derive(Deserialize, Debug)]
struct Config {
    main: Table,
}

fn read_config(path: &str) -> Result<Config, &'static str> {
    let config_str = std::fs::read_to_string(path).expect("Failed to read config.toml");
    if let Ok(config) = toml::from_str(config_str.as_str()) {
        Ok(config)
    } else {
        Err("sefwef")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config: Config = read_config("config.toml").unwrap();
        println!("{:?}", config);
        println!("{}", config.main);
    }

    #[test]
    #[ignore = "Rust playground"]
    fn test_something() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
        let inp = "leftctrl down + wait(500) + leftctrl up";
        let mut v = vec![1, 2, 3, 5];
        let mut ccc = || {
            println!("closure {v:?}");
            v.push(666);
        };

        fn ppp(v: &mut Vec<i32>) {
            println!("func ppp");
            v.push(1);
            println!("{v:?}");
        }
        ccc();
        ppp(&mut v);
        v.sort_by(|a, b| b.cmp(a));
        println!("{v:?}");
        // let novel = String::from("Call me Ishmael. Some years ago...");
        fn fff(sss: &str) -> ImportantExcerpt {
            let first_sentence = sss.split(' ').nth(1).unwrap();
            let i = ImportantExcerpt {
                part: first_sentence,
            };
            println!("fff: {}", i.part);
            println!("Address of i: {:p}", &i);

            i
        }

        let a = 123;
        println!("Address of a {:p}", &a);
        let i1 = inp.as_ptr() as usize;
        println!("Memory address of inp: {:p}", inp);
        let i = fff(inp);
        println!("Address of i: {:p}", &i);
        let i2 = i.part.as_ptr() as usize;
        let heap_var = Box::new(42);
        println!("Address of heap_var: {:p}", &*heap_var);
        drop(heap_var);

        println!("return : {} {i2}", i.part);
        println!("dif {}", i2 - i1);

        println!("{}", first_word(inp));
        struct Colors {}

        impl Colors {
            pub const RED: &'static str = "Red";
            pub const GREEN: &'static str = "Green";
            pub const BLUE: &'static str = "Blue";
        }
        println!("{}", Colors::RED);
    }
}
