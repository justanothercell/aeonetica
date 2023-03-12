use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use nanoserde::{DeBin, SerBin};
use crate::Id;
use crate::util::type_to_id;

#[deprecated(note="Do not implement this trait directly! Implement aeonetica_client::networking::messaging::ClientHandle instead!")]
pub unsafe trait ClientHandle {}