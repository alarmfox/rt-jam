# RT-Jam

## Esecuzione dell'applicazione
Al momento dello sviluppo solo i browser chromium-based implementano l'API WebTransport: l'
applicazione quindi **NON** funziona su altri browser. Inoltre, il protocollo QUIC rende
obbligatorio l'utilizzo del protocollo TLS. In questa repository, sono già forniti dei 
certificati generati con `openssl` (vedi sezione successiva) per eseguire l'applicazione
localmente.

Attraverso lo script `launch_chrome.sh` viene eseguita un'istanaza di chrome con parametri 
disponibili solo da riga di comando che bypassano la verifica del certificato e forzano 
l'utilizzo di localhost per connettersi al protocollo QUIC. 
```sh 
 google-chrome --origin-to-force-quic-on=127.0.0.1:4433 --ignore-certificate-errors-spki-list="$SPKI" --enable-logging --v=1
```

### Generazione di certificati ssl
I certificati SSL vengono generati con i seguenti comandi:

```sh 
    openssl req -x509 -newkey rsa:2048 -keyout "backend/certs/localhost.dev.key" -out "backend/certs/localhost.dev.pem" -days 365 -nodes -subj "/CN=127.0.0.1"
    openssl x509 -in "backend/certs/localhost.dev.pem" -outform der -out "backend/certs/localhost.dev.der"
    openssl rsa -in "backend/certs/localhost.dev.key" -outform DER -out "backend/certs/localhost.dev.key.der"
```
