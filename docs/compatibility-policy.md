# Compatibility Policy

Compatibility targets Unciv 4.21.10 at commit `19de45e6eef6245f1a9774b2e1a080a0f1c87174`.

An inventory item may be marked complete only with an automated fixture or differential scenario. Differences from upstream require an exception document under `docs/compatibility-exceptions/` containing the affected IDs, observable difference, reason, user impact, tests, and approval status.

The Phase 2 smoke fixture normalizes seeded map coordinates, base terrain, and terrain features. It intentionally omits resources because Unciv 4.21.10 resource placement varies between fresh JVM runs for the same seed. Resource parity requires distribution/invariant fixtures until deterministic upstream behavior can be isolated.

Allowed statuses are `pending`, `implemented`, `alias`, `deprecated`, and `exception`. An exception is not a silent omission: it must link to a reviewed document and a regression test encoding the chosen behavior.
