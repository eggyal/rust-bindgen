/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]



/// Because this struct have array larger than 32 items
/// and --with-derive-partialeq --impl-partialeq --impl-debug is provided,
/// this struct should manually implement `Debug` and `PartialEq`.
#[repr(C)]
#[derive(Copy)]
pub struct Foo {
    pub large: [::std::os::raw::c_int; 33usize],
    pub _bitfield_1: [u8; 2usize],
    pub __bindgen_padding_0: u16,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        136usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    assert_eq!(
        unsafe { &(*(0 as *const Foo)).large as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(Foo),
            "::",
            stringify!(large)
        )
    );
}
extern "C" {
    #[link_name = "_ZN3Foo4typeEv"]
    pub fn Foo_type(this: *mut Foo) -> ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "_ZN3Foo9set_type_Ec"]
    pub fn Foo_set_type_(this: *mut Foo, c: ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "_ZN3Foo8set_typeEc"]
    pub fn Foo_set_type(this: *mut Foo, c: ::std::os::raw::c_char);
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Foo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "Foo {{ large: [{}], type_ : {:?},  }}",
            self.large
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{}{:?}", if i > 0 { ", " } else { "" }, v))
                .collect::<String>(),
            self.type__bindgen_bitfield()
        )
    }
}
impl ::std::cmp::PartialEq for Foo {
    fn eq(&self, other: &Foo) -> bool {
        &self.large[..] == &other.large[..]
            && self.type__bindgen_bitfield() == other.type__bindgen_bitfield()
    }
}
impl Foo {
    #[inline]
    pub fn type__bindgen_bitfield(&self) -> ::std::os::raw::c_char {
        let mut unit_field_val: u16 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u16 as *mut u8,
                ::std::mem::size_of::<u16>(),
            )
        };
        let mask = 7u64 as u16;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_type__bindgen_bitfield(&mut self, val: ::std::os::raw::c_char) {
        let mask = 7u64 as u16;
        let val = val as u8 as u16;
        let mut unit_field_val: u16 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u16 as *mut u8,
                ::std::mem::size_of::<u16>(),
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                ::std::mem::size_of::<u16>(),
            );
        }
    }
    #[inline]
    pub fn new_bitfield_1(type__bindgen_bitfield: ::std::os::raw::c_char) -> u16 {
        (0 | ((type__bindgen_bitfield as u8 as u16) << 0usize) & (7u64 as u16))
    }
    #[inline]
    pub unsafe fn type_(&mut self) -> ::std::os::raw::c_char {
        Foo_type(self)
    }
    #[inline]
    pub unsafe fn set_type_(&mut self, c: ::std::os::raw::c_char) {
        Foo_set_type_(self, c)
    }
    #[inline]
    pub unsafe fn set_type(&mut self, c: ::std::os::raw::c_char) {
        Foo_set_type(self, c)
    }
}
