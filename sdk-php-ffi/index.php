<?php

$ffi = \FFI::cdef(
    \file_get_contents( __DIR__ . '/bridge/php-ffi.h'),
    __DIR__ . "/../target/release/libsdk_php_ffi.dylib"
   
);

// Sum numbers
var_dump($ffi->sum_numbers(10, 30));
var_dump($ffi->sum_numbers(-1, 30));


// Person
