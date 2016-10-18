use message::Message;
use constant::VALUE_END;
use std::any::Any;
use std::collections::HashSet;
use std::io::Write;
use std::ops::{Deref,DerefMut};

pub enum Action {
    Nothing,
    AddRequiredTags(HashSet<&'static [u8]>),
    BeginGroup{message: Box<Message>},
    PrepareForBytes{bytes_tag: &'static [u8]},
    ConfirmPreviousTag{previous_tag: &'static [u8]}, //TODO: Probably redundant to the PrepareForBytes definition. Should be automatically inferred.
}

pub trait FieldType {
    fn new() -> Self;

    fn action() -> Option<Action> {
        None
    }

    fn set_value(&mut self,_bytes: &[u8]) -> bool {
        false
    }

    fn set_groups(&mut self,_groups: &[Box<Message>]) -> bool {
        false
    }

    fn read(&self,buf: &mut Vec<u8>) -> usize;
}

#[derive(Clone,Default,PartialEq)]
pub struct NoneFieldType {
}

impl NoneFieldType {
    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn len(&self) -> usize {
        0
    }
}

impl FieldType for NoneFieldType {
    fn new() -> Self {
        NoneFieldType {}
    }

    fn read(&self,_buf: &mut Vec<u8>) -> usize {
        0
    }
}

#[derive(Clone,Default,PartialEq)]
pub struct StringFieldType {
    value: String,
}

impl FieldType for StringFieldType {
    fn new() -> Self {
        StringFieldType {
            value: String::new(),
        }
    }

    fn set_value(&mut self,bytes: &[u8]) -> bool {
        self.value = String::from_utf8_lossy(bytes).into_owned();
        true
    }

    fn read(&self,buf: &mut Vec<u8>) -> usize {
        buf.write(self.value.as_bytes()).unwrap()
    }
}

impl Deref for StringFieldType {
    type Target = String;

    fn deref(&self) -> &String {
        &self.value
    }
}

impl DerefMut for StringFieldType {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

#[derive(Clone,Default,PartialEq)]
pub struct DataFieldType {
    value: Vec<u8>,
}

impl FieldType for DataFieldType {
    fn new() -> Self {
        DataFieldType {
            value: Vec::new(),
        }
    }

    fn set_value(&mut self,bytes: &[u8]) -> bool {
        self.value.resize(bytes.len(),0);
        self.value.copy_from_slice(bytes);
        true
    }

    fn read(&self,buf: &mut Vec<u8>) -> usize {
        buf.write(&self.value).unwrap()
    }
}

impl Deref for DataFieldType {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.value
    }
}

#[derive(Clone,Default,PartialEq)]
pub struct RepeatingGroupFieldType<T: Message + PartialEq> {
    groups: Vec<Box<T>>,
}

impl<T: Message + Any + Clone + Default + PartialEq> FieldType for RepeatingGroupFieldType<T> {
    fn new() -> Self {
        RepeatingGroupFieldType {
            groups: Vec::new(),
        }
    }

    fn action() -> Option<Action> {
        Some(Action::BeginGroup{ message: Box::new(<T as Default>::default()) })
    }

    fn set_groups(&mut self,groups: &[Box<Message>]) -> bool {
        self.groups.clear();

        for group in groups {
            match group.as_any().downcast_ref::<T>() {
                //TODO: Avoid the clone below.
                Some(casted_group) => self.groups.push(Box::new(casted_group.clone())),
                None => return false,
            }
        }

        true
    }

    fn read(&self,buf: &mut Vec<u8>) -> usize {
        let group_count_str = self.groups.len().to_string();
        let mut result = 1;

        result += buf.write(group_count_str.as_bytes()).unwrap();
        buf.push(VALUE_END);

        for group in &self.groups {
            result += group.read_body(buf);
        }

        result
    }
}

impl<T: Message + PartialEq> Deref for RepeatingGroupFieldType<T> {
    type Target = Vec<Box<T>>;

    fn deref(&self) -> &Vec<Box<T>> {
        &self.groups
    }
}

pub trait Field {
    type Type;
    fn action() -> Action;
    fn tag() -> &'static [u8];
    fn read(buf: &mut Vec<u8>,field: &Self::Type) -> usize;
}

#[macro_export]
macro_rules! define_field {
    ( $( $field_name:ident : $field_type:ty = $tag:expr $( => $action:expr )* ),* $(),* ) => { $(
        pub struct $field_name;
        impl Field for $field_name {
            type Type = $field_type;

            #[allow(unreachable_code)]
            fn action() -> Action {
                //If an action is provided, prefer it first.
                $(
                    return $action;
                )*

                //Next, check if the field type provides an action. This way the BeginGroup action
                //can be specified automatically instead of using a nasty boilerplate in each field
                //definition.
                if let Some(action) = <$field_type as FieldType>::action() {
                    action
                }
                //Otherwise, no action was specified.
                else {
                    Action::Nothing
                }
            }

            fn tag() -> &'static [u8] {
                $tag
            }

            fn read(buf: &mut Vec<u8>,field: &Self::Type) -> usize {
                if field.is_empty() {
                    return 0;
                }

                let mut result = 1;

                //If this is part of a Action::PrepareForBytes and Action::ConfirmPreviousTag pair,
                //insert the length tag first.
                if let Action::ConfirmPreviousTag{ previous_tag } = <$field_name as Field>::action() {
                    result += 2;
                    result += buf.write(previous_tag).unwrap();
                    buf.push(TAG_END);
                    result += buf.write(field.len().to_string().as_bytes()).unwrap();
                    buf.push(VALUE_END);
                }

                //Write tag and value.
                result += buf.write($tag).unwrap();
                buf.push(TAG_END);
                result += field.read(buf);

                //Avoid the VALUE_END symbol iff this is not a repeating group field. This is a
                //hack, under the assumption that the field itself adds this symbol, so the field
                //can append the remaining groups.
                if let Action::BeginGroup{ .. } = <$field_name as Field>::action() {}
                else {
                    result += 1;
                    buf.push(VALUE_END);
                }

                result
            }
        }
    )*};
}
