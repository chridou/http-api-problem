# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.50.0] - unreleased

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
