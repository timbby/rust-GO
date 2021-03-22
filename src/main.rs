/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let x = add(1, 2);
/// ```
fn add(a: i32, b: i32) -> i32 {
    return a + b
}

fn main() {
    println!("Hello, world!");
    let a = 12;
    println!("a is {}", a);
    println!("1 + 2 is {}", add(1, 2));
    let b = 5;

    // if
    if b > 3 {
        println!("b is more than {}", 3);
    } else {
        println!("b is less than {}", 3);
    }

    // while
    let mut number = 1;
    while number < 4 {
        println!("number is {}", number);
        number += 1
    }

    // for
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("i is {}", i);
    }

    // loop
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        println!("i is {}", i);
    }

    // string slice
    let s = String::from("string");
    let part1 = &s[0..3];
    let part2 = &s[3..6];
    println!("part1 is {}, part2 is {}", part1, part2);


    // struct
    #[derive(Debug)]
    struct Site {
        protocol: String,
        domain: String,
        port: u32,
        path: String,
    }

    impl Site {
        // 结构体方法 用于实例
        fn get_url(&self) -> String {
            format!("{}//{}:{}{}", self.protocol, self.domain, self.port, self.path)
        }
        // 结构体关联函数
        fn create(protocol: &str, domain: &str, port: u32, path: &str) -> Site {
            Site {
                protocol: String::from(protocol),
                domain: String::from(domain),
                port,
                path: String::from(path)
            }
        }
    }

    let site = Site {
        protocol: String::from("https:"),
        domain: String::from("www.baidu.com"),
        port: 443,
        path: String::from("/"),
    };
    println!("{:?}", site);
    println!("{:#?}", site);
    println!("site url is {}", site.get_url());

    let site1 = Site::create("https:", "www.baidu.com", 443, "/");
    println!("{:#?}", site1);
    println!("site1 url is {}", site1.get_url());

    // enum
    #[derive(Debug)]
    enum Language {
        Rust,
        Python,
        Java,
    }

    let lang = Language::Rust;

    // match
    match lang {
        Language::Rust {} => {
            println!("language is {:?}", lang);
        },
        Language::Python {} => {
            println!("language is {:?}", lang);
        },
        Language::Java {} => {
            println!("language is {:?}", lang);
        },
    };

    // Option 枚举类
    enum Option<T> {
        Some(T),
        None,
    };

    let opt = Option::Some("Hello");

    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    let opt1 = Some("World");

    match opt1 {
        Some(something) => {
            println!("{}", something);
        },
        _ => {
            println!("opt is nothing");
        }
    }

    // 不可恢复的错误
    // panic!("error occured");

    // 可恢复的错误
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    use std::fs::File;
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

    let f1 = File::open("hello.txt");
    if let Ok(file) = f1 {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }

    // 向量
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    // 向量取值
    let mut v2: Vec<i32> = vec![16, 32, 64];
    vector.append(&mut v2);
    println!("{:?}", vector);
    println!("vector 0 is {}", match vector.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });

    println!("vector 4 is {}", vector[4]);

    // 遍历向量
    for i in &mut vector {
        println!("i is {}", i);
    }

    // 映射表
    use std::collections::HashMap;

    let mut map = HashMap::new();

    map.insert("color", "red");
    map.insert("size", "10 m^2");

    println!("{}", map.get("color").unwrap());

    // 映射表循环
    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }
}
