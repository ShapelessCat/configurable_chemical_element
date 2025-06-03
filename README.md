# Configurable Chemical Element

## Overview

This document provides an overview of the configurable_chemical_element repository, which implements a Rust library with Python bindings for managing chemical element data with configurable properties. The system uses compile-time code generation to create strongly-typed chemical element representations from JSON data sources.

For detailed information about the workspace architecture and component relationships, see Architecture. For specifics about the core library functionality, see Core Library. For information about the macro-based code generation system, see Macro System.

## Purpose and Scope

The configurable_chemical_element system serves as a high-performance, type-safe library for working with chemical element data in both Rust and Python environments. The library allows consumers to selectively include only the chemical properties they need through Cargo feature flags, making it suitable for applications ranging from educational tools to scientific computing.

## Multi-Language Support

Support Rust and Python (through PyO3).

## Data Source

All chemical element info are in the *periodic_table_info.json*. We can enrich it.
