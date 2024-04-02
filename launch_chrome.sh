#!/bin/bash

set -e

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
CERTSPATH="$SCRIPTPATH/backend/certs"

SPKI=$(openssl x509 -inform der -in "$CERTSPATH/localhost.dev.der" -pubkey -noout | openssl pkey -pubin -outform der | openssl dgst -sha256 -binary | openssl enc -base64)

echo "Opening google chrome"

case $(uname) in
    (*Linux*)  thorium-browser --origin-to-force-quic-on=127.0.0.1:4433 --ignore-certificate-errors-spki-list="$SPKI" --enable-logging --v=1 ;;
    (*Darwin*)  open -a "Google Chrome" --args --origin-to-force-quic-on=127.0.0.1:4433 --ignore-certificate-errors-spki-list="$SPKI" --enable-logging --v=1 ;;
esac

