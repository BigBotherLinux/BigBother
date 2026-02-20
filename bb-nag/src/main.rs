use notify_rust::{Notification, Urgency};
use rand::prelude::*;
use std::thread;
use std::time::Duration;

const MANUAL_OVERRIDE: &[&str] = &[
    "You are now breathing manually.",
    "You are now blinking manually.",
    "You are now aware of your tongue resting in your mouth.",
    "You can always see your nose. Your brain just ignores it. Not anymore.",
    "You are now swallowing manually.",
    "You are now aware of the weight of your jaw.",
    "You can feel your clothes touching your skin.",
    "You can feel your toes touching each other.",
    "You are now aware of how often you swallow.",
    "You just became aware of your own blinking rhythm.",
];

const UNCOMFORTABLE_TRUTHS: &[&str] = &[
    "You just lost The Game.",
    "Every 'C' in 'Pacific Ocean' is pronounced differently.",
    "Your stomach acid is strong enough to dissolve metal. It's inside you right now.",
    "There are more planes in the ocean than submarines in the sky.",
    "You have never seen your own face. Only reflections and photos.",
    "Your teeth are the only part of your skeleton you clean.",
    "You were once the youngest person on Earth.",
    "Somewhere out there, a spider has memorized your sleep schedule.",
    "Your bed is a shelf you put yourself on at night.",
    "The average person walks past 36 murderers in their lifetime.",
];

const UNSETTLING: &[&str] = &[
    "When was the last time you looked behind you?",
    "Did you leave the stove on?",
];

const EXISTENTIAL: &[&str] = &[
    "You have mass. You are being pulled toward the sun right now.",
    "Your organs have never seen the light of day.",
    "Every decision you've ever made has led you to reading this notification.",
    "You are a mass of atoms that somehow learned to be anxious.",
    "The universe existed for billions of years before you and will continue for billions after.",
    "You are the universe experiencing itself.",
];

struct MessagePool {
    categories: Vec<(&'static str, &'static [&'static str])>,
}

impl MessagePool {
    fn new() -> Self {
        Self {
            categories: vec![
                ("Manual Override", MANUAL_OVERRIDE),
                ("Did You Know?", UNCOMFORTABLE_TRUTHS),
                ("\u{26a0}\u{fe0f} Alert", UNSETTLING),
                ("Existential Reminder", EXISTENTIAL),
            ],
        }
    }

    fn random_message(&self, rng: &mut impl Rng) -> (&'static str, &'static str) {
        let (title, messages) = self.categories.choose(rng).unwrap();
        let message = messages.choose(rng).unwrap();
        (title, message)
    }
}

const USELESS_ACTIONS: &[(&str, &str)] = &[
    ("ok", "OK"),
    ("acknowledge", "I Understand"),
    ("noted", "Noted"),
    ("why", "Why?"),
    ("stop", "Please Stop"),
    ("thanks", "Thanks"),
    ("cool", "Cool"),
    ("hmm", "Hmm"),
];

const DISMISS_RESPONSES: &[&str] = &[
    "You clicked the button. Nothing happened. You knew it wouldn't.",
    "Your input has been noted and discarded.",
    "Thank you for your feedback. It means nothing to us.",
    "That button was purely decorative.",
];

fn send_notification(pool: &MessagePool, rng: &mut impl Rng) {
    let (title, body) = pool.random_message(rng);

    let num_actions = rng.random_range(1..=2);
    let actions: Vec<&(&str, &str)> = USELESS_ACTIONS
        .sample(rng, num_actions)
        .collect();

    let urgency = match rng.random_range(0..10u32) {
        0 => Urgency::Critical,
        1..=3 => Urgency::Normal,
        _ => Urgency::Low,
    };

    let mut notification = Notification::new();
    notification
        .summary(title)
        .body(body)
        .urgency(urgency)
        .timeout(if urgency == Urgency::Critical { 0 } else { 10000 });

    for (id, label) in &actions {
        notification.action(id, label);
    }

    match notification.show() {
        Ok(handle) => {
            handle.wait_for_action(|action| {
                if action != "__closed" {
                    let response = DISMISS_RESPONSES.choose(rng).unwrap();
                    let _ = Notification::new()
                        .summary("Response")
                        .body(response)
                        .urgency(Urgency::Low)
                        .timeout(5000)
                        .show();
                }
            });
        }
        Err(e) => eprintln!("Failed to send notification: {}", e),
    }
}

fn main() {
    let mut rng = rand::rng();
    let pool = MessagePool::new();

    println!("bb-nag: Department of Unsolicited Reminders is now operational.");

    loop {
        send_notification(&pool, &mut rng);

        let delay_secs = rng.random_range(60u64..=900);
        println!("Next notification in {} seconds", delay_secs);
        thread::sleep(Duration::from_secs(delay_secs));
    }
}
