#!/bin/bash

# Compile rary2
rustc --crate-type=lib rary2.rs

# Compile rary
rustc --crate-type=lib rary.rs --extern rary2=library2.rlib

# Compile executable
rustc executable.rs --extern rary2 --extern rary -L . --edition=2018 && ./executable
