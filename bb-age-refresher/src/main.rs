use bb_age_attestation::types::{
    AGE_ATTESTATION_INTERFACE, AGE_ATTESTATION_OBJECT_PATH, AgeBracket,
};
use notify_rust::{Notification, Urgency};
use std::time::Duration;

mod popup;

async fn get_current_bracket() -> Option<AgeBracket> {
    let connection = zbus::Connection::system().await.ok()?;
    let reply = connection
        .call_method(
            Some(AGE_ATTESTATION_INTERFACE),
            AGE_ATTESTATION_OBJECT_PATH,
            Some(AGE_ATTESTATION_INTERFACE),
            "GetAgeBracket",
            &(),
        )
        .await
        .ok()?;
    let bracket_str: String = reply.body().deserialize().ok()?;
    AgeBracket::try_from(bracket_str.as_str()).ok()
}

fn send_reminder(bracket: &AgeBracket) -> bool {
    let body = format!("Is your age still {}?", bracket.label());

    let result = Notification::new()
        .summary("Age Attestation")
        .body(&body)
        .urgency(Urgency::Critical)
        .timeout(Duration::from_secs(60))
        .action("yes", "Yes, I'm still that age")
        .action("no", "No, my age has changed")
        .show();

    match result {
        Ok(handle) => {
            let mut answered_yes = true;
            handle.wait_for_action(|action| {
                answered_yes = action == "yes" || action == "__closed";
            });
            answered_yes
        }
        Err(_) => true,
    }
}

fn main() {
    if std::env::args().any(|a| a == "--preview") {
        popup::preview_popup();
        return;
    }

    // Run the async D-Bus lookup, then drop the runtime before blocking GUI calls
    let bracket = tokio::runtime::Runtime::new()
        .expect("failed to create tokio runtime")
        .block_on(get_current_bracket());

    let needs_popup = match bracket {
        Some(b) => {
            if send_reminder(&b) {
                false
            } else {
                !popup::show_age_popup()
            }
        }
        None => !popup::show_age_popup(),
    };

    if needs_popup {
        std::process::exit(1);
    }
}
