use std::fmt;

#[derive(Debug)]
pub enum TestResult<U> where U: Eq + fmt::Display {
    Passed,
    Failed(U),
    Skipped,
}

impl<U> fmt::Display for TestResult<U> where U: Eq + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestResult::Passed => write!(f, "Passed"),
            TestResult::Skipped => write!(f, "Skipped"),
            TestResult::Failed(actual) => write!(f, "Failed with {}", actual),
        }
    }
}

pub struct Problem<T, U> where U: Eq + fmt::Display {
    test_fn: fn(T) -> U,
    skip_tests: bool,
}

impl<T, U> Problem<T, U> where U: Eq + fmt::Display {
    pub fn new(test_fn: fn(T) -> U) -> Problem<T, U> { 
        Problem {
            test_fn,
            skip_tests: false,
        }
     }

    pub fn skip(self) -> Problem<T, U> {
        Problem {
            test_fn: self.test_fn,
            skip_tests: true,
        }
    }

    pub fn test(self, test: T, expect: U) -> TestResult<U> {
        if self.skip_tests {
            return TestResult::Skipped;
        }

        let result = (self.test_fn)(test);
        if result == expect {
            return TestResult::Passed;
        }
        TestResult::Failed(result)
    }
}
