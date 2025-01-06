# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.60.0] - 2025-01-06

### CHANGED

- update `axum-core` to 0.5.0
- update `hyper` to 1.5
- update `salvo` to 0.75.0
- update `http` to 1.2
- update MSRV to `1.80` due to the requirement from `salvo`

## [0.59.0] - 2024-07-07

### CHANGED

- update `hyper` to 1.0 (use String as body)
- update `salvo` to 0.68.0
- use `rust 2021`
- update MSRV to `1.79`
- use pretty json string

### FIXED

- clippy warnings (use clone_from)

## [0.58.0] - 2023-12-14

### CHANGED

- update `http` to 1.0
- update `axum-core` to 0.4.1
- update MSRV to 1.68

## [0.57.0] - 2023-07-04

### CHANGED

- provide `PartialEq` and `Eq` derives for the `HttpApiProblem` struct
Previously, it was only available in the test profiles

### ADDED

- conversions from std::convert::Inallible

## [0.56.0] - 2022-11-25

### CHANGED

- update `axum-core` to 0.3.0

## [0.55.0] - 2022-09-05

### ADDED

- `axum` support

## [0.54.0] - 2022-08-26

### ADDED

- make `fields` of `ApiError` mutably accessible
- make `additional_fields` of `HttpApiProblem` mutably accessible
- `http::Extensions` on `HttpApiProblem`

## [0.53.0]

### ADDED

- derive `IntoApiError` macro
- OpenApi support

## [0.52.0] - 2022-05-15

### CHANGED

- bump salvo dependency to 0.23
- bump actix-web dependency to 4
- bump actix dependency to 0.13
- Updated MSRV to 1.60

## [0.51.0] - 2021-11-03

### ADDED

- rocket support

### CHANGED

- MSRV 1.46 (warp only)

## [0.50.1] - 2021-04-16

### CHANGED

- improved documentation on deserialization of invalid status codes
## [0.50.0] - 2021-04-13

This release contains multiple breaking changes

### CHANGED

- make `HttpApiProblem` methods more builder like [BREAKING CHANGE]
- `title` is no longer a mandatory field on `HttpApiProblem` [BREAKING CHANGE]
- only accept statuscode as parameter for ApiError [BREAKING CHANGE] to get rid of 500 as a default
- `display_message` now writes into a formatter
- `From<StatusCode> for HttpApiProblem` will set the status only
- `Display` for `HttpApiProblem`
- Fields of `ApiError` are private
- `ApiError` now has an `instance field`

### ADDED

- tide support

## [0.23.0] - 2021-03-31

### ADDED

- salvo support
- Builder for ApiError

## [0.22.0] - 2021-03-28

### Changed

- Updated `actix` to 0.11
- MSRV 1.46 (warp only)
## [0.21.0] - 2021-01-24

### Changed

- Drop `with-` prefix from all features.
  The new set of supported features is: `warp`, `actix-web`, `hyper` and `api-error`.

## [0.20.0] - 2021-01-22

- use warp 0.3

## [0.19.0] - 2021-01-15

## CHANGED
- updated web frameworks (features)
- MSRV: 1.45


## [0.18.0] - 2021-01-15

## CHANGED

- make to_hyper_response accept &self #26
- MSRV: 1.42

## [0.17.0] - 2020-02-21
### Removed
- failure support and "with-failure" feature. This can brake codes but clients simply can use "compat"

## [0.16.0] - 2020-02-05
### Added
- A changelog
- `ApiError` implements `std::error::Error` if `failure` feature is not activated.

### Changed
- `failure` support must be activated via feature `with-failure`
- If `failure` feature is activated, both `HttpApiProblem` and `ApiError` implement `failure::Fail`, otherwise `std::error::Error`
- Feature `with_warp` became `with-warp`
- Feature `with_actix_web` became `with-actix-web`
- Feature `with_hyper` became `with-hyper`
- Updated `README.md`
