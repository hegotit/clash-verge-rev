use crate::log_if_err;
use crate::utils::{config, dirs};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// ### `verge.yaml` schema
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct VergeConfig {
  // i18n
  pub language: Option<String>,

  /// `light` or `dark`
  pub theme_mode: Option<String>,

  /// enable blur mode
  /// maybe be able to set the alpha
  pub theme_blur: Option<bool>,

  /// enable traffic graph default is true
  pub traffic_graph: Option<bool>,

  /// clash tun mode
  pub enable_tun_mode: Option<bool>,

  /// can the app auto startup
  pub enable_auto_launch: Option<bool>,

  /// not show the window on launch
  pub enable_silent_start: Option<bool>,

  /// set system proxy
  pub enable_system_proxy: Option<bool>,

  /// enable proxy guard
  pub enable_proxy_guard: Option<bool>,

  /// set system proxy bypass
  pub system_proxy_bypass: Option<String>,

  /// proxy guard duration
  pub proxy_guard_duration: Option<u64>,

  /// theme setting
  pub theme_setting: Option<VergeTheme>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct VergeTheme {
  pub primary_color: Option<String>,
  pub secondary_color: Option<String>,
  pub primary_text: Option<String>,
  pub secondary_text: Option<String>,

  pub info_color: Option<String>,
  pub error_color: Option<String>,
  pub warning_color: Option<String>,
  pub success_color: Option<String>,

  pub font_family: Option<String>,
  pub css_injection: Option<String>,
}

impl VergeConfig {
  pub fn new() -> Self {
    config::read_yaml::<VergeConfig>(dirs::verge_path())
  }

  /// Save Verge App Config
  pub fn save_file(&self) -> Result<()> {
    config::save_yaml(
      dirs::verge_path(),
      self,
      Some("# The Config for Clash Verge App\n\n"),
    )
  }
}

/// Verge App abilities
#[derive(Debug)]
pub struct Verge {
  /// manage the verge config
  pub config: VergeConfig,
}

impl Default for Verge {
  fn default() -> Self {
    Verge::new()
  }
}

impl Verge {
  pub fn new() -> Self {
    Verge {
      config: VergeConfig::new(),
    }
  }

  /// patch verge config
  /// only save to file
  pub fn patch_config(&mut self, patch: VergeConfig) -> Result<()> {
    // only change it
    if patch.language.is_some() {
      self.config.language = patch.language;
    }
    if patch.theme_mode.is_some() {
      self.config.theme_mode = patch.theme_mode;
    }
    if patch.theme_blur.is_some() {
      self.config.theme_blur = patch.theme_blur;
    }
    if patch.theme_setting.is_some() {
      self.config.theme_setting = patch.theme_setting;
    }
    if patch.traffic_graph.is_some() {
      self.config.traffic_graph = patch.traffic_graph;
    }

    // system setting
    if patch.enable_silent_start.is_some() {
      self.config.enable_silent_start = patch.enable_silent_start;
    }
    if patch.enable_auto_launch.is_some() {
      self.config.enable_auto_launch = patch.enable_auto_launch;
    }

    // proxy
    if patch.enable_system_proxy.is_some() {
      self.config.enable_system_proxy = patch.enable_system_proxy;
    }
    if patch.system_proxy_bypass.is_some() {
      self.config.system_proxy_bypass = patch.system_proxy_bypass;
    }
    if patch.enable_proxy_guard.is_some() {
      self.config.enable_proxy_guard = patch.enable_proxy_guard;
    }
    if patch.proxy_guard_duration.is_some() {
      self.config.proxy_guard_duration = patch.proxy_guard_duration;
    }

    // tun mode
    if patch.enable_tun_mode.is_some() {
      self.config.enable_tun_mode = patch.enable_tun_mode;
    }

    self.config.save_file()
  }
}
