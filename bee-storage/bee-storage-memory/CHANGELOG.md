# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## Unreleased - YYYY-MM-DD

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security -->

## 0.3.0 - 2022-03-17

### Added

- Implementation of `InsertStrict<MessageId, MessageMetadata>` for `Storage`;

### Removed

- Implementation of `Insert<MessageId, MessageMetadata>` for `Storage`;

## 0.2.0 - 2022-03-11

### Added

- Implementation of `Update` for `Storage`;

## 0.1.0 - 2021-10-21

### Added

- Implementation of `StorageBackend` using in-memory data structures;
- Implementation of the following traits for all bee storable types:
  - `Batch`;
  - `Delete`;
  - `Exist`;
  - `Fetch`;
  - `Insert`;
  - `MultiFetch`;
  - `AsIterator`;
  - `Truncate`;
