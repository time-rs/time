# `time` security policy

## No guarantees

As specified in the license, `time` is provided as-is with no guarantees whatsoever.

## Supported versions

The latest release of `time` is the only supported version. Security patches will not be backported
unless otherwise specified by a support contract.

### Included crates

All repositories in the `time-rs` organization are included in this security policy. This includes,
at the time of writing,

- `time`
- `time-core`
- `time-macros`

## Reporting a vulnerability

Security vulnerabilities are taken seriously. If you discover a security vulnerability in `time`,
please report it by email using the email address on [the maintainer's GitHub profile][gh-profile].
Please do not disclose the vulnerability publicly until an opportunity has been given to investigate
and release a patch.

[gh-profile]: https://github.com/jhpratt

When reporting a vulnerability, please include the following information if possible:

- A description of the vulnerability
- Steps to reproduce the vulnerability
- Any proof of concept code

After receiving notice of a potential vulnerability, it will be investigated to determine whether
there is in fact a vulnerability. If it is determined that there is, a patch will be developed and
released in a timely manner, keeping in mind that this is a volunteer project and that the work is
being done in the maintainer's free time. After a patch is released, the vulnerability will be
publicly disclosed, and the reporter will be credited. A CVE and/or RUSTSEC advisory will be
requested as appropriate.
