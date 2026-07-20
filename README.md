## TODO

### TODO

- [ ] expand the refresh tokens to have session as parent
- [ ] fix all todos
- [ ] rate limit refresh token with device id

### Auth / Security

- [ ] Rate limiting with tower-governor
- [ ] Lock recovery with OTP

### Decisions Needed

- [ ] Add runtime var for dryrun SMS in local env
- [ ] Passwordless login requires given_name/family_name to be optional
- [ ] Drop email from user, make phone_number non-optional

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
