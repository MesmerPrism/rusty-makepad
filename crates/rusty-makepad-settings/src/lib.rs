//! Canonical Makepad app settings surfaces and deterministic resolver.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Schema id for Makepad app settings surfaces.
pub const SETTINGS_SURFACE_SCHEMA: &str = "rusty.gui.makepad.app_settings_surface.v1";
/// Schema id for Makepad settings profiles.
pub const SETTINGS_PROFILE_SCHEMA: &str = "rusty.gui.makepad.settings_profile.v1";
/// Schema id for effective settings reports.
pub const EFFECTIVE_SETTINGS_SCHEMA: &str = "rusty.gui.makepad.effective_settings.v1";
/// Schema id for hotload proposals.
pub const HOTLOAD_PROPOSAL_SCHEMA: &str = "rusty.gui.makepad.hotload_proposal.v1";
/// Schema id for hotload decisions.
pub const HOTLOAD_DECISION_SCHEMA: &str = "rusty.gui.makepad.hotload_decision.v1";
/// Schema id for hotload application results.
pub const HOTLOAD_APPLICATION_RESULT_SCHEMA: &str =
    "rusty.gui.makepad.hotload_application_result.v1";

/// Canonical settings surface for one Makepad app.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettingsSurface {
    /// Schema id.
    pub schema: String,
    /// Stable app id.
    pub app_id: String,
    /// Surface version.
    pub version: u32,
    /// Setting definitions.
    pub settings: Vec<SettingDefinition>,
}

/// One canonical setting definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingDefinition {
    /// Canonical setting id.
    pub id: String,
    /// Repo that owns the setting definition.
    pub owner_repo: String,
    /// Module or component that owns the setting.
    pub owner_module: String,
    /// Type accepted by the setting.
    pub value_type: SettingValueType,
    /// Optional unit label.
    #[serde(default)]
    pub unit: Option<String>,
    /// Optional numeric minimum.
    #[serde(default)]
    pub minimum: Option<f64>,
    /// Optional numeric maximum.
    #[serde(default)]
    pub maximum: Option<f64>,
    /// Accepted values for enum settings.
    #[serde(default)]
    pub enum_values: Vec<String>,
    /// Default value for the setting.
    #[serde(rename = "default")]
    pub default_value: Value,
    /// Hotload behavior for accepted changes.
    pub hotload_policy: HotloadPolicy,
    /// Which entry point may write the value.
    pub writer_policy: WriterPolicy,
    /// Public/sensitivity class.
    pub sensitivity: Sensitivity,
    /// External exposure names.
    #[serde(default)]
    pub exposures: Vec<SettingExposure>,
    /// Field name emitted in effective/readback reports.
    #[serde(default)]
    pub readback_field: Option<String>,
}

/// Supported setting value types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingValueType {
    /// Boolean true/false.
    Boolean,
    /// Signed or unsigned integer.
    Integer,
    /// Floating point number.
    Number,
    /// String value.
    String,
    /// String constrained to `enum_values`.
    Enum,
}

/// Hotload policy for a setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotloadPolicy {
    /// May change only at startup.
    StartupOnly,
    /// May change during a frame without rebuilding the scene.
    FrameSafe,
    /// Requires scene resources to be rebuilt.
    SceneRebuild,
    /// Requires app restart.
    RestartRequired,
}

/// Writer policy for a setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriterPolicy {
    /// Setting is read-only after schema/default definition.
    ReadOnly,
    /// Setting is owned by profiles.
    ProfileOwned,
    /// Setting may be changed by session hotload.
    SessionHotload,
    /// UI may propose changes through the resolver.
    UiProposed,
}

/// Sensitivity class for a setting value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sensitivity {
    /// Public-safe diagnostic or behavior setting.
    PublicSafe,
    /// Diagnostic setting that should be reviewed before publication.
    Diagnostic,
    /// Sensitive value that should not be emitted publicly.
    Sensitive,
}

/// External exposure name for a setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingExposure {
    /// Exposure kind.
    pub kind: ExposureKind,
    /// Exposure name, such as a CLI flag or Android property.
    pub name: String,
}

/// Supported setting exposure kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExposureKind {
    /// Command line flag.
    CliFlag,
    /// Environment variable.
    EnvironmentVariable,
    /// Android property.
    AndroidProperty,
    /// UI control id.
    UiControl,
    /// Manifest key.
    ManifestKey,
    /// Manifold command id.
    ManifoldCommand,
    /// Hotload file key.
    HotloadFile,
}

