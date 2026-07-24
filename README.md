## TODO

### TODO

- [ ]
- [ ] logout functionality to revoke sessions
- [ ] Sms rate limiting/sms fraud bumping limiting. Try calling elks and ask for fraud detection
- [ ] Rate limiting with tower govenor
- [ ] Add soft-lock with captcha/proof-of-work for sub max limit attempts
      combine them; reset the counter when the lock window expires.
- [ ] A single `crypto.secret` keys both OTP and refresh-token HMACs
      ([main.rs:31](src/main.rs#L31)); consider domain separation.
- [ ] add a ValidatedJson struct so i dont need to call validate on structs
- [ ] change email/phone fucntionality
- [ ] expand the refresh tokens to have session as parent
- [ ] fix all todos
- [ ] rate limit refresh token with device id
- [ ] create sqlx tests for all auth flows so they can be ran in the pipeline
- [ ] social login with google and apple

### Auth / Security

- [ ] Rate limiting with tower-governor
- [ ] Lock recovery with OTP

### Decisions Needed

- [ ] Add runtime var for dryrun SMS in local env
- [ ] Drop email from user, make phone_number non-optional
- [ ] probably move to another architecture like feature folder (even tough it will break fast because of importing eachother)
- [ ] limit check on how many devices, the phone login takes in device id, can end up with many sessions if the client is stupid

### Proposals

- [ ] add test runtime to use a cache rather than sending actual messages
- [ ] add last_login_at for tracing user logins without refresh token for metrics
- [ ] create otp service, remove crypto from state, and inject into otp service
