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
    fn as_str(self) -> &'static str {
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
}

/// One proposed profile value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileValue {
    /// Canonical setting id.
    pub setting_id: String,
    /// Proposed value.
    pub value: Value,
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

    let mut profile_values: BTreeMap<&str, &ProfileValue> = BTreeMap::new();
    for value in &profile.values {
        profile_values.insert(value.setting_id.as_str(), value);
    }

    let mut settings = Vec::with_capacity(surface.settings.len());
    for setting in &surface.settings {
        let default_layer = RejectedSettingLayer {
            layer: SettingLayer::SchemaDefault.as_str().to_string(),
            source_id: format!("{}.default", surface.app_id),
            value: setting.default_value.clone(),
            reason: format!("overridden_by_{}", profile.layer.as_str()),
        };
        let (value, winning_layer, winning_source_id, rejected_layers) =
            if let Some(profile_value) = profile_values.get(setting.id.as_str()) {
                (
                    profile_value.value.clone(),
                    profile.layer.as_str().to_string(),
                    profile.profile_id.clone(),
                    vec![default_layer],
                )
            } else {
                (
                    setting.default_value.clone(),
                    SettingLayer::SchemaDefault.as_str().to_string(),
                    format!("{}.default", surface.app_id),
                    Vec::new(),
                )
            };
        settings.push(EffectiveSetting {
            setting_id: setting.id.clone(),
            value,
            winning_layer,
            winning_source_id,
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

#[cfg(test)]
mod tests {
    use super::{
        resolve_profile, validate_profile, validate_surface, AppSettingsSurface, SettingsProfile,
        EFFECTIVE_SETTINGS_SCHEMA,
    };

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
