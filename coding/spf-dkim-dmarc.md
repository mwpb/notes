# SPF, DKIM and DMARC

## Sources

https://www.higherlogic.com/blog/spf-dkim-dmarc-email-authentication/

## Summary

- SPF: a whitelist of servers allowed to send email as this domain published via DNS records.
- DKIM: the email is signed using a private key and a public key is used to verify the signature.
- DMARC: a DNS entry that instructs a receiving mail server what to do if either SPF or DKIM break and where to send detailed reports of these failures.
