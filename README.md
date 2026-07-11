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

### Configuration

- [ ] Setup config repo to load config
- [ ] Static data to provide non-imported and non-injected vars
- [ ] Add runtime var for dryrun SMS in local env
- [ ] Read OTP TTL from config
- [ ] Read OTP max tries from config
- [ ] Read OTP max messages per 24h from config
- [ ] Read access token expiry from config
- [ ] Read audit log eviction TTL from config

### Auth / Security

- [ ] Rate limiting with tower-governor
- [ ] Lock recovery with OTP
- [ ] OTP max attempts enforcement
- [ ] OTP rate limiting (max per 24h)

### Decisions Needed

- [ ] Passwordless login requires given_name/family_name to be optional

### Schema

- [ ] Drop email from user, make phone_number non-optional

### Proposals

- [ ] add test runtime to use a cache rather than sending actual messages
- [ ] add last_login_at for tracing user logins without refresh token for metrics
- [ ] should trace audit logs for failed logins

### CI/CD

- [ ] Audit in actions
- [ ] Clippy in actions
- [ ] Fmt in actions
- [ ] Test in actions

# hops
