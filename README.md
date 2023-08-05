# Temporal

Work with date-times using a called temporal API. It is based on a TC39 proposal. Cross-platform and designed for the Rust language.

> **NOTE:** Currently unavailable and under development.

## Roadmap

<details>

<summary>Roadmap list</summary>

- [ ] Optional browser support by passing a `browser_support` feature.
- References
  - https://tc39.es/proposal-temporal/docs
  - https://github.com/tc39/proposal-temporal
- [ ] Create a crate for gathering data from IANA timezone database.
- [ ] Start and finish the ambiguity documentation in docs/ambiguity.rs
  - https://tc39.es/proposal-temporal/docs/ambiguity.html
- [ ] There are a few documentation pages in addition to ambiguity too; just iterate the TC39 Temporal API docs. Add all the home sections too to summarize the API and the _Other documentation_ section.
- [ ] Fully document the public API according to the TC39 proposal
- [ ] `temporal::now`
  - [ ] `instant`
  - [ ] `timezone_id`
  - [ ] `zoned_date_time`
  - [ ] `zoned_date_time_iso`
  - [ ] `plain_date`
  - [ ] `plain_date_iso`
  - [ ] `plain_time_iso`
  - [ ] `plain_date_time`
  - [ ] `plain_date_time_iso`
- [ ] Types with addition and subtraction also implement `(Add|Sub)Assign` (`+=` and `-=`)
- [ ] `temporal::Instant`
- [ ] `temporal::ZonedDateTime`
  - Can be constructed with an options object. Implement `Default` for it.
- [ ] `temporal::PlainDate`
- [ ] `temporal::PlainTime`
- [ ] `temporal::PlainDateTime`
- [ ] `temporal::PlainYearMonth`
- [ ] `temporal::PlainMonthDay`
- [ ] `temporal::Duration`
  - Constructed via methods such as `Duration::from_milliseconds()` and things can be accessed like `years()`.
- [ ] `temporal::TimeZone`
  - Consider implementing the `Display` trait
- [ ] `temporal::Calendar`
- [ ] Implement `Display` for every type

</details>