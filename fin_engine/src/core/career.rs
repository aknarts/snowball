//! Career and job system

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Job level/seniority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobLevel {
    /// Entry level (0-2 years experience)
    Entry,
    /// Junior level (2-4 years experience)
    Junior,
    /// Mid level (4-7 years experience)
    Mid,
    /// Senior level (7-10 years experience)
    Senior,
    /// Lead level (10+ years experience)
    Lead,
}

impl JobLevel {
    /// Returns minimum years of experience required
    pub fn min_experience(&self) -> u8 {
        match self {
            JobLevel::Entry => 0,
            JobLevel::Junior => 2,
            JobLevel::Mid => 4,
            JobLevel::Senior => 7,
            JobLevel::Lead => 10,
        }
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            JobLevel::Entry => "Entry Level",
            JobLevel::Junior => "Junior",
            JobLevel::Mid => "Mid-Level",
            JobLevel::Senior => "Senior",
            JobLevel::Lead => "Lead",
        }
    }

    /// Returns all job levels in order
    pub fn all() -> Vec<JobLevel> {
        vec![
            JobLevel::Entry,
            JobLevel::Junior,
            JobLevel::Mid,
            JobLevel::Senior,
            JobLevel::Lead,
        ]
    }
}

/// Career field/industry
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CareerField {
    Technology,
    Finance,
    Healthcare,
    Education,
    Retail,
    Manufacturing,
    Other(String),
}

impl CareerField {
    /// Returns the display name
    pub fn name(&self) -> String {
        match self {
            CareerField::Technology => "Technology".to_string(),
            CareerField::Finance => "Finance".to_string(),
            CareerField::Healthcare => "Healthcare".to_string(),
            CareerField::Education => "Education".to_string(),
            CareerField::Retail => "Retail".to_string(),
            CareerField::Manufacturing => "Manufacturing".to_string(),
            CareerField::Other(name) => name.clone(),
        }
    }

    /// Returns available career fields
    pub fn available_fields() -> Vec<CareerField> {
        vec![
            CareerField::Technology,
            CareerField::Finance,
            CareerField::Healthcare,
            CareerField::Education,
            CareerField::Retail,
            CareerField::Manufacturing,
        ]
    }
}

/// A job offer or position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    /// Unique identifier
    pub id: String,
    /// Job title (e.g., "Software Developer", "Accountant")
    pub title: String,
    /// Career field
    pub field: CareerField,
    /// Job level/seniority
    pub level: JobLevel,
    /// Monthly gross salary
    pub monthly_salary: Decimal,
    /// Minimum years of experience required
    pub required_experience: u8,
    /// Company name (optional)
    pub company: Option<String>,
}

impl Job {
    /// Creates a new job
    pub fn new(
        id: String,
        title: String,
        field: CareerField,
        level: JobLevel,
        monthly_salary: Decimal,
        company: Option<String>,
    ) -> Self {
        Job {
            id,
            title,
            field,
            required_experience: level.min_experience(),
            level,
            monthly_salary,
            company,
        }
    }

    /// Checks if the player qualifies for this job
    pub fn qualifies(&self, years_experience: u8) -> bool {
        years_experience >= self.required_experience
    }

    /// Returns the job level name
    pub fn level_name(&self) -> &'static str {
        self.level.name()
    }
}

/// Player's career information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Career {
    /// Current job (if employed)
    pub current_job: Option<Job>,
    /// Total years of work experience
    pub years_experience: u8,
    /// Months in current job
    pub months_in_current_job: u8,
    /// Previous jobs (job history)
    pub job_history: Vec<Job>,
}

impl Career {
    /// Creates a new career with no employment
    pub fn new() -> Self {
        Career {
            current_job: None,
            years_experience: 0,
            months_in_current_job: 0,
            job_history: Vec::new(),
        }
    }

    /// Accepts a new job
    pub fn accept_job(&mut self, job: Job) {
        // If currently employed, add to history
        if let Some(current) = self.current_job.take() {
            self.job_history.push(current);
        }

        self.current_job = Some(job);
        self.months_in_current_job = 0;
    }

