src/
├── main.rs
├── features/
│ ├── users/
│ │ ├── mod.rs
│ │ ├── routes.rs # Axum handlers
│ │ ├── service.rs # forretningslogikk
│ │ ├── repository.rs # trait + DB-impl
│ │ └── models.rs # domenetyper
│ └── notifications/
│ ├── mod.rs
│ ├── routes.rs
│ ├── service.rs
│ └── models.rs
└── adapters/
├── auth0.rs # delt på tvers av features
└── sms.rs # delt på tvers av features

## TODO

- setup config repo to load config
- static data to provided non imported and non injected vars
- add runtime var, so if local you use dryrun in the sms stuff so you do not really send
- traits under adapters file, use these in the app state
- use repos inside services and then inejct into state?

### TODO - IDP

- rate limiting with tower governor
- lock recovery med otp?

Actions

- audit and clippy and fmt and test in actions

# hops
