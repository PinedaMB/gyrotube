use serde::{Deserialize, Serialize};

const GITHUB_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/PinedaMB/gyrotube/releases/latest";
const GITHUB_RELEASES_PAGE: &str = "https://github.com/PinedaMB/gyrotube/releases/latest";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub update_available: bool,
    pub can_auto_install: bool,
    pub release_url: String,
    pub download_url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: Option<String>,
    body: Option<String>,
    #[serde(default)]
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

/// Parses a `v1.2.3` or `1.2.3` semver string into `(major, minor, patch)`.
pub fn parse_semver(raw: &str) -> Option<(u64, u64, u64)> {
    let clean = raw.trim().trim_start_matches(['v', 'V']);
    let core = clean.split(['-', '+']).next()?;
    let mut parts = core.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next().unwrap_or("0").parse::<u64>().ok()?;
    let patch = parts.next().unwrap_or("0").parse::<u64>().ok()?;
    Some((major, minor, patch))
}

/// Returns true if `latest` is strictly newer than `current`.
pub fn is_newer_version(current: &str, latest: &str) -> bool {
    match (parse_semver(current), parse_semver(latest)) {
        (Some(c), Some(l)) => l > c,
        _ => false,
    }
}

fn pick_platform_asset(assets: &[GithubAsset]) -> Option<String> {
    let find_matching = |pred: &dyn Fn(&str) -> bool| -> Option<String> {
        assets
            .iter()
            .find(|a| pred(&a.name.to_ascii_lowercase()))
            .map(|a| a.browser_download_url.clone())
    };

    #[cfg(target_os = "android")]
    {
        return find_matching(&|n| n.ends_with(".apk"));
    }

    #[cfg(target_os = "windows")]
    {
        return find_matching(&|n| n.ends_with("-setup.exe") || n.ends_with(".exe"))
            .or_else(|| find_matching(&|n| n.ends_with(".msi")));
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            return find_matching(&|n| {
                n.ends_with(".dmg") && (n.contains("apple-silicon") || n.contains("aarch64"))
            })
            .or_else(|| find_matching(&|n| n.ends_with(".dmg")));
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            return find_matching(&|n| {
                n.ends_with(".dmg") && (n.contains("intel") || n.contains("x64") || n.contains("x86_64"))
            })
            .or_else(|| find_matching(&|n| n.ends_with(".dmg")));
        }
    }

    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("APPIMAGE").is_some()
            && let Some(url) = find_matching(&|n| n.ends_with(".appimage"))
        {
            return Some(url);
        }
        if (std::path::Path::new("/etc/fedora-release").exists()
            || std::path::Path::new("/etc/redhat-release").exists())
            && let Some(url) = find_matching(&|n| n.ends_with(".rpm"))
        {
            return Some(url);
        }
        if std::path::Path::new("/etc/debian_version").exists()
            && let Some(url) = find_matching(&|n| n.ends_with(".deb"))
        {
            return Some(url);
        }
        find_matching(&|n| n.ends_with(".appimage"))
            .or_else(|| find_matching(&|n| n.ends_with(".rpm")))
            .or_else(|| find_matching(&|n| n.ends_with(".deb")))
    }

    #[cfg(not(any(
        target_os = "android",
        target_os = "windows",
        target_os = "macos",
        target_os = "linux"
    )))]
    {
        let _ = find_matching;
        None
    }
}

/// Checks GitHub Releases (`releases/latest`) for a newer version of GyroTube.
pub async fn check_github_release(current_version: &str) -> Result<UpdateCheckResult, String> {
    let client = crate::tls::http_client_builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let resp = client
        .get(GITHUB_LATEST_RELEASE_URL)
        .header(
            reqwest::header::USER_AGENT,
            format!("GyroTube/{current_version}"),
        )
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("Failed to query GitHub Releases: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub Releases returned HTTP {}", resp.status()));
    }

    let release: GithubRelease = resp
        .json()
        .await
        .map_err(|e| format!("Invalid GitHub Releases JSON: {e}"))?;

    let latest_clean = release
        .tag_name
        .trim()
        .trim_start_matches(['v', 'V'])
        .to_string();
    let update_available = is_newer_version(current_version, &latest_clean);
    let download_url = pick_platform_asset(&release.assets);
    let release_url = release
        .html_url
        .filter(|u| u.starts_with("https://"))
        .unwrap_or_else(|| GITHUB_RELEASES_PAGE.to_string());

    #[cfg(target_os = "android")]
    let can_auto_install = download_url.is_some();
    #[cfg(not(target_os = "android"))]
    let can_auto_install = false;

    Ok(UpdateCheckResult {
        current_version: current_version.to_string(),
        latest_version: Some(latest_clean),
        update_available,
        can_auto_install,
        release_url,
        download_url,
        notes: release.body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_semver_versions_correctly() {
        assert!(is_newer_version("1.0.0", "1.0.1"));
        assert!(is_newer_version("1.0.0", "v1.1.0"));
        assert!(is_newer_version("1.2.9", "2.0.0"));
        assert!(!is_newer_version("1.0.0", "1.0.0"));
        assert!(!is_newer_version("1.0.0", "v1.0.0"));
        assert!(!is_newer_version("1.2.0", "1.1.9"));
    }

    #[test]
    fn parses_semver_with_prerelease_suffix() {
        assert_eq!(parse_semver("v1.2.3-beta.1"), Some((1, 2, 3)));
        assert_eq!(parse_semver("2.0"), Some((2, 0, 0)));
        assert_eq!(parse_semver("invalid"), None);
    }
}
