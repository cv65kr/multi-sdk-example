Run:
```
cargo build
```

Run tests:

```bash
php -d extension=../target/debug/libsdk_php.dylib vendor/bin/phpunit
```

Stubs
```bash
cargo php stubs --stdout
```
and put into IDE