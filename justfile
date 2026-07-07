run:
    cargo run

ex-comms:
    cargo run --example comms_example

reset-db:
    cargo sqlx database reset --force -y

generate-keys:
    #!/bin/bash
    openssl genrsa -out /tmp/private.pem 2048 2>/dev/null
    openssl rsa -in /tmp/private.pem -pubout -out /tmp/public.pem 2>/dev/null
    echo "APP__AUTH__PRIVATE_KEY_BASE64=$(cat /tmp/private.pem | base64 | tr -d '\n')"
    echo ""
    echo "APP__AUTH__PUBLIC_KEY_BASE64=$(cat /tmp/public.pem | base64 | tr -d '\n')"
    rm /tmp/private.pem /tmp/public.pem