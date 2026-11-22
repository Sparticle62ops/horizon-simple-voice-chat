//! svc_client: an unofficial, independent Simple Voice Chat client for
//! Horizon, MIT (c) 2026 sparticle. NOT affiliated with Max Henkel
//! (henkelmax), the author of Simple Voice Chat; it speaks the documented
//! Simple Voice Chat wire protocol (compatibility version 20) so it
//! interops with Java servers running the original mod. No code, assets or
//! lang files from the original Java mod are included.
//!
//! Attributions: the protocol module is derived from the voicechat-server
//! native mod in the horizon-mods registry (MIT, Ludovic Marie and the
//! Horizon contributors); voice is Opus via the vendored libopus 1.4
//! (Xiph.Org Foundation, BSD three-clause). Full notices in LICENSE.
//!
//! This root carries the mod API wiring on the pinned SDK (Horizon
//! f00125f61f, mixins e59da909, mod api 0.2): the launcher-facing entry.
//! It registers nothing and hooks nothing yet: the client mod API has no
//! net-channel, key/input, world-lifecycle, tick or command hook surface,
//! so no voice chat happens in game and commands (the /svc and /vcgroup
//! gate, which claims a command only for those two, see
//! `runtime::command_is_ours`) have nothing to run in. The logic runs its
//! tests; the seams stay recorded in the README and get wired the day
//! those surfaces land.
//!
//! Device capture/playback (`audio_io`, cpal) sits behind the optional
//! `audio` feature: it needs the host audio headers (ALSA on Linux), which
//! a build machine may not carry. Everything else, including the Opus
//! codec, protocol, session and runtime logic, builds and tests without
//! it.

pub mod audio;
#[cfg(feature = "audio")]
pub mod audio_io;
pub mod opus_ffi;
pub mod protocol;
pub mod runtime;
pub mod session;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {}