/// Profile value bundle over a settings surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsProfile {
    /// Schema id.
    pub schema: String,
    /// Stable profile id.
    pub profile_id: String,
    /// App id this profile targets.
    pub surface_app_id: String,
    /// Resolution layer represented by this profile.
    pub layer: SettingLayer,
    /// Profile values.
    pub values: Vec<ProfileValue>,
}

/// Resolution layer for setting values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingLayer {
    /// Schema default layer.
    SchemaDefault,
    /// App default fixture layer.
    AppDefault,
    /// Platform or device profile layer.
    PlatformProfile,
    /// Runtime behavior profile layer.
    RuntimeProfile,
    /// Launch or CLI override layer.
    LaunchOverride,
    /// Environment or Android property override layer.
    EnvironmentOverride,
    /// Hotload or session override layer.
    HotloadOverride,
}

impl SettingLayer {
    /// Stable layer label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SchemaDefault => "schema_default",
            Self::AppDefault => "app_default",
            Self::PlatformProfile => "platform_profile",
            Self::RuntimeProfile => "runtime_profile",
            Self::LaunchOverride => "launch_override",
            Self::EnvironmentOverride => "environment_override",
            Self::HotloadOverride => "hotload_override",
        }
    }

    /// Default precedence for deterministic resolution.
    pub fn precedence(self) -> u32 {
        match self {
            Self::SchemaDefault => 0,
            Self::AppDefault => 10,
            Self::PlatformProfile => 20,
            Self::RuntimeProfile => 30,
            Self::LaunchOverride => 40,
            Self::EnvironmentOverride => 50,
            Self::HotloadOverride => 60,
        }
    }
}

/// One proposed profile value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileValue {
    /// Canonical setting id.
    pub setting_id: String,
    /// Proposed value.
    pub value: Value,
}

/// One layer of proposed values over a settings surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsLayerValues {
    /// Resolution layer.
    pub layer: SettingLayer,
    /// Source id for provenance.
    pub source_id: String,
    /// Values proposed by this layer.
    pub values: Vec<ProfileValue>,
}

/// Hotload proposal over canonical setting ids.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotloadProposal {
    /// Schema id.
    pub schema: String,
    /// Stable proposal id.
    pub proposal_id: String,
    /// Target app id.
    pub app_id: String,
    /// Entry point or session id that proposed the change.
    pub source_id: String,
    /// Effective-settings revision the proposal was based on.
    pub requested_revision: u64,
    /// Proposed setting values.
    pub values: Vec<ProfileValue>,
}

/// Hotload proposal application result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotloadApplicationResult {
    /// Schema id.
    pub schema: String,
    /// Decision evidence.
    pub decision: HotloadDecision,
    /// Effective settings after accepted hotload values were applied.
    pub report: EffectiveSettingsReport,
}

/// Accepted/rejected hotload decision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotloadDecision {
    /// Schema id.
    pub schema: String,
    /// Stable proposal id.
    pub proposal_id: String,
    /// Target app id.
    pub app_id: String,
    /// Entry point or session id that proposed the change.
    pub source_id: String,
    /// Effective-settings revision the proposal was based on.
    pub requested_revision: u64,
    /// Effective-settings revision produced by this decision.
    pub accepted_revision: u64,
    /// Decision timestamp or stable fixture timestamp.
    pub generated_at: String,
    /// Overall decision status.
    pub status: HotloadDecisionStatus,
    /// Accepted values.
    pub accepted_values: Vec<AcceptedHotloadValue>,
    /// Rejected values.
    pub rejected_values: Vec<RejectedHotloadValue>,
}

/// Hotload decision status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotloadDecisionStatus {
    /// Every proposed value was accepted.
    Accepted,
    /// Some values were accepted and some were rejected.
    PartiallyAccepted,
    /// No proposed values were accepted.
    Rejected,
}

/// Runtime action required for an accepted hotload value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotloadRequiredAction {
    /// Apply during the current or next frame.
    ApplyThisFrame,
    /// Rebuild scene resources before the value takes effect.
    RebuildScene,
}

/// Accepted hotload value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcceptedHotloadValue {
    /// Canonical setting id.
    pub setting_id: String,
    /// Accepted value.
    pub value: Value,
    /// Action required before the value takes effect.
    pub required_action: HotloadRequiredAction,
}

