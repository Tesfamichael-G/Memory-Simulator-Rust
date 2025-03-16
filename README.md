# Rust DDR Memory Simulator

This repository contains two Rust projects designed to simulate DDR (Double Data Rate) memory:

## Projects

### 1. **`mem_rus` (Main Project)**
The `mem_rus` project implements the primary simulation logic for DDR memory. It models the behavior and timing characteristics of DDR memory systems, providing a detailed and accurate simulation environment.

### 2. **`memory_interface` (Library Submodule)**
The `memory_interface` library defines common interfaces used by the `mem_rus` project. These interfaces include abstractions for memory operations, timing parameters, and other components critical to the simulation.

## Features
- Comprehensive simulation of DDR memory systems.
- Modular design: the `memory_interface` library ensures reusability and clarity.
- Extensible for additional memory models or interfaces.
