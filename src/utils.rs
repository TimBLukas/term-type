use crate::result_eval::TestResult;

pub fn get_ascii_art<'a>() -> &'a str {
    r#"
  *   )                      *   )                 
` )  /(  (  (      )       ` )  /((            (   
 ( )(_))))\ )(    (     ___ ( )(_))\ ) `  )   ))\  
(_(_())/((_|()\   )\  '|___(_(_()|()/( /(/(  /((_) 
|_   _(_))  ((_)_((_))     |_   _|)(_)|(_)_\(_))   
  | | / -_)| '_| '  \()      | | | || | '_ \) -_)  
  |_| \___||_| |_|_|_|       |_|  \_, | .__/\___|  
                                  |__/|_|          
"#
}

pub fn get_start_screen(language: &str, words: u32, sensible: bool) -> String {
    format!(
        r#"
┌──────────────────────────────────────────────────────────────┐
│                      RUST TYPING TEST                        │
├──────────────────────────────────────────────────────────────┤
│ Sprache            │ {:<39} │
│ Wörter             │ {:<39} │
│ Sinnvolle Sätze    │ {:<39} │
└──────────────────────────────────────────────────────────────┘
"#,
        language,
        words,
        if sensible { "Yes" } else { "No" },
    )
}

pub fn get_test_summary(test_result: &mut TestResult) -> String {
    test_result.eval_correct_chars();

    format!(
        r#"
┌──────────────────────────────────────────────────────────────┐
│                      TEST RESULTS                            │
├──────────────────────────────────────────────────────────────┤
│ Time              │ {:<40} │
│ Correct Chars     │ {:<40} │
│ Wrong Chars       │ {:<40} │
│ Percentage        │ {:<40} │
│ WPM               │ {:<40} │
└──────────────────────────────────────────────────────────────┘
"#,
        format!("{:.2} s", test_result.time.as_secs_f32()),
        test_result.amount_correct_chars,
        test_result.amount_wrong_chars,
        format!("{:.2} %", test_result.percentage),
        test_result.wpm,
    )
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
