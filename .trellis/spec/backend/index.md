# Backend Development Guidelines

> Repo-backed conventions for Rust CLI, vault-file workflow, and skill-bundle
> contract development in this project.

---

## Overview

This project is not a server application. In Trellis, "backend" means the
deterministic Rust CLI, vault-file lifecycle logic, repo contract validators,
and runtime skill-bundle support code.

---

## Guidelines Index

| Guide | Description | Status |
|-------|-------------|--------|
| [Directory Structure](./directory-structure.md) | Module organization and file layout | Filled |
| [File Persistence And Manifests](./database-guidelines.md) | Vault files, manifests, JSON/JSONL outputs | Filled |
| [Error Handling](./error-handling.md) | Error types, handling strategies | Filled |
| [Quality Guidelines](./quality-guidelines.md) | Code standards, forbidden patterns | Filled |
| [Logging Guidelines](./logging-guidelines.md) | Structured payloads and audit events | Filled |

---

## Pre-Development Checklist

Before editing Rust CLI, vault workflow, skill contract, fixture, or docs
contract code:

1. Read [Directory Structure](./directory-structure.md).
2. Read [File Persistence And Manifests](./database-guidelines.md) for any
   change that reads or writes vault files, manifests, audit logs, or generated
   outputs.
3. Read [Error Handling](./error-handling.md) for command or validator changes.
4. Read [Logging Guidelines](./logging-guidelines.md) before adding output,
   diagnostics, or audit events.
5. Read [Quality Guidelines](./quality-guidelines.md) before selecting tests or
   deciding whether a contract/doc/fixture update is required.

---

**Language**: All documentation should be written in **English**.
