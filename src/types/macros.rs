macro_rules! gen_primitive_types_io {
    (
        $($t:ty: $tmod:ident);*
    ) => (
            $(
                pub mod $tmod {
                    use std::ptr;
                    use std::mem;
                    pub fn read(mem_ptr: usize) -> $t {
                         unsafe {
                            ptr::read(mem_ptr as *mut $t)
                        }
                    }
                    pub fn write(val: $t, mem_ptr: usize) {
                        unsafe {
                            ptr::write(mem_ptr as *mut $t, val)
                        }
                    }
                    pub fn size(_: usize) -> usize {
                        mem::size_of::<$t>()
                    }
                    pub fn val_size(_: $t) -> usize {
                        size(0)
                    }
                }
            )*
    );
}

macro_rules! gen_compound_types_io {
    (
        $($t:ident, $tmod:ident, $reader:expr, $writer: expr, $size:expr);*
    ) => (
            $(
                pub mod $tmod {
                    use types::*;
                    pub fn read(mem_ptr: usize) -> $t {
                        let read = $reader;
                        read(mem_ptr)
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        let write = $writer;
                        write(val, mem_ptr)
                    }
                    pub fn size(_: usize) -> usize {
                        $size
                    }
                    pub fn val_size(_: &$t) -> usize {
                        size(0)
                    }
                }
            )*
    );
}

macro_rules! gen_variable_types_io {
    (
        $($t:ident, $tmod:ident, $reader:expr, $writer: expr, $size:expr, $val_size:expr);*
    ) => (
            $(
                pub mod $tmod {
                    use types::*;
                    pub fn read(mem_ptr: usize) -> $t {
                        let read = $reader;
                        read(mem_ptr)
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        let write = $writer;
                        write(val, mem_ptr)
                    }
                    pub fn size(mem_ptr: usize) -> usize {
                        let size = $size;
                        size(mem_ptr)
                    }
                    pub fn val_size(val: &$t) -> usize {
                        let size = $val_size;
                        size(val)
                    }
                }
            )*
    );
}

macro_rules! get_from_val {
    (true, $e:ident, $d:ident) => (
        match $d {
            &Value::$e(ref v) => Some(v),
            _ => None
        }
    );
    (false, $e:ident, $d:ident) => (
        match $d {
            &Value::$e(v) => Some(v),
            _ => None
        }
    )
}

macro_rules! get_from_val_fn {
    (true, $e:ident, $t:ty) => (
        pub fn $e(&self) -> Option<&$t> {
            get_from_val!(true, $e, self)
        }
    );
    (false, $e:ident, $t:ty) => (
        pub fn $e(&self) -> Option<$t> {
            get_from_val!(false, $e, self)
        }
    )
}

macro_rules! define_types {
    (
        $(
            [ $( $name:expr ),* ], $id:expr, $t:ty, $e:ident, $r:ident, $io:ident
         );*
    ) => (

        #[derive(Copy, Clone)]
        pub enum TypeId {
            $(
                $e = $id,
            )*
            Map = 0 // No matter which id we pick for 'Map' because w/r planners will ignore it when sub_fields is not 'None'
        }

        $(
            impl ToValue for $t {
                fn value(self) -> Value {
                    Value::$e(self)
                }
            }
        )*

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub enum Value {
            $(
                $e($t),
            )*
            Map(Map),
            Array(Vec<Value>),
            NA,
            Null
        }

        impl Eq for Value {}

        impl Value {
            $(
                get_from_val_fn!($r, $e, $t);
            )*
            pub fn Map(&self) -> Option<&Map> {
                match self {
                    &Value::Map(ref m) => Some(m),
                    _ => None
                }
            }
        }
        pub fn get_type_id (name: String) -> u32 {
           match name.as_ref() {
                $(
                    $($name => $id,)*
                )*
                _ => 0,
           }
        }
        pub fn get_id_type (id: u32) -> &'static str {
           match id {
                $(
                    $id => [$($name),*][0],
                )*
                _ => "N/A",
           }
        }
        pub fn get_size (id: u32, mem_ptr: usize) -> usize {
           match id {
                $(
                    $id => $io::size(mem_ptr),
                )*
                _ => 0,
           }
        }
        pub fn get_val (id:u32, mem_ptr: usize) -> Value {
             match id {
                 $(
                     $id => Value::$e($io::read(mem_ptr)),
                 )*
                 _ => Value::NA,
             }
        }
        pub fn set_val (id:u32, val: &Value, mem_ptr: usize) {
             match id {
                 $(
                     $id => $io::write(get_from_val!($r, $e, val).unwrap() , mem_ptr),
                 )*
                 _ => panic!("Type id not illegal {}", id),
             }
        }
        pub fn get_vsize (id: u32, val: &Value) -> usize {
            match id {
                $(
                    $id => {
                        let val_opt = get_from_val!($r, $e, val);
                        if val_opt.is_none() {
                            panic!("value does not match id");
                        } else {
                            $io::val_size(val_opt.unwrap())
                        }
                    },
                )*
                _ => {panic!("type id does not found");},
           }
        }
    );
}