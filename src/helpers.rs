use std::{fs, path::Path};

use liquid::ValueView;
use log::error;

use crate::errors::PacdError;

pub fn create_dir_for_path(filepath: &Path) -> Result<(), PacdError> {
    filepath
        .parent()
        .filter(|p| !p.exists())
        .map_or(Ok(()), |parent| {
            fs::create_dir_all(parent).map_err(|e| {
                error!(target: "create_dir_for_path", "Error creating directory {e}");
                PacdError::DestCreationError(parent.display().to_string())
            })
        })
}

pub fn get_id_string(obj: &dyn ValueView, coll_name: &str) -> Result<String, PacdError> {
    obj.as_object()
        .and_then(|o| o.get("id"))
        .ok_or(PacdError::NoIDField(coll_name.to_string()))?
        .as_scalar()
        .map(|s| s.to_kstr().into_string())
        .ok_or(PacdError::IDParseError(coll_name.to_string()))
}

// #[derive(Debug)]
// pub struct CompositeObject<'a> {
//     pub left: &'a Object,
//     pub right: &'a Object,
// }

// impl<'a> ValueView for CompositeObject<'a> {
//     fn as_debug(&self) -> &dyn std::fmt::Debug {
//         self
//     }

//     fn render(&self) -> DisplayCow<'_> {
//         DisplayCow::Owned(Box::new(format!("{:?}", self)))
//     }

//     fn source(&self) -> liquid::model::DisplayCow<'_> {
//         DisplayCow::Owned(Box::new(format!("{:?}", self)))
//     }

//     fn type_name(&self) -> &'static str {
//         "CompositeObject"
//     }

//     fn query_state(&self, state: State) -> bool {
//         self.left.query_state(state) && self.right.query_state(state)
//     }

//     fn to_kstr(&self) -> KStringCow<'_> {
//         KStringCow::from_string(format!("{:?}", self))
//     }

//     fn to_value(&self) -> Value {
//         let tmp = self.right.clone().into_iter();
//         let mut clone = self.left.clone();
//         clone.extend(tmp);
//         Value::Object(clone)
//     }
// }

// impl<'a> ObjectView for CompositeObject<'a> {
//     fn as_value(&self) -> &dyn ValueView {
//         self
//     }

//     fn size(&self) -> i64 {
//         self.left.size() + self.right.size()
//     }

//     fn keys<'k>(&'k self) -> Box<dyn Iterator<Item = liquid::model::KStringCow<'k>> + 'k> {
//         let left_keys = self.left.keys();
//         let right_keys = self.right.keys();

//         let combined = left_keys.chain(right_keys);

//         left_keys
//     }

//     fn values<'k>(&'k self) -> Box<dyn Iterator<Item = &'k dyn ValueView> + 'k> {
//         todo!()
//     }

//     fn iter<'k>(
//         &'k self,
//     ) -> Box<dyn Iterator<Item = (liquid::model::KStringCow<'k>, &'k dyn ValueView)> + 'k> {
//         todo!()
//     }

//     fn contains_key(&self, index: &str) -> bool {
//         todo!()
//     }

//     fn get<'s>(&'s self, index: &str) -> Option<&'s dyn ValueView> {
//         todo!()
//     }
// }
