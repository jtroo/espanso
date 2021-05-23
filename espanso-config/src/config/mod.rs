/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::Result;
use std::{collections::HashSet, path::Path};
use thiserror::Error;

mod parse;
mod path;
mod resolve;
mod util;
pub(crate) mod default;
pub(crate) mod store;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait Config: Send {
  fn id(&self) -> i32;
  fn label(&self) -> &str;
  fn match_paths(&self) -> &[String];
  fn backend(&self) -> Backend;

  // Number of chars after which a match is injected with the clipboard
  // backend instead of the default one. This is done for efficiency
  // reasons, as injecting a long match through separate events becomes 
  // slow for long strings.
  fn clipboard_threshold(&self) -> usize;
  
  // Delay (in ms) that espanso should wait to trigger the paste shortcut
  // after copying the content in the clipboard. This is needed because
  // if we trigger a "paste" shortcut before the content is actually
  // copied in the clipboard, the operation will fail.
  fn pre_paste_delay(&self) -> usize;

  // TODO: add other delay options (start by the ones needed in clipboard injector)

  // Defines the key that disables/enables espanso when double pressed
  fn toggle_key(&self) -> Option<ToggleKey>;

  fn is_match<'a>(&self, app: &AppProperties<'a>) -> bool;
}

pub trait ConfigStore: Send {
  fn default(&self) -> &dyn Config;
  fn active<'a>(&'a self, app: &AppProperties) -> &'a dyn Config;
  fn configs(&self) -> Vec<&dyn Config>;

  fn get_all_match_paths(&self) -> HashSet<String>;
}

pub struct AppProperties<'a> {
  pub title: Option<&'a str>,
  pub class: Option<&'a str>,
  pub exec: Option<&'a str>,
}

#[derive(Debug, Copy, Clone)]
pub enum Backend {
  Inject,
  Clipboard,
  Auto,
}


#[derive(Debug, Copy, Clone)]
pub enum ToggleKey {
  Ctrl,
  Meta,
  Alt,
  Shift,
  RightCtrl,
  RightAlt,
  RightShift,
  RightMeta,
  LeftCtrl,
  LeftAlt,
  LeftShift,
  LeftMeta,
}

pub fn load_store(config_dir: &Path) -> Result<impl ConfigStore> {
  store::DefaultConfigStore::load(config_dir)
}

#[derive(Error, Debug)]
pub enum ConfigStoreError {
  #[error("invalid config directory")]
  InvalidConfigDir(),

  #[error("missing default.yml config")]
  MissingDefault(),

  #[error("io error")]
  IOError(#[from] std::io::Error),
}
