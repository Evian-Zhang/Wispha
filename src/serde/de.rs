use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fmt::{self, Display};
use std::path::PathBuf;

use crate::core::structs::*;

use serde::Deserialize;
use toml;