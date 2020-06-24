use crate::error::invalid_value_variant;
use crate::model::shapes::{Member, Valued};
use crate::model::values::{Key, NodeValue};
use crate::model::{Identifier, ShapeID};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the "service" shape.
///
#[derive(Clone, Debug)]
pub struct Service {
    version: Member,    // **required** Value::String
    operations: Member, // Value::Array(Value::ShapeID)
    resources: Member,  // Value::Array(Value::ShapeID)
}

///
/// Corresponds to the "operation" shape.
///
#[derive(Clone, Debug)]
pub struct Operation {
    input: Member,  // Value::ShapeID
    output: Member, // Value::ShapeID
    errors: Member, // Value::Array(Value::ShapeID)
}

///
/// Corresponds to the "resource" shape.
///
#[derive(Clone, Debug)]
pub struct Resource {
    identifiers: Member,           // Value::Object(Identifier, Value::ShapeID)
    create: Member,                // Value::ShapeID
    put: Member,                   // Value::ShapeID
    read: Member,                  // Value::ShapeID
    update: Member,                // Value::ShapeID
    delete: Member,                // Value::ShapeID
    list: Member,                  // Value::ShapeID
    operations: Member,            // Value::List(Value::ShapeID)
    collection_operations: Member, // Value::List(Value::ShapeID)
    resources: Member,             // Value::List(Value::ShapeID)
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! optional_member {
    ($has:ident, $member:ident, $setter:ident, $unsetter:ident) => {
        /// Returns `true` if this shape has a value for this member, else `false`.
        pub fn $has(&self) -> bool {
            self.$member.value().is_some()
        }

        /// Return the current value of this member.
        pub fn $member(&self) -> Option<&ShapeID> {
            match &self.$member.value().as_ref() {
                None => None,
                Some(NodeValue::ShapeID(id)) => Some(id),
                _ => invalid_value_variant("ShapeID"),
            }
        }

        /// Set the current value of this member.
        pub fn $setter(&mut self, $member: ShapeID) {
            self.$member.set_value(NodeValue::ShapeID($member))
        }

        /// Set the current value of this member to `None`.
        pub fn $unsetter(&mut self) {
            self.$member.unset_value();
        }
    };
}

#[doc(hidden)]
macro_rules! array_member {
    ($collection:ident, $member:ident, $has_fn:ident, $add_fn:ident, $append_fn:ident, $remove_fn:ident) => {
        /// Returns `true` if this member's collection has _any_ elements, else `false`.
        pub fn $has_fn(&self) -> bool {
            match self.$collection.value() {
                Some(v) => match v {
                    NodeValue::Array(vs) => !vs.is_empty(),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Return an iterator over all elements in this member's collection.
        pub fn $collection(&self) -> impl Iterator<Item = &ShapeID> {
            match self.$collection.value() {
                Some(v) => match v {
                    NodeValue::Array(vs) => vs.iter().map(|v| {
                        if let NodeValue::ShapeID(id) = v {
                            id
                        } else {
                            invalid_value_variant("ShapeID")
                        }
                    }),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Add an element to this member's collection.
        pub fn $add_fn(&mut self, $member: ShapeID) {
            match self.$collection.value_mut() {
                Some(v) => match v {
                    NodeValue::Array(vs) => vs.push(NodeValue::ShapeID($member)),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Add all these elements to this member's collection.
        pub fn $append_fn(&mut self, $collection: &[ShapeID]) {
            match self.$collection.value_mut() {
                Some(v) => match v {
                    NodeValue::Array(vs) => vs.append(
                        &mut $collection
                            .iter()
                            .cloned()
                            .map(NodeValue::ShapeID)
                            .collect(),
                    ),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Remove an element, with the given identifier, to this member's collection.
        pub fn $remove_fn(&mut self, $member: &ShapeID) {
            match self.$collection.value_mut() {
                Some(v) => match v {
                    NodeValue::Array(vs) => {
                        let id_value = NodeValue::ShapeID($member.clone());
                        vs.retain(|v| v == &id_value);
                    }
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Service {
    fn default() -> Self {
        Self {
            version: Member::new(Identifier::from_str("version").unwrap()),
            operations: Member::with_value(
                Identifier::from_str("operations").unwrap(),
                NodeValue::Array(Default::default()),
            ),
            resources: Member::with_value(
                Identifier::from_str("resources").unwrap(),
                NodeValue::Array(Default::default()),
            ),
        }
    }
}

impl Service {
    pub fn version(&self) -> &String {
        match &self.version.value().as_ref().unwrap() {
            NodeValue::String(v) => v,
            _ => invalid_value_variant("String"),
        }
    }
    pub fn set_version(&mut self, version: &str) {
        self.version
            .set_value(NodeValue::String(version.to_string()))
    }

    array_member! { operations, operation, has_operations, add_operation, append_operations, remove_operation }

    array_member! { resources, resource, has_resources, add_resource, append_resources, remove_resource }
}

// ------------------------------------------------------------------------------------------------

impl Default for Operation {
    fn default() -> Self {
        Self {
            input: Member::new(Identifier::from_str("input").unwrap()),
            output: Member::new(Identifier::from_str("output").unwrap()),
            errors: Member::with_value(
                Identifier::from_str("errors").unwrap(),
                NodeValue::Array(Default::default()),
            ),
        }
    }
}

impl Operation {
    optional_member! { has_input, input, set_input, unset_input }

    optional_member! { has_output, output, set_output, unset_output }

    array_member! { errors, error, has_errors, add_error, append_errors, remove_error }
}

// ------------------------------------------------------------------------------------------------

impl Default for Resource {
    fn default() -> Self {
        Self {
            identifiers: Member::with_value(
                Identifier::from_str("identifiers").unwrap(),
                NodeValue::Object(Default::default()),
            ),
            create: Member::new(Identifier::from_str("create").unwrap()),
            put: Member::new(Identifier::from_str("put").unwrap()),
            read: Member::new(Identifier::from_str("read").unwrap()),
            update: Member::new(Identifier::from_str("update").unwrap()),
            delete: Member::new(Identifier::from_str("delete").unwrap()),
            list: Member::new(Identifier::from_str("list").unwrap()),
            operations: Member::with_value(
                Identifier::from_str("operations").unwrap(),
                NodeValue::Array(Default::default()),
            ),
            collection_operations: Member::with_value(
                Identifier::from_str("collection_operations").unwrap(),
                NodeValue::Array(Default::default()),
            ),
            resources: Member::with_value(
                Identifier::from_str("resources").unwrap(),
                NodeValue::Array(Default::default()),
            ),
        }
    }
}

impl Resource {
    pub fn has_identifiers(&self) -> bool {
        match self.identifiers.value() {
            Some(v) => match v {
                NodeValue::Object(vs) => !vs.is_empty(),
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    pub fn identifiers(&self) -> impl Iterator<Item = (&Identifier, &ShapeID)> {
        match self.identifiers.value() {
            Some(v) => match v {
                NodeValue::Object(vs) => vs
                    .iter()
                    .map(|(k, v)| (k.as_identifier().unwrap(), v.as_reference().unwrap())),
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    pub fn add_identifier(&mut self, id: Identifier, shape: ShapeID) {
        match self.identifiers.value_mut() {
            Some(v) => match v {
                NodeValue::Object(vs) => {
                    let _ = vs.insert(id.into(), shape.into());
                }
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    pub fn remove_identifier(&mut self, id: &Identifier) {
        match self.identifiers.value_mut() {
            Some(v) => match v {
                NodeValue::Object(vs) => {
                    let key: Key = id.clone().into();
                    vs.retain(|k, _| k == &key);
                }
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    optional_member! { has_create, create, set_create, unset_create }

    optional_member! { has_put, put, set_put, unset_put }

    optional_member! { has_read, read, set_read, unset_read }

    optional_member! { has_update, update, set_update, unset_update }

    optional_member! { has_delete, delete, set_delete, unset_delete }

    optional_member! { has_list, list, set_list, unset_list }

    array_member! { operations, operation, has_operations, add_operation, append_operations, remove_operation }

    array_member! { collection_operations, collection_operation, has_collection_operations, add_collection_operation, append_collection_operations, remove_collection_operation }

    array_member! { resources, resource, has_resources, add_resource, append_resources, remove_resource }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
