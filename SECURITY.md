# Security Policy

We take security seriously. Thank you for helping keep TKAWEN safe.

## Reporting a vulnerability

**Do not open public issues for security vulnerabilities.**

Email **security@tkawen.com** (or `DIRECTION@takawen.dz` if the first bounces) with:

- A description of the issue
- Steps to reproduce
- Affected versions or commits
- Your name / handle (for credit, if desired)

We will:

- **Acknowledge** within **24 hours**
- **Triage** and respond with severity assessment within **72 hours**
- **Patch** critical issues within **7 days**, others within **30 days**
- **Credit** you publicly in the changelog (with consent), or keep you anonymous if preferred

## Supported versions

Security patches are issued for the **latest stable release** and the **previous minor** version. Older versions are unsupported.

## Scope

In scope:

- All code in this repository
- The deployed service it powers, where applicable
- Configuration we ship (Docker images, systemd units, nginx configs)

Out of scope:

- Third-party dependencies (please report upstream — but we appreciate a heads-up)
- DoS via traffic volume against our production
- Issues already in our public issue tracker
- Spam / social engineering against employees

## Coordinated disclosure

We follow a **90-day coordinated disclosure** window. After we ship a fix and notify affected users, you are free to publish your findings.

If you believe the issue requires immediate public disclosure for safety, contact us first to discuss the timeline.

## PGP

A PGP key for encrypted reports is available on request.
