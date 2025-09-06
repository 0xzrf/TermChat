mod communication;
mod errors;
mod helper;
mod user_onboard;

use tokio::runtime::Runtime;
pub use {communication::*, errors::OnboardErrors, helper::*, user_onboard::print_minimal_welcome};

pub fn run() -> Result<(), OnboardErrors> {
    print_minimal_welcome();
    let user_name = std::env::var("USER").unwrap_or("Guest".to_string());

    let mut communication = Communication::build(user_name);

    async_runtime(communication.user_response_onboarding())?;

    Ok(())
}

pub fn async_runtime<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}
