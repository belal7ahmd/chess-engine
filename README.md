# Chess Engine & GUI

This is a personal project to build a Chess Engine in Rust with a web-based GUI for debugging.

## ⚠️ SECURITY WARNING (Read Before Using)

**The GUI included in this repository is strictly for local testing and prototyping purposes.**

* **NOT SECURE:** The Node.js server (`server.js`) has not been audited for security. It may contain vulnerabilities such as Command Injection or Path Traversal.
* **LOCAL USE ONLY:** Do NOT host this GUI on a public server or expose it to the internet. It is intended to be run only on your own machine (`localhost`).
* **NO INPUT SANITIZATION:** The engine and GUI assume that all inputs come from a trusted user (you).

**Use at your own risk.**

---

## Structure
* `engine/` - The Chess Engine (written in Rust).
* `gui/` - Web interface (Node.js + HTML) for visualizing the search.
* `prototyping/` - Python scripts used for testing algorithms before implementation.