# named_type

[![crates.io](https://img.shields.io/crates/v/named_type.svg)](https://crates.io/crates/named_type)
[![docs.rs](https://docs.rs/named_type/badge.svg)](https://docs.rs/named_type)
[![CircleCI](https://circleci.com/gh/cjhowe7/named_type.svg?style=svg)](https://circleci.com/gh/cjhowe7/named_type)

This is a small Rust procedural macro that lets you get the name of a type,
whether it is an enum or a struct. By simply deriving the trait on your type,
you can automatically add a function to return the name of the type.
