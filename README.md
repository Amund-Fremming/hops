## TODO

### TODO

- [ ] expand the refresh tokens to have session as parent
- [ ] fix all todos
- [ ] rate limit refresh token with device id
- [ ] create sqlx tests for all auth flows so they can be ran in the pipeline

### Auth / Security

- [ ] Rate limiting with tower-governor
- [ ] Lock recovery with OTP

### Decisions Needed

- [ ] Add runtime var for dryrun SMS in local env
- [ ] Passwordless login requires given_name/family_name to be optional
- [ ] Drop email from user, make phone_number non-optional
- [ ] probably move to another architecture like feature folder (even tough it will break fast because of importing eachother)
- [ ] limit check on how many devices, the phone login takes in device id, can end up with many sessions if the client is stupid

### Proposals

- [ ] add test runtime to use a cache rather than sending actual messages
- [ ] add last_login_at for tracing user logins without refresh token for metrics
- [ ] should trace audit logs for failed logins
- [ ] create otp service, remove crypto from state, and inject into otp service

### CI/CD

- [ ] Audit in actions
- [ ] Clippy in actions
- [ ] Fmt in actions
- [ ] Test in actions

# hops
