run:
    cargo run

ex-comms:
    cargo run --example comms_example

reset-db:
    cargo sqlx database reset --force -y