use ext_php_rs::{
    builders::ModuleBuilder,
    info_table_end,
    info_table_row,
    info_table_start,
    prelude::*,
    zend::ModuleEntry,
};

mod calculator;
use crate::calculator::_internal_php_sum_numbers;

mod person;
use crate::person::Person;

pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("multi-sdk", "enabled");
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.info_function(php_module_info)
}