/// Rejected hotload value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RejectedHotloadValue {
    /// Canonical setting id.
    pub setting_id: String,
    /// Rejected value.
    pub value: Value,
    /// Rejection reason.
    pub reason: String,
}

/// Effective settings report after deterministic resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectiveSettingsReport {
    /// Schema id.
    pub schema: String,
    /// Stable app id.
    pub app_id: String,
    /// Settings surface schema id.
    pub surface_schema: String,
    /// App settings surface version.
    pub surface_version: u32,
    /// Effective settings revision.
    pub revision: u64,
    /// Generation timestamp or stable test timestamp.
    pub generated_at: String,
    /// Effective values.
    pub settings: Vec<EffectiveSetting>,
}

/// One effective setting with provenance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectiveSetting {
    /// Canonical setting id.
    pub setting_id: String,
    /// Final value.
    pub value: Value,
    /// Winning layer.
    pub winning_layer: String,
    /// Winning source id.
    pub winning_source_id: String,
    /// Rejected lower-priority values.
    pub rejected_layers: Vec<RejectedSettingLayer>,
    /// Hotload policy from the settings surface.
    pub hotload_policy: HotloadPolicy,
    /// Writer policy from the settings surface.
    pub writer_policy: WriterPolicy,
    /// Readback field from the settings surface.
    pub readback_field: Option<String>,
}

/// A value rejected because a higher-priority layer won.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RejectedSettingLayer {
    /// Rejected layer.
    pub layer: String,
    /// Rejected source id.
    pub source_id: String,
    /// Rejected value.
    pub value: Value,
    /// Rejection reason.
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
struct SettingCandidate {
    value: Value,
    layer: SettingLayer,
    source_id: String,
    layer_order: usize,
}

/// Validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Human-readable message.
    pub message: String,
}

