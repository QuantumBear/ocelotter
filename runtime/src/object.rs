use std::fmt;

use crate::JvmValue;
use crate::OtField;
use crate::CONTEXT;

#[derive(Clone, Debug, Trace, Finalize)]
pub enum OtObj {
    vm_obj {
        id: usize,
        mark: u64,
        klassid: usize,
        fields: Vec<JvmValue>,
    },
    vm_arr_int {
        id: usize,
        mark: u64,
        klassid: usize,
        length: i32,
        elements: Vec<i32>,
    },
    vm_arr_long {
        id: usize,
        mark: u64,
        klassid: usize,
        length: i32,
        elements: Vec<i64>,
    },
}

impl OtObj {
    pub fn obj_of(klass_id: usize, obj_id: usize, fields: Vec<JvmValue>) -> OtObj {
        OtObj::vm_obj {
            id: obj_id,
            mark: 0u64,
            klassid: klass_id,
            fields: fields,
        }
    }

    pub fn int_arr_of(size: i32, obj_id: usize) -> OtObj {
        let sz = size as usize;
        let mut elts = Vec::with_capacity(sz);
        elts.resize(sz, 0);
        OtObj::vm_arr_int {
            id: obj_id,
            mark: 0u64,
            klassid: 2, // FIXME Need Object in the mix soon...
            length: size,
            elements: elts,
        }
    }

    pub fn put_field(&mut self, f: OtField, val: JvmValue) -> () {
        let (kid, fields) = match self {
            OtObj::vm_obj {
                id: _,
                mark: _,
                klassid: id,
                fields: fs,
            } => (id, fs),
            _ => panic!("Not an object"),
        };
        // Get klass
        let mut klass = match CONTEXT
            .lock()
            .unwrap()
            .get_repo()
            .id_lookup.get(&kid) {
            Some(k) => k.clone(),
            None => panic!("No klass with ID {} found in repo", kid),
        };
        // Lookup offset in klass
        let offset = klass.get_field_offset(&f);
        fields[offset] = val.clone();
    }

    // NOTE: Returns a by-value copy of what's held, put should unconditionally overwrite
    pub fn get_value(&self, f: OtField) -> JvmValue {
        let (kid, fields) = match self {
            OtObj::vm_obj {
                id: _,
                mark: _,
                klassid: id,
                fields: fs,
            } => (id, fs),
            _ => panic!("Not an object"),
        };
        // Get klass
        let mut klass = match CONTEXT
            .lock()
            .unwrap()
            .get_repo()
            .id_lookup.get(&kid) {
            Some(k) => k.clone(),
            None => panic!("No klass with ID {} found in repo", kid),
        };
        // Lookup offset in klass
        let offset = klass.get_field_offset(&f);
        match fields.get(offset) {
            Some(v) => v.clone(),
            None => panic!("Fields should hold a value"),
        }
    }


    pub fn get_null() -> OtObj {
        OtObj::vm_obj {
            id: 0,
            mark: 0u64,
            klassid: 0, // klassid of 0 implies null
            fields: Vec::new(),
        }
    }

    pub fn is_null(&self) -> bool {
        if self.get_mark() == 0u64 && self.get_klassid() == 0 {
            true
        } else {
            false
        }
    }

    pub fn get_id(&self) -> usize {
        match *self {
            OtObj::vm_obj {
                id: i,
                mark: _,
                klassid: _,
                fields: _,
            } => i,
            OtObj::vm_arr_int {
                id: i,
                mark: _,
                klassid: _,
                length: _,
                elements: _,
            } => i,
            OtObj::vm_arr_long {
                id: i,
                mark: _,
                klassid: _,
                length: _,
                elements: _,
            } => i,
        }
    }

    pub fn get_mark(&self) -> u64 {
        match *self {
            OtObj::vm_obj {
                id: _,
                mark: m,
                klassid: _,
                fields: _,
            } => m,
            OtObj::vm_arr_int {
                id: _,
                mark: m,
                klassid: _,
                length: _,
                elements: _,
            } => m,
            OtObj::vm_arr_long {
                id: _,
                mark: m,
                klassid: _,
                length: _,
                elements: _,
            } => m,
        }
    }

    pub fn get_klassid(&self) -> usize {
        match *self {
            OtObj::vm_obj {
                id: _,
                mark: _,
                klassid: k,
                fields: _,
            } => k,
            OtObj::vm_arr_int {
                id: _,
                mark: _,
                klassid: k,
                length: _,
                elements: _,
            } => k,
            OtObj::vm_arr_long {
                id: _,
                mark: _,
                klassid: k,
                length: _,
                elements: _,
            } => k,
        }
    }

    pub fn length(&self) -> i32 {
        match *self {
            OtObj::vm_obj {
                id: _,
                mark: _,
                klassid: _,
                fields: _,
            } => panic!("Attempted to take the length of a normal object!"),
            OtObj::vm_arr_int {
                id: _,
                mark: _,
                klassid: _,
                length: l,
                elements: _,
            } => l,
            OtObj::vm_arr_long {
                id: _,
                mark: _,
                klassid: _,
                length: l,
                elements: _,
            } => l,
        }
    }
}

impl fmt::Display for OtObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MarK: {} ; Klass: {}",
            self.get_mark(),
            self.get_klassid()
        )
    }
}