    /// Quits the current job
    pub fn quit_job(&mut self) {
        if let Some(job) = self.current_job.take() {
            self.job_history.push(job);
        }
        self.months_in_current_job = 0;
    }

    /// Advances career by one month (call at end of month)
    pub fn advance_month(&mut self) {
        if self.current_job.is_some() {
            self.months_in_current_job += 1;

            // Every 12 months, gain 1 year of experience
            if self.months_in_current_job % 12 == 0 {
                self.years_experience += 1;
            }
        }
    }

    /// Returns true if currently employed
    pub fn is_employed(&self) -> bool {
        self.current_job.is_some()
    }

    /// Returns current monthly salary (0 if unemployed)
    pub fn monthly_salary(&self) -> Decimal {
        self.current_job
            .as_ref()
            .map(|j| j.monthly_salary)
            .unwrap_or(Decimal::ZERO)
    }

    /// Returns the highest job level the player qualifies for
    pub fn max_qualified_level(&self) -> JobLevel {
        for level in JobLevel::all().iter().rev() {
            if self.years_experience >= level.min_experience() {
                return *level;
            }
        }
        JobLevel::Entry
    }
}

impl Default for Career {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_job_level_requirements() {
        assert_eq!(JobLevel::Entry.min_experience(), 0);
        assert_eq!(JobLevel::Junior.min_experience(), 2);
        assert_eq!(JobLevel::Mid.min_experience(), 4);
        assert_eq!(JobLevel::Senior.min_experience(), 7);
        assert_eq!(JobLevel::Lead.min_experience(), 10);
    }

    #[test]
    fn test_job_qualification() {
        let job = Job::new(
            "job1".to_string(),
            "Senior Developer".to_string(),
            CareerField::Technology,
            JobLevel::Senior,
            dec!(80000),
            None,
        );

        assert!(!job.qualifies(5)); // 5 years < 7 required
        assert!(job.qualifies(7)); // exactly 7 years
        assert!(job.qualifies(10)); // more than enough
    }

    #[test]
    fn test_career_progression() {
        let mut career = Career::new();
        assert!(!career.is_employed());
        assert_eq!(career.years_experience, 0);

        let job = Job::new(
            "job1".to_string(),
            "Junior Dev".to_string(),
            CareerField::Technology,
            JobLevel::Junior,
            dec!(40000),
            None,
        );

        career.accept_job(job);
        assert!(career.is_employed());
        assert_eq!(career.monthly_salary(), dec!(40000));

        // Advance 12 months
        for _ in 0..12 {
            career.advance_month();
        }

        assert_eq!(career.years_experience, 1);
        assert_eq!(career.months_in_current_job, 12);
    }

    #[test]
    fn test_job_switching() {
        let mut career = Career::new();

        let job1 = Job::new(
            "job1".to_string(),
            "Junior Dev".to_string(),
            CareerField::Technology,
            JobLevel::Junior,
            dec!(40000),
            None,
        );

        career.accept_job(job1);
        assert_eq!(career.job_history.len(), 0);

        let job2 = Job::new(
            "job2".to_string(),
            "Mid Dev".to_string(),
            CareerField::Technology,
            JobLevel::Mid,
            dec!(60000),
            None,
        );

        career.accept_job(job2);
        assert_eq!(career.job_history.len(), 1);
        assert_eq!(career.monthly_salary(), dec!(60000));
    }

    #[test]
    fn test_max_qualified_level() {
        let mut career = Career::new();
        assert_eq!(career.max_qualified_level(), JobLevel::Entry);

        career.years_experience = 3;
        assert_eq!(career.max_qualified_level(), JobLevel::Junior);

        career.years_experience = 8;
        assert_eq!(career.max_qualified_level(), JobLevel::Senior);

        career.years_experience = 15;
        assert_eq!(career.max_qualified_level(), JobLevel::Lead);
    }
}
