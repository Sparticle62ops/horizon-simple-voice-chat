#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/advancements.rs"] pub mod advancements;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/block_entities.rs"] pub mod block_entities;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/block_states.rs"] pub mod block_states;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/boss_bars.rs"] pub mod boss_bars;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/client_input.rs"] pub mod client_input;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/client_narrator.rs"] pub mod client_narrator;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/client_options.rs"] pub mod client_options;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/client_render.rs"] pub mod client_render;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/client_text.rs"] pub mod client_text;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/client_view.rs"] pub mod client_view;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/config.rs"] pub mod config;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/data_maps.rs"] pub mod data_maps;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/displays.rs"] pub mod displays;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/draw.rs"] pub mod draw;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/effects.rs"] pub mod effects;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/entities.rs"] pub mod entities;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/fluids.rs"] pub mod fluids;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/fluids_energy.rs"] pub mod fluids_energy;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/item_entities.rs"] pub mod item_entities;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/item_particles.rs"] pub mod item_particles;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/items.rs"] pub mod items;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/keys.rs"] pub mod keys;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/levels.rs"] pub mod levels;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/living.rs"] pub mod living;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/math.rs"] pub mod math;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/menus.rs"] pub mod menus;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/merchants.rs"] pub mod merchants;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/minecarts.rs"] pub mod minecarts;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/mobs.rs"] pub mod mobs;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/mod_state.rs"] pub mod mod_state;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/network.rs"] pub mod network;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/payload_tokens.rs"] pub mod payload_tokens;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/payloads.rs"] pub mod payloads;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/players.rs"] pub mod players;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/projectiles.rs"] pub mod projectiles;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/raids.rs"] pub mod raids;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/recipe_properties.rs"] pub mod recipe_properties;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/recipes.rs"] pub mod recipes;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/resources.rs"] pub mod resources;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/screens.rs"] pub mod screens;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/services.rs"] pub mod services;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/tags.rs"] pub mod tags;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/text.rs"] pub mod text;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/world_edit.rs"] pub mod world_edit;
#[path = "/tmp/svc/.horizon/sdk/api/crates/horizon-mod-vocab/src/host/world_request.rs"] pub mod world_request;
/// Every host trait of every area: bring it into scope to implement the game's side or a test double.
pub mod prelude {
pub use super::advancements::*;
pub use super::block_entities::*;
pub use super::block_states::*;
pub use super::boss_bars::*;
pub use super::client_input::*;
pub use super::client_narrator::*;
pub use super::client_options::*;
pub use super::client_render::*;
pub use super::client_text::*;
pub use super::client_view::*;
pub use super::config::*;
pub use super::data_maps::*;
pub use super::displays::*;
pub use super::draw::*;
pub use super::effects::*;
pub use super::entities::*;
pub use super::fluids::*;
pub use super::fluids_energy::*;
pub use super::item_entities::*;
pub use super::item_particles::*;
pub use super::items::*;
pub use super::keys::*;
pub use super::levels::*;
pub use super::living::*;
pub use super::math::*;
pub use super::menus::*;
pub use super::merchants::*;
pub use super::minecarts::*;
pub use super::mobs::*;
pub use super::mod_state::*;
pub use super::network::*;
pub use super::payload_tokens::*;
pub use super::payloads::*;
pub use super::players::*;
pub use super::projectiles::*;
pub use super::raids::*;
pub use super::recipe_properties::*;
pub use super::recipes::*;
pub use super::resources::*;
pub use super::screens::*;
pub use super::services::*;
pub use super::tags::*;
pub use super::text::*;
pub use super::world_edit::*;
pub use super::world_request::*;
}
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_advancements { ([] $body:item) => { $body }; ([advancements $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_advancements!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_advancements as __skip_advancements;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_block_entities { ([] $body:item) => { $body }; ([block_entities $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_block_entities!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_block_entities as __skip_block_entities;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_block_states { ([] $body:item) => { $body }; ([block_states $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_block_states!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_block_states as __skip_block_states;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_boss_bars { ([] $body:item) => { $body }; ([boss_bars $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_boss_bars!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_boss_bars as __skip_boss_bars;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_client_input { ([] $body:item) => { $body }; ([client_input $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_client_input!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_client_input as __skip_client_input;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_client_narrator { ([] $body:item) => { $body }; ([client_narrator $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_client_narrator!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_client_narrator as __skip_client_narrator;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_client_options { ([] $body:item) => { $body }; ([client_options $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_client_options!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_client_options as __skip_client_options;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_client_render { ([] $body:item) => { $body }; ([client_render $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_client_render!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_client_render as __skip_client_render;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_client_text { ([] $body:item) => { $body }; ([client_text $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_client_text!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_client_text as __skip_client_text;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_client_view { ([] $body:item) => { $body }; ([client_view $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_client_view!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_client_view as __skip_client_view;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_config { ([] $body:item) => { $body }; ([config $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_config!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_config as __skip_config;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_data_maps { ([] $body:item) => { $body }; ([data_maps $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_data_maps!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_data_maps as __skip_data_maps;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_displays { ([] $body:item) => { $body }; ([displays $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_displays!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_displays as __skip_displays;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_draw { ([] $body:item) => { $body }; ([draw $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_draw!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_draw as __skip_draw;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_effects { ([] $body:item) => { $body }; ([effects $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_effects!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_effects as __skip_effects;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_entities { ([] $body:item) => { $body }; ([entities $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_entities!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_entities as __skip_entities;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_fluids { ([] $body:item) => { $body }; ([fluids $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_fluids!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_fluids as __skip_fluids;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_fluids_energy { ([] $body:item) => { $body }; ([fluids_energy $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_fluids_energy!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_fluids_energy as __skip_fluids_energy;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_item_entities { ([] $body:item) => { $body }; ([item_entities $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_item_entities!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_item_entities as __skip_item_entities;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_item_particles { ([] $body:item) => { $body }; ([item_particles $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_item_particles!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_item_particles as __skip_item_particles;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_items { ([] $body:item) => { $body }; ([items $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_items!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_items as __skip_items;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_keys { ([] $body:item) => { $body }; ([keys $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_keys!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_keys as __skip_keys;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_levels { ([] $body:item) => { $body }; ([levels $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_levels!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_levels as __skip_levels;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_living { ([] $body:item) => { $body }; ([living $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_living!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_living as __skip_living;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_math { ([] $body:item) => { $body }; ([math $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_math!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_math as __skip_math;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_menus { ([] $body:item) => { $body }; ([menus $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_menus!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_menus as __skip_menus;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_merchants { ([] $body:item) => { $body }; ([merchants $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_merchants!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_merchants as __skip_merchants;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_minecarts { ([] $body:item) => { $body }; ([minecarts $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_minecarts!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_minecarts as __skip_minecarts;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_mobs { ([] $body:item) => { $body }; ([mobs $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_mobs!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_mobs as __skip_mobs;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_mod_state { ([] $body:item) => { $body }; ([mod_state $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_mod_state!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_mod_state as __skip_mod_state;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_network { ([] $body:item) => { $body }; ([network $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_network!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_network as __skip_network;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_payload_tokens { ([] $body:item) => { $body }; ([payload_tokens $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_payload_tokens!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_payload_tokens as __skip_payload_tokens;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_payloads { ([] $body:item) => { $body }; ([payloads $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_payloads!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_payloads as __skip_payloads;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_players { ([] $body:item) => { $body }; ([players $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_players!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_players as __skip_players;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_projectiles { ([] $body:item) => { $body }; ([projectiles $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_projectiles!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_projectiles as __skip_projectiles;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_raids { ([] $body:item) => { $body }; ([raids $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_raids!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_raids as __skip_raids;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_recipe_properties { ([] $body:item) => { $body }; ([recipe_properties $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_recipe_properties!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_recipe_properties as __skip_recipe_properties;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_recipes { ([] $body:item) => { $body }; ([recipes $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_recipes!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_recipes as __skip_recipes;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_resources { ([] $body:item) => { $body }; ([resources $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_resources!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_resources as __skip_resources;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_screens { ([] $body:item) => { $body }; ([screens $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_screens!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_screens as __skip_screens;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_services { ([] $body:item) => { $body }; ([services $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_services!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_services as __skip_services;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_tags { ([] $body:item) => { $body }; ([tags $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_tags!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_tags as __skip_tags;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_text { ([] $body:item) => { $body }; ([text $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_text!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_text as __skip_text;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_world_edit { ([] $body:item) => { $body }; ([world_edit $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_world_edit!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_world_edit as __skip_world_edit;
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_skip_world_request { ([] $body:item) => { $body }; ([world_request $($rest:ident)*] $body:item) => {}; ([$other:ident $($rest:ident)*] $body:item) => { $crate::__skip_world_request!([$($rest)*] $body); }; }
#[doc(hidden)]
pub use __mod_api_skip_world_request as __skip_world_request;
/// Implements the area traits of `Host` for `$host` with their defaults, except the areas listed in `except(...)`, which the caller implements itself. The union trait is left to the caller.
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_impl_host_areas { ($host:ty $(, except($($skip:ident),*))?) => { $crate::__skip_block_entities!([$($($skip)*)?] impl $crate::prelude::BlockEntitiesHost for $host {});
$crate::__skip_block_states!([$($($skip)*)?] impl $crate::prelude::BlockStatesHost for $host {});
$crate::__skip_client_view!([$($($skip)*)?] impl $crate::prelude::ClientViewHost for $host {});
$crate::__skip_config!([$($($skip)*)?] impl $crate::prelude::ConfigHost for $host {});
$crate::__skip_entities!([$($($skip)*)?] impl $crate::prelude::EntitiesHost for $host {});
$crate::__skip_fluids!([$($($skip)*)?] impl $crate::prelude::FluidsHost for $host {});
$crate::__skip_item_entities!([$($($skip)*)?] impl $crate::prelude::ItemEntitiesHost for $host {});
$crate::__skip_items!([$($($skip)*)?] impl $crate::prelude::ItemsHost for $host {});
$crate::__skip_keys!([$($($skip)*)?] impl $crate::prelude::KeysHost for $host {});
$crate::__skip_levels!([$($($skip)*)?] impl $crate::prelude::LevelsHost for $host {});
$crate::__skip_living!([$($($skip)*)?] impl $crate::prelude::LivingHost for $host {});
$crate::__skip_math!([$($($skip)*)?] impl $crate::prelude::MathHost for $host {});
$crate::__skip_mobs!([$($($skip)*)?] impl $crate::prelude::MobsHost for $host {});
$crate::__skip_mod_state!([$($($skip)*)?] impl $crate::prelude::ModStateHost for $host {});
$crate::__skip_payload_tokens!([$($($skip)*)?] impl $crate::prelude::PayloadTokensHost for $host {});
$crate::__skip_players!([$($($skip)*)?] impl $crate::prelude::PlayersHost for $host {});
$crate::__skip_recipe_properties!([$($($skip)*)?] impl $crate::prelude::RecipePropertiesHost for $host {});
$crate::__skip_recipes!([$($($skip)*)?] impl $crate::prelude::RecipesHost for $host {});
$crate::__skip_resources!([$($($skip)*)?] impl $crate::prelude::ResourcesHost for $host {});
$crate::__skip_tags!([$($($skip)*)?] impl $crate::prelude::TagsHost for $host {});
$crate::__skip_text!([$($($skip)*)?] impl $crate::prelude::TextHost for $host {});
 }; }
#[doc(hidden)]
pub use __mod_api_impl_host_areas as impl_host_areas;
/// Implements the area traits of `ServerHost` for `$host` with their defaults, except the areas listed in `except(...)`, which the caller implements itself. The union trait is left to the caller.
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_impl_server_host_areas { ($host:ty $(, except($($skip:ident),*))?) => { $crate::__skip_advancements!([$($($skip)*)?] impl $crate::prelude::AdvancementsServerHost for $host {});
$crate::__skip_block_entities!([$($($skip)*)?] impl $crate::prelude::BlockEntitiesServerHost for $host {});
$crate::__skip_block_states!([$($($skip)*)?] impl $crate::prelude::BlockStatesServerHost for $host {});
$crate::__skip_boss_bars!([$($($skip)*)?] impl $crate::prelude::BossBarsServerHost for $host {});
$crate::__skip_config!([$($($skip)*)?] impl $crate::prelude::ConfigServerHost for $host {});
$crate::__skip_displays!([$($($skip)*)?] impl $crate::prelude::DisplaysServerHost for $host {});
$crate::__skip_entities!([$($($skip)*)?] impl $crate::prelude::EntitiesServerHost for $host {});
$crate::__skip_items!([$($($skip)*)?] impl $crate::prelude::ItemsServerHost for $host {});
$crate::__skip_menus!([$($($skip)*)?] impl $crate::prelude::MenusServerHost for $host {});
$crate::__skip_merchants!([$($($skip)*)?] impl $crate::prelude::MerchantsServerHost for $host {});
$crate::__skip_minecarts!([$($($skip)*)?] impl $crate::prelude::MinecartsServerHost for $host {});
$crate::__skip_mobs!([$($($skip)*)?] impl $crate::prelude::MobsServerHost for $host {});
$crate::__skip_mod_state!([$($($skip)*)?] impl $crate::prelude::ModStateServerHost for $host {});
$crate::__skip_network!([$($($skip)*)?] impl $crate::prelude::NetworkServerHost for $host {});
$crate::__skip_players!([$($($skip)*)?] impl $crate::prelude::PlayersServerHost for $host {});
$crate::__skip_projectiles!([$($($skip)*)?] impl $crate::prelude::ProjectilesServerHost for $host {});
$crate::__skip_services!([$($($skip)*)?] impl $crate::prelude::ServicesServerHost for $host {});
$crate::__skip_world_edit!([$($($skip)*)?] impl $crate::prelude::WorldEditServerHost for $host {});
 }; }
#[doc(hidden)]
pub use __mod_api_impl_server_host_areas as impl_server_host_areas;
/// Implements the area traits of `ClientHost` for `$host` with their defaults, except the areas listed in `except(...)`, which the caller implements itself. The union trait is left to the caller.
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_impl_client_host_areas { ($host:ty $(, except($($skip:ident),*))?) => { $crate::__skip_client_input!([$($($skip)*)?] impl $crate::prelude::ClientInputClientHost for $host {});
$crate::__skip_client_narrator!([$($($skip)*)?] impl $crate::prelude::ClientNarratorClientHost for $host {});
$crate::__skip_client_options!([$($($skip)*)?] impl $crate::prelude::ClientOptionsClientHost for $host {});
$crate::__skip_client_render!([$($($skip)*)?] impl $crate::prelude::ClientRenderClientHost for $host {});
$crate::__skip_client_text!([$($($skip)*)?] impl $crate::prelude::ClientTextClientHost for $host {});
$crate::__skip_client_view!([$($($skip)*)?] impl $crate::prelude::ClientViewClientHost for $host {});
$crate::__skip_config!([$($($skip)*)?] impl $crate::prelude::ConfigClientHost for $host {});
$crate::__skip_draw!([$($($skip)*)?] impl $crate::prelude::DrawClientHost for $host {});
$crate::__skip_effects!([$($($skip)*)?] impl $crate::prelude::EffectsClientHost for $host {});
$crate::__skip_item_particles!([$($($skip)*)?] impl $crate::prelude::ItemParticlesClientHost for $host {});
$crate::__skip_items!([$($($skip)*)?] impl $crate::prelude::ItemsClientHost for $host {});
$crate::__skip_menus!([$($($skip)*)?] impl $crate::prelude::MenusClientHost for $host {});
$crate::__skip_network!([$($($skip)*)?] impl $crate::prelude::NetworkClientHost for $host {});
$crate::__skip_payloads!([$($($skip)*)?] impl $crate::prelude::PayloadsClientHost for $host {});
$crate::__skip_players!([$($($skip)*)?] impl $crate::prelude::PlayersClientHost for $host {});
$crate::__skip_recipes!([$($($skip)*)?] impl $crate::prelude::RecipesClientHost for $host {});
$crate::__skip_screens!([$($($skip)*)?] impl $crate::prelude::ScreensClientHost for $host {});
$crate::__skip_services!([$($($skip)*)?] impl $crate::prelude::ServicesClientHost for $host {});
$crate::__skip_world_request!([$($($skip)*)?] impl $crate::prelude::WorldRequestClientHost for $host {});
 }; }
#[doc(hidden)]
pub use __mod_api_impl_client_host_areas as impl_client_host_areas;
/// Implements the area traits of `Backend` for `$host` with their defaults, except the areas listed in `except(...)`, which the caller implements itself. The union trait is left to the caller.
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_impl_backend_areas { ($host:ty $(, except($($skip:ident),*))?) => { $crate::__skip_block_entities!([$($($skip)*)?] impl $crate::prelude::BlockEntitiesBackend for $host {});
$crate::__skip_block_states!([$($($skip)*)?] impl $crate::prelude::BlockStatesBackend for $host {});
$crate::__skip_boss_bars!([$($($skip)*)?] impl $crate::prelude::BossBarsBackend for $host {});
$crate::__skip_data_maps!([$($($skip)*)?] impl $crate::prelude::DataMapsBackend for $host {});
$crate::__skip_displays!([$($($skip)*)?] impl $crate::prelude::DisplaysBackend for $host {});
$crate::__skip_effects!([$($($skip)*)?] impl $crate::prelude::EffectsClientBackend for $host {});
$crate::__skip_entities!([$($($skip)*)?] impl $crate::prelude::EntitiesBackend for $host {});
$crate::__skip_fluids_energy!([$($($skip)*)?] impl $crate::prelude::FluidsEnergyBackend for $host {});
$crate::__skip_item_entities!([$($($skip)*)?] impl $crate::prelude::ItemEntitiesBackend for $host {});
$crate::__skip_items!([$($($skip)*)?] impl $crate::prelude::ItemsBackend for $host {});
$crate::__skip_keys!([$($($skip)*)?] impl $crate::prelude::KeysBackend for $host {});
$crate::__skip_levels!([$($($skip)*)?] impl $crate::prelude::LevelsBackend for $host {});
$crate::__skip_living!([$($($skip)*)?] impl $crate::prelude::LivingBackend for $host {});
$crate::__skip_living!([$($($skip)*)?] impl $crate::prelude::LivingClientBackend for $host {});
$crate::__skip_menus!([$($($skip)*)?] impl $crate::prelude::MenusBackend for $host {});
$crate::__skip_menus!([$($($skip)*)?] impl $crate::prelude::MenusClientBackend for $host {});
$crate::__skip_merchants!([$($($skip)*)?] impl $crate::prelude::MerchantsBackend for $host {});
$crate::__skip_minecarts!([$($($skip)*)?] impl $crate::prelude::MinecartsBackend for $host {});
$crate::__skip_mobs!([$($($skip)*)?] impl $crate::prelude::MobsBackend for $host {});
$crate::__skip_players!([$($($skip)*)?] impl $crate::prelude::PlayersBackend for $host {});
$crate::__skip_players!([$($($skip)*)?] impl $crate::prelude::PlayersClientBackend for $host {});
$crate::__skip_projectiles!([$($($skip)*)?] impl $crate::prelude::ProjectilesBackend for $host {});
$crate::__skip_raids!([$($($skip)*)?] impl $crate::prelude::RaidsBackend for $host {});
$crate::__skip_recipes!([$($($skip)*)?] impl $crate::prelude::RecipesBackend for $host {});
 }; }
#[doc(hidden)]
pub use __mod_api_impl_backend_areas as impl_backend_areas;
/// Implements the area traits of `BackendMut` for `$host` with their defaults, except the areas listed in `except(...)`, which the caller implements itself. The union trait is left to the caller.
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_impl_backend_mut_areas { ($host:ty $(, except($($skip:ident),*))?) => { $crate::__skip_block_entities!([$($($skip)*)?] impl $crate::prelude::BlockEntitiesBackendMut for $host {});
$crate::__skip_boss_bars!([$($($skip)*)?] impl $crate::prelude::BossBarsBackendMut for $host {});
$crate::__skip_displays!([$($($skip)*)?] impl $crate::prelude::DisplaysBackendMut for $host {});
$crate::__skip_entities!([$($($skip)*)?] impl $crate::prelude::EntitiesBackendMut for $host {});
$crate::__skip_fluids_energy!([$($($skip)*)?] impl $crate::prelude::FluidsEnergyBackendMut for $host {});
$crate::__skip_item_entities!([$($($skip)*)?] impl $crate::prelude::ItemEntitiesBackendMut for $host {});
$crate::__skip_items!([$($($skip)*)?] impl $crate::prelude::ItemsBackendMut for $host {});
$crate::__skip_levels!([$($($skip)*)?] impl $crate::prelude::LevelsBackendMut for $host {});
$crate::__skip_living!([$($($skip)*)?] impl $crate::prelude::LivingBackendMut for $host {});
$crate::__skip_menus!([$($($skip)*)?] impl $crate::prelude::MenusBackendMut for $host {});
$crate::__skip_merchants!([$($($skip)*)?] impl $crate::prelude::MerchantsBackendMut for $host {});
$crate::__skip_minecarts!([$($($skip)*)?] impl $crate::prelude::MinecartsBackendMut for $host {});
$crate::__skip_mobs!([$($($skip)*)?] impl $crate::prelude::MobsBackendMut for $host {});
$crate::__skip_payloads!([$($($skip)*)?] impl $crate::prelude::PayloadsBackendMut for $host {});
$crate::__skip_players!([$($($skip)*)?] impl $crate::prelude::PlayersBackendMut for $host {});
$crate::__skip_projectiles!([$($($skip)*)?] impl $crate::prelude::ProjectilesBackendMut for $host {});
$crate::__skip_raids!([$($($skip)*)?] impl $crate::prelude::RaidsBackendMut for $host {});
 }; }
#[doc(hidden)]
pub use __mod_api_impl_backend_mut_areas as impl_backend_mut_areas;
/// Implements the area traits of `ClientBackendMut` for `$host` with their defaults, except the areas listed in `except(...)`, which the caller implements itself. The union trait is left to the caller.
#[doc(hidden)]
#[macro_export]
macro_rules! __mod_api_impl_client_backend_mut_areas { ($host:ty $(, except($($skip:ident),*))?) => { $crate::__skip_entities!([$($($skip)*)?] impl $crate::prelude::EntitiesClientBackendMut for $host {});
$crate::__skip_levels!([$($($skip)*)?] impl $crate::prelude::LevelsClientBackendMut for $host {});
$crate::__skip_menus!([$($($skip)*)?] impl $crate::prelude::MenusClientBackendMut for $host {});
$crate::__skip_players!([$($($skip)*)?] impl $crate::prelude::PlayersClientBackendMut for $host {});
 }; }
#[doc(hidden)]
pub use __mod_api_impl_client_backend_mut_areas as impl_client_backend_mut_areas;
