use std::fmt;

use colored::*;

#[derive(Debug)]
pub enum TestResult<U>
where
    U: Eq + fmt::Display,
{
    Passed,
    Failed(U),
    Skipped,
}

impl<U> fmt::Display for TestResult<U>
where
    U: Eq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestResult::Passed => write!(f, "{}", "✅ Passed".green()),
            TestResult::Skipped => write!(f, "{}", "💤 Skipped".white().dimmed()),
            TestResult::Failed(actual) => write!(f, "{} {}", "❌ Failed with".red(), actual),
        }
    }
}

pub struct Problem<T, U>
where
    U: Eq + fmt::Display,
{
    test_fn: fn(T) -> U,
    skip_tests: bool,
}

impl<T, U> Problem<T, U>
where
    U: Eq + fmt::Display,
{
    pub fn new(test_fn: fn(T) -> U) -> Problem<T, U> {
        Problem {
            test_fn,
            skip_tests: false,
        }
    }

    pub fn without_tests(mut self) -> Problem<T, U> {
        self.skip_tests = true;
        self
    }

    pub fn test(&self, test: T, expect: U) -> TestResult<U> {
        if self.skip_tests {
            return TestResult::Skipped;
        }

        let result = (self.test_fn)(test);
        if result == expect {
            return TestResult::Passed;
        }
        TestResult::Failed(result)
    }

    pub fn run(&self, test: T) -> U {
        (self.test_fn)(test)
    }
}