impl ValidationError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Validate a settings surface.
pub fn validate_surface(surface: &AppSettingsSurface) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if surface.schema != SETTINGS_SURFACE_SCHEMA {
        errors.push(ValidationError::new(format!(
            "unsupported surface schema {}",
            surface.schema
        )));
    }
    if surface.app_id.trim().is_empty() {
        errors.push(ValidationError::new("app_id must not be empty"));
    }
    if surface.settings.is_empty() {
        errors.push(ValidationError::new(
            "settings surface must contain at least one setting",
        ));
    }

    let mut ids = BTreeSet::new();
    let mut exposures: BTreeMap<(ExposureKind, String), String> = BTreeMap::new();
    for setting in &surface.settings {
        if !ids.insert(setting.id.clone()) {
            errors.push(ValidationError::new(format!(
                "duplicate setting id {}",
                setting.id
            )));
        }
        if setting.owner_repo.trim().is_empty() || setting.owner_module.trim().is_empty() {
            errors.push(ValidationError::new(format!(
                "setting {} must declare owner_repo and owner_module",
                setting.id
            )));
        }
        validate_value(setting, &setting.default_value, "default", &mut errors);
        if setting.value_type == SettingValueType::Enum && setting.enum_values.is_empty() {
            errors.push(ValidationError::new(format!(
                "enum setting {} must declare enum_values",
                setting.id
            )));
        }
        for exposure in &setting.exposures {
            if exposure.name.trim().is_empty() {
                errors.push(ValidationError::new(format!(
                    "setting {} has empty exposure name",
                    setting.id
                )));
                continue;
            }
            let key = (exposure.kind, exposure.name.clone());
            if let Some(existing) = exposures.insert(key, setting.id.clone()) {
                errors.push(ValidationError::new(format!(
                    "exposure {} maps to both {} and {}",
                    exposure.name, existing, setting.id
                )));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate a profile against a settings surface.
pub fn validate_profile(
    surface: &AppSettingsSurface,
    profile: &SettingsProfile,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if let Err(surface_errors) = validate_surface(surface) {
        errors.extend(surface_errors);
    }
    if profile.schema != SETTINGS_PROFILE_SCHEMA {
        errors.push(ValidationError::new(format!(
            "unsupported profile schema {}",
            profile.schema
        )));
    }
    if profile.surface_app_id != surface.app_id {
        errors.push(ValidationError::new(format!(
            "profile {} targets {}, expected {}",
            profile.profile_id, profile.surface_app_id, surface.app_id
        )));
    }
    let settings_by_id: BTreeMap<&str, &SettingDefinition> = surface
        .settings
        .iter()
        .map(|setting| (setting.id.as_str(), setting))
        .collect();
    let mut seen_values = BTreeSet::new();
    for value in &profile.values {
        if !seen_values.insert(value.setting_id.clone()) {
            errors.push(ValidationError::new(format!(
                "profile {} repeats setting {}",
                profile.profile_id, value.setting_id
            )));
            continue;
        }
        let Some(setting) = settings_by_id.get(value.setting_id.as_str()) else {
            errors.push(ValidationError::new(format!(
                "profile {} references unknown setting {}",
                profile.profile_id, value.setting_id
            )));
            continue;
        };
        validate_value(setting, &value.value, "profile value", &mut errors);
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Resolve a profile over a settings surface.
pub fn resolve_profile(
    surface: &AppSettingsSurface,
    profile: &SettingsProfile,
    revision: u64,
    generated_at: impl Into<String>,
) -> Result<EffectiveSettingsReport, Vec<ValidationError>> {
    validate_profile(surface, profile)?;
    resolve_layers(
        surface,
        [SettingsLayerValues {
            layer: profile.layer,
            source_id: profile.profile_id.clone(),
            values: profile.values.clone(),
        }],
        revision,
        generated_at,
    )
}

/// Resolve multiple deterministic layers over a settings surface.
pub fn resolve_layers(
    surface: &AppSettingsSurface,
    layers: impl IntoIterator<Item = SettingsLayerValues>,
    revision: u64,
    generated_at: impl Into<String>,
) -> Result<EffectiveSettingsReport, Vec<ValidationError>> {
    let mut errors = Vec::new();
    if let Err(surface_errors) = validate_surface(surface) {
        errors.extend(surface_errors);
    }

    let settings_by_id: BTreeMap<&str, &SettingDefinition> = surface
        .settings
        .iter()
        .map(|setting| (setting.id.as_str(), setting))
        .collect();
    let mut candidates: BTreeMap<&str, Vec<SettingCandidate>> = BTreeMap::new();
    for setting in &surface.settings {
        candidates
            .entry(setting.id.as_str())
            .or_default()
            .push(SettingCandidate {
                value: setting.default_value.clone(),
                layer: SettingLayer::SchemaDefault,
                source_id: format!("{}.default", surface.app_id),
                layer_order: 0,
            });
    }

    for (index, layer) in layers.into_iter().enumerate() {
        if layer.source_id.trim().is_empty() {
            errors.push(ValidationError::new(format!(
                "{} layer must declare source_id",
                layer.layer.as_str()
            )));
        }
        let mut seen_values = BTreeSet::new();
        for proposed in layer.values {
            if !seen_values.insert(proposed.setting_id.clone()) {
                errors.push(ValidationError::new(format!(
                    "layer {} repeats setting {}",
                    layer.source_id, proposed.setting_id
                )));
                continue;
            }
            let Some(setting) = settings_by_id.get(proposed.setting_id.as_str()) else {
                errors.push(ValidationError::new(format!(
                    "layer {} references unknown setting {}",
                    layer.source_id, proposed.setting_id
                )));
                continue;
            };
            validate_value(setting, &proposed.value, "layer value", &mut errors);
            candidates
                .entry(setting.id.as_str())
                .or_default()
                .push(SettingCandidate {
                    value: proposed.value,
                    layer: layer.layer,
                    source_id: layer.source_id.clone(),
                    layer_order: index + 1,
                });
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let mut settings = Vec::with_capacity(surface.settings.len());
    for setting in &surface.settings {
        let mut setting_candidates = candidates
            .remove(setting.id.as_str())
            .expect("defaults should create candidates for every setting");
        setting_candidates.sort_by(|left, right| {
            right
                .layer
                .precedence()
                .cmp(&left.layer.precedence())
                .then_with(|| right.layer_order.cmp(&left.layer_order))
        });
        let winner = setting_candidates
            .first()
            .expect("setting must have at least the default candidate")
            .clone();
        let rejected_layers = setting_candidates
            .iter()
            .skip(1)
            .map(|candidate| RejectedSettingLayer {
                layer: candidate.layer.as_str().to_string(),
                source_id: candidate.source_id.clone(),
                value: candidate.value.clone(),
                reason: format!("overridden_by_{}", winner.layer.as_str()),
            })
            .collect();
        settings.push(EffectiveSetting {
            setting_id: setting.id.clone(),
            value: winner.value,
            winning_layer: winner.layer.as_str().to_string(),
            winning_source_id: winner.source_id,
            rejected_layers,
            hotload_policy: setting.hotload_policy,
            writer_policy: setting.writer_policy,
            readback_field: setting.readback_field.clone(),
        });
    }

    Ok(EffectiveSettingsReport {
        schema: EFFECTIVE_SETTINGS_SCHEMA.to_string(),
        app_id: surface.app_id.clone(),
        surface_schema: surface.schema.clone(),
        surface_version: surface.version,
        revision,
        generated_at: generated_at.into(),
        settings,
    })
}

/// Apply a hotload proposal over existing setting layers.
pub fn apply_hotload_proposal(
    surface: &AppSettingsSurface,
    base_layers: impl IntoIterator<Item = SettingsLayerValues>,
    proposal: &HotloadProposal,
    accepted_revision: u64,
    generated_at: impl Into<String>,
) -> Result<HotloadApplicationResult, Vec<ValidationError>> {
    let generated_at = generated_at.into();
    validate_hotload_proposal_header(surface, proposal)?;

    let settings_by_id: BTreeMap<&str, &SettingDefinition> = surface
        .settings
        .iter()
        .map(|setting| (setting.id.as_str(), setting))
        .collect();

    let mut accepted_values = Vec::new();
    let mut accepted_profile_values = Vec::new();
    let mut rejected_values = Vec::new();
    let mut seen_values = BTreeSet::new();

    for proposed in &proposal.values {
        if !seen_values.insert(proposed.setting_id.clone()) {
            rejected_values.push(RejectedHotloadValue {
                setting_id: proposed.setting_id.clone(),
                value: proposed.value.clone(),
                reason: "duplicate_setting_in_proposal".to_string(),
            });
            continue;
        }

        let Some(setting) = settings_by_id.get(proposed.setting_id.as_str()) else {
            rejected_values.push(RejectedHotloadValue {
                setting_id: proposed.setting_id.clone(),
                value: proposed.value.clone(),
                reason: "unknown_setting".to_string(),
            });
            continue;
        };

        if let Some(reason) = hotload_rejection_reason(setting) {
            rejected_values.push(RejectedHotloadValue {
                setting_id: proposed.setting_id.clone(),
                value: proposed.value.clone(),
                reason,
            });
            continue;
        }

        let value_errors = validation_errors_for_value(setting, &proposed.value, "hotload value");
        if !value_errors.is_empty() {
            rejected_values.push(RejectedHotloadValue {
                setting_id: proposed.setting_id.clone(),
                value: proposed.value.clone(),
                reason: value_errors
                    .into_iter()
                    .map(|error| sanitize_rejection_reason(&error.message))
                    .collect::<Vec<_>>()
                    .join(";"),
            });
            continue;
        }

        let required_action = match setting.hotload_policy {
            HotloadPolicy::FrameSafe => HotloadRequiredAction::ApplyThisFrame,
            HotloadPolicy::SceneRebuild => HotloadRequiredAction::RebuildScene,
            HotloadPolicy::StartupOnly | HotloadPolicy::RestartRequired => {
                unreachable!("hotload policy rejection should run before action selection")
            }
        };
        accepted_values.push(AcceptedHotloadValue {
            setting_id: proposed.setting_id.clone(),
            value: proposed.value.clone(),
            required_action,
        });
        accepted_profile_values.push(proposed.clone());
    }

    let mut layers: Vec<SettingsLayerValues> = base_layers.into_iter().collect();
    if !accepted_profile_values.is_empty() {
        layers.push(SettingsLayerValues {
            layer: SettingLayer::HotloadOverride,
            source_id: proposal.source_id.clone(),
            values: accepted_profile_values,
        });
    }

    let report = resolve_layers(surface, layers, accepted_revision, generated_at.clone())?;
    let status = match (accepted_values.is_empty(), rejected_values.is_empty()) {
        (false, true) => HotloadDecisionStatus::Accepted,
        (false, false) => HotloadDecisionStatus::PartiallyAccepted,
        (true, _) => HotloadDecisionStatus::Rejected,
    };
    let decision = HotloadDecision {
        schema: HOTLOAD_DECISION_SCHEMA.to_string(),
        proposal_id: proposal.proposal_id.clone(),
        app_id: proposal.app_id.clone(),
        source_id: proposal.source_id.clone(),
        requested_revision: proposal.requested_revision,
        accepted_revision,
        generated_at,
        status,
        accepted_values,
        rejected_values,
    };

    Ok(HotloadApplicationResult {
        schema: HOTLOAD_APPLICATION_RESULT_SCHEMA.to_string(),
        decision,
        report,
    })
}

/// Build log marker lines for an effective settings report.
pub fn effective_settings_marker_lines(
    marker: &str,
    backend: &str,
    phase: &str,
    report: &EffectiveSettingsReport,
) -> Vec<String> {
    const FIELDS_PER_LINE: usize = 4;
    let marker = sanitize_marker_token(marker);
    let backend = sanitize_marker_token(backend);
    let phase = sanitize_marker_token(phase);
    let fields = report
        .settings
        .iter()
        .map(effective_setting_marker_token)
        .collect::<Vec<_>>();
    let part_count = fields.len().div_ceil(FIELDS_PER_LINE).max(1);
    if fields.is_empty() {
        return vec![format!(
            "{} schema={} app={} phase={} backend={} revision={} part=1/1 fieldCount=0 fields=none",
            marker,
            EFFECTIVE_SETTINGS_SCHEMA,
            sanitize_marker_token(&report.app_id),
            phase,
            backend,
            report.revision
        )];
    }
    fields
        .chunks(FIELDS_PER_LINE)
        .enumerate()
        .map(|(index, chunk)| {
            format!(
                "{} schema={} app={} phase={} backend={} revision={} part={}/{} fieldCount={} fields={}",
                marker,
                EFFECTIVE_SETTINGS_SCHEMA,
                sanitize_marker_token(&report.app_id),
                phase,
                backend,
                report.revision,
                index + 1,
                part_count,
                fields.len(),
                chunk.join(";")
            )
        })
        .collect()
}

fn effective_setting_marker_token(setting: &EffectiveSetting) -> String {
    format!(
        "{}[value={},layer={},source={},rejected={},hotload={:?}]",
        sanitize_marker_token(&setting.setting_id),
        sanitize_marker_token(&setting.value.to_string()),
        sanitize_marker_token(&setting.winning_layer),
        sanitize_marker_token(&setting.winning_source_id),
        setting.rejected_layers.len(),
        setting.hotload_policy
    )
}

fn sanitize_marker_token(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' | ':' => ch,
            _ => '_',
        })
        .collect()
}

fn validate_value(
    setting: &SettingDefinition,
    value: &Value,
    label: &str,
    errors: &mut Vec<ValidationError>,
) {
    match setting.value_type {
        SettingValueType::Boolean => {
            if !value.is_boolean() {
                errors.push(value_error(setting, value, label, "boolean"));
            }
        }
        SettingValueType::Integer => {
            let number = value.as_i64().map(|number| number as f64).or_else(|| {
                value
                    .as_u64()
                    .and_then(|number| (number <= i64::MAX as u64).then_some(number as f64))
            });
            validate_number(setting, value, label, number, errors, "integer");
        }
        SettingValueType::Number => {
            validate_number(setting, value, label, value.as_f64(), errors, "number");
        }
        SettingValueType::String => {
            if !value.is_string() {
                errors.push(value_error(setting, value, label, "string"));
            }
        }
        SettingValueType::Enum => {
            let Some(value_string) = value.as_str() else {
                errors.push(value_error(setting, value, label, "enum string"));
                return;
            };
            if !setting
                .enum_values
                .iter()
                .any(|allowed| allowed == value_string)
            {
                errors.push(ValidationError::new(format!(
                    "{} for {} has invalid enum value {:?}",
                    label, setting.id, value
                )));
            }
        }
    }
}

fn validate_number(
    setting: &SettingDefinition,
    value: &Value,
    label: &str,
    number: Option<f64>,
    errors: &mut Vec<ValidationError>,
    expected: &str,
) {
    let Some(number) = number else {
        errors.push(value_error(setting, value, label, expected));
        return;
    };
    if let Some(minimum) = setting.minimum {
        if number < minimum {
            errors.push(ValidationError::new(format!(
                "{} for {} is below minimum {}",
                label, setting.id, minimum
            )));
        }
    }
    if let Some(maximum) = setting.maximum {
        if number > maximum {
            errors.push(ValidationError::new(format!(
                "{} for {} is above maximum {}",
                label, setting.id, maximum
            )));
        }
    }
}

fn value_error(
    setting: &SettingDefinition,
    value: &Value,
    label: &str,
    expected: &str,
) -> ValidationError {
    ValidationError::new(format!(
        "{} for {} must be {}, got {:?}",
        label, setting.id, expected, value
    ))
}

fn validate_hotload_proposal_header(
    surface: &AppSettingsSurface,
    proposal: &HotloadProposal,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if let Err(surface_errors) = validate_surface(surface) {
        errors.extend(surface_errors);
    }
    if proposal.schema != HOTLOAD_PROPOSAL_SCHEMA {
        errors.push(ValidationError::new(format!(
            "unsupported hotload proposal schema {}",
            proposal.schema
        )));
    }
    if proposal.proposal_id.trim().is_empty() {
        errors.push(ValidationError::new("proposal_id must not be empty"));
    }
    if proposal.app_id != surface.app_id {
        errors.push(ValidationError::new(format!(
            "hotload proposal {} targets {}, expected {}",
            proposal.proposal_id, proposal.app_id, surface.app_id
        )));
    }
    if proposal.source_id.trim().is_empty() {
        errors.push(ValidationError::new("source_id must not be empty"));
    }
    if proposal.values.is_empty() {
        errors.push(ValidationError::new(
            "hotload proposal must contain at least one value",
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn hotload_rejection_reason(setting: &SettingDefinition) -> Option<String> {
    if !matches!(
        setting.writer_policy,
        WriterPolicy::SessionHotload | WriterPolicy::UiProposed
    ) {
        return Some(format!(
            "writer_policy_{}_does_not_accept_hotload",
            writer_policy_id(setting.writer_policy)
        ));
    }
    match setting.hotload_policy {
        HotloadPolicy::FrameSafe | HotloadPolicy::SceneRebuild => None,
        HotloadPolicy::StartupOnly => Some("hotload_policy_startup_only".to_string()),
        HotloadPolicy::RestartRequired => Some("hotload_policy_restart_required".to_string()),
    }
}

fn writer_policy_id(policy: WriterPolicy) -> &'static str {
    match policy {
        WriterPolicy::ReadOnly => "read_only",
        WriterPolicy::ProfileOwned => "profile_owned",
        WriterPolicy::SessionHotload => "session_hotload",
        WriterPolicy::UiProposed => "ui_proposed",
    }
}

fn validation_errors_for_value(
    setting: &SettingDefinition,
    value: &Value,
    label: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    validate_value(setting, value, label, &mut errors);
    errors
}

fn sanitize_rejection_reason(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' | ':' => ch,
            _ => '_',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        apply_hotload_proposal, effective_settings_marker_lines, resolve_layers, resolve_profile,
        validate_profile, validate_surface, AppSettingsSurface, HotloadDecisionStatus,
        HotloadProposal, HotloadRequiredAction, ProfileValue, SettingLayer, SettingsLayerValues,
        SettingsProfile, EFFECTIVE_SETTINGS_SCHEMA,
    };
    use serde_json::json;

    fn surface() -> AppSettingsSurface {
        serde_json::from_str(include_str!(
            "../../../fixtures/settings/makepad-app-surface.json"
        ))
        .expect("valid surface JSON")
    }

    fn profile() -> SettingsProfile {
        serde_json::from_str(include_str!(
            "../../../fixtures/profiles/mesh-replay-fast.profile.json"
        ))
        .expect("valid profile JSON")
    }

    fn hotload_proposal() -> HotloadProposal {
        serde_json::from_str(include_str!(
            "../../../fixtures/hotload/mesh-replay-hotload.proposal.json"
        ))
        .expect("valid hotload proposal JSON")
    }

    #[test]
    fn valid_surface_passes() {
        validate_surface(&surface()).expect("surface should validate");
    }

    #[test]
    fn valid_profile_resolves_with_provenance() {
        let surface = surface();
        let profile = profile();
        let report =
            resolve_profile(&surface, &profile, 7, "2026-06-09T00:00:00Z").expect("resolve");
        assert_eq!(report.schema, EFFECTIVE_SETTINGS_SCHEMA);
        assert_eq!(report.revision, 7);
        let speed = report
            .settings
            .iter()
            .find(|setting| setting.setting_id == "makepad.mesh_replay.speed")
            .expect("speed setting");
        assert_eq!(speed.winning_layer, "runtime_profile");
        assert_eq!(speed.rejected_layers.len(), 1);
    }

    #[test]
    fn higher_layers_win_and_rejected_layers_are_recorded() {
        let surface = surface();
        let report = resolve_layers(
            &surface,
            [
                SettingsLayerValues {
                    layer: SettingLayer::RuntimeProfile,
                    source_id: "profile.mesh".to_string(),
                    values: vec![ProfileValue {
                        setting_id: "makepad.mesh_replay.speed".to_string(),
                        value: json!(2.0),
                    }],
                },
                SettingsLayerValues {
                    layer: SettingLayer::HotloadOverride,
                    source_id: "hotload.session.7".to_string(),
                    values: vec![ProfileValue {
                        setting_id: "makepad.mesh_replay.speed".to_string(),
                        value: json!(3.0),
                    }],
                },
            ],
            8,
            "2026-06-09T00:00:00Z",
        )
        .expect("resolve layers");
        let speed = report
            .settings
            .iter()
            .find(|setting| setting.setting_id == "makepad.mesh_replay.speed")
            .expect("speed setting");
        assert_eq!(speed.value, json!(3.0));
        assert_eq!(speed.winning_layer, "hotload_override");
        assert_eq!(speed.rejected_layers.len(), 2);
    }

    #[test]
    fn marker_lines_include_effective_settings_trace() {
        let report = resolve_profile(&surface(), &profile(), 9, "2026-06-09T00:00:00Z")
            .expect("resolve profile");
        let lines = effective_settings_marker_lines(
            "RUSTY_MAKEPAD_EFFECTIVE_SETTINGS",
            "test",
            "startup",
            &report,
        );
        let joined = lines.join("\n");
        assert!(joined.contains("schema=rusty.gui.makepad.effective_settings.v1"));
        assert!(joined.contains("makepad.mesh_replay.speed"));
        assert!(joined.contains("layer=runtime_profile"));
    }

    #[test]
    fn hotload_proposal_accepts_only_policy_allowed_values() {
        let surface = surface();
        let profile = profile();
        let result = apply_hotload_proposal(
            &surface,
            [SettingsLayerValues {
                layer: profile.layer,
                source_id: profile.profile_id,
                values: profile.values,
            }],
            &hotload_proposal(),
            10,
            "2026-06-09T00:00:00Z",
        )
        .expect("hotload proposal should apply");

        assert_eq!(
            result.decision.status,
            HotloadDecisionStatus::PartiallyAccepted
        );
        assert_eq!(result.decision.accepted_values.len(), 1);
        assert_eq!(
            result.decision.accepted_values[0].required_action,
            HotloadRequiredAction::ApplyThisFrame
        );
        assert!(result
            .decision
            .rejected_values
            .iter()
            .any(|value| value.reason.contains("writer_policy_profile_owned")));
        assert!(result
            .decision
            .rejected_values
            .iter()
            .any(|value| value.reason == "unknown_setting"));

        let speed = result
            .report
            .settings
            .iter()
            .find(|setting| setting.setting_id == "makepad.mesh_replay.speed")
            .expect("speed setting");
        assert_eq!(speed.value, json!(3.5));
        assert_eq!(speed.winning_layer, "hotload_override");
    }

    #[test]
    fn duplicate_setting_ids_are_rejected() {
        let damaged: AppSettingsSurface = serde_json::from_str(include_str!(
            "../../../fixtures/damaged/duplicate-setting-id.surface.json"
        ))
        .expect("damaged surface JSON");
        let errors = validate_surface(&damaged).expect_err("must reject duplicate ids");
        assert!(errors
            .iter()
            .any(|error| error.message.contains("duplicate setting id")));
    }

    #[test]
    fn unknown_profile_setting_is_rejected() {
        let damaged: SettingsProfile = serde_json::from_str(include_str!(
            "../../../fixtures/damaged/unknown-profile-setting.profile.json"
        ))
        .expect("damaged profile JSON");
        let errors = validate_profile(&surface(), &damaged).expect_err("must reject unknown id");
        assert!(errors
            .iter()
            .any(|error| error.message.contains("unknown setting")));
    }

    #[test]
    fn profile_values_outside_range_are_rejected() {
        let damaged: SettingsProfile = serde_json::from_str(include_str!(
            "../../../fixtures/damaged/invalid-range.profile.json"
        ))
        .expect("damaged profile JSON");
        let errors = validate_profile(&surface(), &damaged).expect_err("must reject range");
        assert!(errors
            .iter()
            .any(|error| error.message.contains("above maximum")));
    }
}
