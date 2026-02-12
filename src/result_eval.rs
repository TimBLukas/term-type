use std::time::Duration;

pub struct TestResult {
    pub correct_chars: Vec<bool>,
    pub amount_correct_chars: u32,
    pub amount_wrong_chars: u32,
    pub percentage: f32,
    pub time: Duration,
    pub wpm: f32,
}

impl Default for TestResult {
    fn default() -> Self {
        TestResult {
            correct_chars: Vec::new(),
            amount_correct_chars: 0,
            amount_wrong_chars: 0,
            percentage: 0.0,
            time: Duration::new(0, 0),
            wpm: 0.0,
        }
    }
}

impl TestResult {
    pub fn new(
        correct_chars: Vec<bool>,
        amount_correct_chars: u32,
        amount_wrong_chars: u32,
        percentage: f32,
        time: Duration,
        wpm: f32,
    ) -> Self {
        TestResult {
            correct_chars,
            amount_correct_chars,
            amount_wrong_chars,
            percentage,
            time,
            wpm,
        }
    }
    pub fn eval_correct_chars(&mut self) -> String {
        let mut amount_correct = 0;
        let mut amount_wrong = 0;

        for val in &self.correct_chars {
            if *val {
                amount_correct += 1;
            } else {
                amount_wrong += 1;
            }
        }

        let percentage: f32 =
            (amount_correct as f32 / (amount_wrong as f32 + amount_correct as f32)) * 100.0;

        self.amount_correct_chars = amount_correct;
        self.amount_wrong_chars = amount_wrong;
        self.percentage = percentage;

        self.calc_wpm();

        format!(
            "Amount Correct: {}, Amount Wrong {}, Percentage {} %, this is a typing speed of {} WPM",
            self.amount_correct_chars, self.amount_wrong_chars, self.percentage, self.wpm
        )
    }

    fn calc_wpm(&mut self) {
        // Formula: (Total Characters/5)/Time in Minutes
        self.wpm = (self.amount_correct_chars as f32 / 5.0) / (self.time.as_secs_f32() / 60.0);
        println!("{}", self.wpm);
    }
}
