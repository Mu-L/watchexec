# Changelog

## Next (YYYY-MM-DD)

## v7.0.0 (2025-05-15)

- Deps: remove unused dependency `watchexec-signals` ([#930](https://github.com/watchexec/watchexec/pull/930))

## v6.0.0 (2025-02-09)

## v5.0.0 (2024-10-14)

## v4.0.1 (2024-04-28)

## v4.0.0 (2024-04-20)

- Deps: watchexec 4

## v3.0.1 (2024-01-04)

- Normalise paths on all platforms (via `normalize-path`).

## v3.0.0 (2024-01-01)

- Deps: `ignore-files` 2.0.0

## v2.0.1 (2023-12-09)

- Depend on `watchexec-events` instead of the `watchexec` re-export.

## v1.2.1 (2023-05-14)

- Use IO-free dunce::simplify to normalise paths on Windows.
- Known regression: some filtering patterns misbehave slightly on Windows with paths outside the project root.
  - As filters were previously completely broken on Windows, this is still considered an improvement.

## v1.2.0 (2023-03-18)

- Ditch MSRV policy. The `rust-version` indication will remain, for the minimum estimated Rust version for the code features used in the crate's own code, but dependencies may have already moved on. From now on, only latest stable is assumed and tested for. ([#510](https://github.com/watchexec/watchexec/pull/510))

## v1.1.0 (2023-01-09)

- MSRV: bump to 1.61.0

## v1.0.0 (2022-06-23)

- Initial release as a separate crate.
