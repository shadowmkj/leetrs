//! Premium gate — checks whether a user can access a premium problem.
use crate::models::{ProblemSummary, UserDetail};

/// Checks whether the current user can access a given problem.
pub struct PremiumGate;

impl PremiumGate {
    /// Returns `Ok(())` if the user can access the problem, or `Err(message)`
    /// with a human-readable explanation if not.
    pub fn can_access(problem: &ProblemSummary, user: Option<&UserDetail>) -> Result<(), String> {
        if !problem.is_paid {
            return Ok(());
        }
        match user {
            Some(u) if u.is_premium == Some(true) => Ok(()),
            Some(_) => Err("This problem is premium. please subscribe to access it.".to_string()),
            None => Err(
                "This problem is premium. please login to access it. (use `leetrs auth`)"
                    .to_string(),
            ),
        }
    }
}
