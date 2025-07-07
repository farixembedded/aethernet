# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Prior to version
1.0, this project doesn't follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html) and
breaking changes should be expected, but will be called out in this changelog. v1.0 and above will
follow SemVer.

## Unreleased

Initial public release.

**Breaking Changes**
* The types available to Aethernet IPC are now documented and checked at build time. New functions
  that need error handing:
  * TBD
* Tokio spawn helper for RPC and Pubsub servers are now `async`

**Changes**
* Tokio spawn helps now fully initialize before returning.

## [v0.1.0] - 2025-06-12

Unpublished internal release.
