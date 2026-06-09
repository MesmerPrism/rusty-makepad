# Rusty Makepad Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

The validation gate checks formatting, unit tests, valid fixtures, damaged
fixtures, and boundary scans for legacy naming or ad hoc platform writes.

