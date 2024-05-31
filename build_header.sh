#!/usr/bin/env bash

echo "use dioxus::prelude::*;

pub fn component() -> Element {
    rsx! {" > src/header.rs

dx translate --file ./assets/header.html >> src/header.rs

echo "
    }
}" >> src/header.rs