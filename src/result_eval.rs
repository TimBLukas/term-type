use std::time::Duration;

pub struct TestResult {
    correct_chars: Vec<bool>,
    amount_correct_chars: u32,
    amount_wrong_chars: u32,
    percentage: f32,
    time: Duration,
    wpm: i32,
}

impl TestResult {
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

        format!(
            "Amount Correct: {}, Amount Wrong {}, Percentage {} %",
            amount_correct, amount_wrong, percentage
        )
    }
}
