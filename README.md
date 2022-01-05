# kdmapper-rs
A kdmapper library for Rust

Needed to integrate TheCruZ/kdmapper into one of my projects, so I made a library that will let me integrate kdmapper into my Rust code

One nice thing about Rust is that you can include bytes straight into your binary with the `include_bytes!()` macro, so you can easily have your Rust program map a driver without any file dependencies like so:

```rs
fn kdmapper() {
    unsafe {
        let driver = include_bytes!("../kdmapper/HelloWorld.sys");
        let (_, exit) = super::kdmapper(driver.as_slice(), false, true, false, 0, 0).unwrap();
        assert_eq!(exit, 0);
    }
}
```
Also, there you can use the [include_crypt](https://github.com/not-matthias/include_crypt) crate to encrypt your driver so the raw driver bytes aren't in the executable. Hopefully someone can find this useful
