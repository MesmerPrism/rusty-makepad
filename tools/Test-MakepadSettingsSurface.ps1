$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name,
        [Parameter(Mandatory=$true)]
        [string]$File,
        [string[]]$Arguments = @()
    )

    & $File @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE"
    }
}

function Invoke-ExpectedFailure {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name,
        [Parameter(Mandatory=$true)]
        [string[]]$Arguments
    )

    & cargo @Arguments
    if ($LASTEXITCODE -eq 0) {
        throw "$Name unexpectedly succeeded"
    }
}

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $RepoRoot
try {
    $surface = "fixtures\settings\makepad-app-surface.json"
    $profile = "fixtures\profiles\mesh-replay-fast.profile.json"
    $proposal = "fixtures\hotload\mesh-replay-hotload.proposal.json"
    Invoke-Checked "valid settings surface" "cargo" @("run", "-p", "rusty-makepad-settings-cli", "--", "validate-surface", "--surface", $surface, "--profile", $profile)
    Invoke-Checked "effective settings resolve" "cargo" @("run", "-p", "rusty-makepad-settings-cli", "--", "resolve", "--surface", $surface, "--profile", $profile, "--out", "local-artifacts\effective-settings.json")
    Invoke-Checked "hotload proposal decision" "cargo" @("run", "-p", "rusty-makepad-settings-cli", "--", "hotload", "--surface", $surface, "--profile", $profile, "--proposal", $proposal, "--out", "local-artifacts\hotload-result.json")
    $hotload = Get-Content -Path "local-artifacts\hotload-result.json" -Raw | ConvertFrom-Json
    if ($hotload.decision.status -ne "partially_accepted") {
        throw "hotload decision status mismatch: $($hotload.decision.status)"
    }
    if ($hotload.decision.accepted_values.Count -ne 1) {
        throw "hotload accepted value count mismatch: $($hotload.decision.accepted_values.Count)"
    }
    if ($hotload.decision.rejected_values.Count -lt 1) {
        throw "hotload rejected values were not recorded"
    }
    Invoke-ExpectedFailure "duplicate setting id" @("run", "-p", "rusty-makepad-settings-cli", "--", "validate-surface", "--surface", "fixtures\damaged\duplicate-setting-id.surface.json")
    Invoke-ExpectedFailure "unknown setting profile" @("run", "-p", "rusty-makepad-settings-cli", "--", "validate-surface", "--surface", $surface, "--profile", "fixtures\damaged\unknown-profile-setting.profile.json")
    Invoke-ExpectedFailure "invalid range profile" @("run", "-p", "rusty-makepad-settings-cli", "--", "validate-surface", "--surface", $surface, "--profile", "fixtures\damaged\invalid-range.profile.json")
} finally {
    Pop-Location
}
