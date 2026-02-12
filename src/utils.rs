pub fn get_ascii_art<'a>() -> &'a str {
    return r#"
  *   )                      *   )                 
` )  /(  (  (      )       ` )  /((            (   
 ( )(_))))\ )(    (     ___ ( )(_))\ ) `  )   ))\  
(_(_())/((_|()\   )\  '|___(_(_()|()/( /(/(  /((_) 
|_   _(_))  ((_)_((_))     |_   _|)(_)|(_)_\(_))   
  | | / -_)| '_| '  \()      | | | || | '_ \) -_)  
  |_| \___||_| |_|_|_|       |_|  \_, | .__/\___|  
                                  |__/|_|          
"#;
}

#[macro_export]
macro_rules! pause {
    ($expression:expr) => {
        println!("[{}:{}] {}", file!(), line!(), $expression);

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    };
}
