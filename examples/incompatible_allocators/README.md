# incompatible_allocators

## Summary

This example demonstrates undefined behavior caused by mismatched memory allocators between Rust and C++.

## Scenario

- Rust allocates memory using `Box`
- Ownership is transferred to C++
- C++ deallocates using `operator delete`

## Problem

Different allocators may use different metadata layouts.  
Deallocating with the wrong allocator leads to undefined behavior.

## Run

```bash
./run.sh