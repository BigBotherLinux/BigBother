use bevy::prelude::*;
use bevy::tasks::{futures::check_ready, IoTaskPool, Task};
use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkConnectivityStatus>()
            .init_resource::<PendingCheck>()
            .add_systems(
                Update,
                (
                    tick_watch.run_if(resource_exists::<NetworkWatch>),
                    finish_check,
                )
                    .chain(),
            );
    }
}

#[derive(Component, Default, Clone)]
pub struct NetworkConnectivityLabel;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum NetworkConnectivityStatus {
    Online,
    Offline,
    #[default]
    Unknown,
}

impl std::fmt::Display for NetworkConnectivityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// While this resource exists, connectivity is checked every `interval`.
/// Nothing runs when it is absent.
///
/// Start (or restart) with `commands.insert_resource(NetworkWatch::every(..))`,
/// stop early with `commands.remove_resource::<NetworkWatch>()`. With `lasting`
/// it removes itself once the duration is up.
#[derive(Resource, Debug)]
pub struct NetworkWatch {
    interval: Timer,
    /// `None` means keep going until removed.
    lifetime: Option<Timer>,
}

impl NetworkWatch {
    /// A single check, right away.
    pub fn once() -> Self {
        // The interval fires on the first tick and the zero lifetime ends the
        // watch on that same tick, so the interval length never matters.
        Self::every(Duration::from_secs(1)).lasting(Duration::ZERO)
    }

    /// Check now, then again every `interval`.
    pub fn every(interval: Duration) -> Self {
        info!("NetworkWatch: every({:?})", interval);
        let mut interval = Timer::new(interval, TimerMode::Repeating);
        // Fire on the first tick instead of waiting a full interval.
        interval.almost_finish();
        Self {
            interval,
            lifetime: None,
        }
    }

    /// Stop after `duration`.
    pub fn lasting(mut self, duration: Duration) -> Self {
        self.lifetime = Some(Timer::new(duration, TimerMode::Once));
        self
    }
}

/// The check currently in flight. Lives outside `NetworkWatch` so a result
/// still lands after the watch has ended.
#[derive(Resource, Default)]
struct PendingCheck(Option<Task<bool>>);

fn tick_watch(
    mut commands: Commands,
    time: Res<Time>,
    mut watch: ResMut<NetworkWatch>,
    mut pending: ResMut<PendingCheck>,
) {
    // Skip a tick if the previous check is still running rather than piling up.
    if watch.interval.tick(time.delta()).just_finished() && pending.0.is_none() {
        pending.0 = Some(IoTaskPool::get().spawn(async { is_online() }));
    }

    if let Some(lifetime) = &mut watch.lifetime {
        if lifetime.tick(time.delta()).is_finished() {
            commands.remove_resource::<NetworkWatch>();
        }
    }
}

fn finish_check(mut pending: ResMut<PendingCheck>, mut status: ResMut<NetworkConnectivityStatus>) {
    let Some(task) = pending.0.as_mut() else {
        return;
    };
    if let Some(online) = check_ready(task) {
        pending.0 = None;
        status.set_if_neq(if online {
            NetworkConnectivityStatus::Online
        } else {
            NetworkConnectivityStatus::Offline
        });
    }
}

/// Blocking; only call from a task. A TCP connect needs no privileges, unlike
/// a real ICMP ping.
fn is_online() -> bool {
    let addr = SocketAddr::from(([1, 1, 1, 1], 443));
    TcpStream::connect_timeout(&addr, Duration::from_secs(2)).is_ok()
}
