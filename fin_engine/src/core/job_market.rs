//! Job market generation and management

use super::career::{Career, CareerField, Job, JobLevel};
use rust_decimal_macros::dec;

/// Generates job offers based on market and player qualifications
pub struct JobMarket;

impl JobMarket {
    /// Generates available jobs for Czech market
    /// Returns jobs that match or are slightly above player's qualifications
    pub fn generate_czech_jobs(career: &Career) -> Vec<Job> {
        let mut jobs = Vec::new();
        let max_level = career.max_qualified_level();
        let experience = career.years_experience;

        // Always include entry level jobs
        jobs.extend(Self::czech_entry_jobs());

        // Add jobs up to one level above current qualification (stretch opportunities)
        if experience >= 1 {
            jobs.extend(Self::czech_junior_jobs());
        }

        if experience >= 3 {
            jobs.extend(Self::czech_mid_jobs());
        }

        if experience >= 6 {
            jobs.extend(Self::czech_senior_jobs());
        }

        if experience >= 9 {
            jobs.extend(Self::czech_lead_jobs());
        }

        // Filter to show relevant jobs (current level and one above)
        let min_level_to_show = if experience >= 2 {
            JobLevel::Junior
        } else {
            JobLevel::Entry
        };

        jobs.into_iter()
            .filter(|job| {
                job.level as u8 >= min_level_to_show as u8 && job.level as u8 <= max_level as u8 + 1
            })
            .collect()
    }

    fn czech_entry_jobs() -> Vec<Job> {
        vec![
            Job::new(
                "cz_retail_entry".to_string(),
                "Sales Associate".to_string(),
                CareerField::Retail,
                JobLevel::Entry,
                dec!(25000), // 25k CZK/month
                Some("Local Store".to_string()),
            ),
            Job::new(
                "cz_admin_entry".to_string(),
                "Administrative Assistant".to_string(),
                CareerField::Other("Administration".to_string()),
                JobLevel::Entry,
                dec!(28000),
                Some("Office Corp".to_string()),
            ),
            Job::new(
                "cz_tech_entry".to_string(),
                "Junior IT Support".to_string(),
                CareerField::Technology,
                JobLevel::Entry,
                dec!(32000),
                Some("Tech Solutions s.r.o.".to_string()),
            ),
        ]
    }

    fn czech_junior_jobs() -> Vec<Job> {
        vec![
            Job::new(
                "cz_dev_junior".to_string(),
                "Junior Software Developer".to_string(),
                CareerField::Technology,
                JobLevel::Junior,
                dec!(45000),
                Some("CodeCraft Prague".to_string()),
            ),
            Job::new(
                "cz_accountant_junior".to_string(),
                "Junior Accountant".to_string(),
                CareerField::Finance,
                JobLevel::Junior,
                dec!(38000),
                Some("Finance Group".to_string()),
            ),
            Job::new(
                "cz_teacher_junior".to_string(),
                "Elementary School Teacher".to_string(),
                CareerField::Education,
                JobLevel::Junior,
                dec!(35000),
                Some("Praha Elementary".to_string()),
            ),
        ]
    }

    fn czech_mid_jobs() -> Vec<Job> {
        vec![
            Job::new(
                "cz_dev_mid".to_string(),
                "Software Developer".to_string(),
                CareerField::Technology,
                JobLevel::Mid,
                dec!(65000),
                Some("TechCorp Prague".to_string()),
            ),
            Job::new(
                "cz_accountant_mid".to_string(),
                "Accountant".to_string(),
                CareerField::Finance,
                JobLevel::Mid,
                dec!(52000),
                Some("KPMG Czech".to_string()),
            ),
            Job::new(
                "cz_manager_mid".to_string(),
                "Team Manager".to_string(),
                CareerField::Manufacturing,
                JobLevel::Mid,
                dec!(58000),
                Some("Škoda Auto".to_string()),
            ),
            Job::new(
                "cz_nurse_mid".to_string(),
                "Registered Nurse".to_string(),
                CareerField::Healthcare,
                JobLevel::Mid,
                dec!(48000),
                Some("Motol Hospital".to_string()),
            ),
        ]
    }

    fn czech_senior_jobs() -> Vec<Job> {
        vec![
            Job::new(
                "cz_dev_senior".to_string(),
                "Senior Software Engineer".to_string(),
                CareerField::Technology,
                JobLevel::Senior,
                dec!(90000),
                Some("Avast Software".to_string()),
            ),
            Job::new(
                "cz_accountant_senior".to_string(),
                "Senior Financial Analyst".to_string(),
                CareerField::Finance,
                JobLevel::Senior,
                dec!(75000),
                Some("Česká spořitelna".to_string()),
            ),
            Job::new(
                "cz_doctor_senior".to_string(),
                "Specialist Physician".to_string(),
                CareerField::Healthcare,
                JobLevel::Senior,
                dec!(85000),
                Some("General Hospital Prague".to_string()),
            ),
        ]
    }

    fn czech_lead_jobs() -> Vec<Job> {
        vec![
            Job::new(
                "cz_arch_lead".to_string(),
                "Lead Software Architect".to_string(),
                CareerField::Technology,
                JobLevel::Lead,
                dec!(120000),
                Some("O2 Czech Republic".to_string()),
            ),
            Job::new(
                "cz_cfo_lead".to_string(),
                "Finance Director".to_string(),
                CareerField::Finance,
                JobLevel::Lead,
                dec!(110000),
                Some("Česká pojišťovna".to_string()),
            ),
            Job::new(
                "cz_director_lead".to_string(),
                "Operations Director".to_string(),
                CareerField::Manufacturing,
                JobLevel::Lead,
                dec!(100000),
                Some("ČEZ Group".to_string()),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_jobs_for_new_player() {
        let career = Career::new();
        let jobs = JobMarket::generate_czech_jobs(&career);

        // Should only show entry level jobs
        assert!(!jobs.is_empty());
        assert!(jobs.iter().all(|j| j.level == JobLevel::Entry));
    }

    #[test]
    fn test_generate_jobs_with_experience() {
        let mut career = Career::new();
        career.years_experience = 5; // Qualifies for Mid level

        let jobs = JobMarket::generate_czech_jobs(&career);

        // Should show Junior and Mid level jobs (one above)
        assert!(jobs.iter().any(|j| j.level == JobLevel::Junior));
        assert!(jobs.iter().any(|j| j.level == JobLevel::Mid));

        // Should not show Entry anymore
        assert!(!jobs.iter().any(|j| j.level == JobLevel::Entry));
    }

    #[test]
    fn test_salary_progression() {
        let career = Career::new();

        // Entry level salary
        let entry_jobs = JobMarket::generate_czech_jobs(&career);
        let entry_max = entry_jobs.iter().map(|j| j.monthly_salary).max().unwrap();

        // Mid level salary
        let mut mid_career = Career::new();
        mid_career.years_experience = 5;
        let mid_jobs = JobMarket::generate_czech_jobs(&mid_career);
        let mid_max = mid_jobs.iter().map(|j| j.monthly_salary).max().unwrap();

        // Senior should pay more than entry
        assert!(mid_max > entry_max);
    }
}
