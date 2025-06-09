use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use owo_colors::OwoColorize;
use specta_typescript::primitives::export;
use specta_typescript::{BigIntExportBehavior, Typescript};

use crate::routes::RouteInfo;

const TYPES_PATH_VAR: &str = "TYPES_PATH";

pub fn export_types(debug_location: bool, routes: Vec<RouteInfo>) -> Result<()> {
  let types_path = env::var(TYPES_PATH_VAR)?;

  if types_path.is_empty() {
    bail!("{} environment variable is not set", TYPES_PATH_VAR);
  }

  let types_path = Path::new(&types_path).canonicalize()?;

  println!("Exporting types to: {}", display_path(&types_path));

  let types = specta::export();

  let config = Typescript::new().bigint(BigIntExportBehavior::Number);

  let mut type_outputs = types
    .into_unsorted_iter()
    .map(|ty| {
      (
        ty.name().to_string(),
        ty.clone(),
        display_path_str(ty.module_path().as_ref()),
      )
    })
    .collect::<Vec<_>>();

  // sort by name, then by location
  // otherwise the position in the source file determines order of the types
  type_outputs.sort_by(|(a, ..), (b, ..)| a.cmp(b));

  // location comparison should strip the line/column number
  // `src/routes/mod.rs:12:30` -> `src/routes/mod.rs`
  type_outputs.sort_by(|(_, _, a), (_, _, b)| strip_location_position(a).cmp(strip_location_position(b)));

  let longest_name = type_outputs
    .iter()
    .map(|(name, ..)| name.len())
    .max()
    .unwrap_or_default();

  let type_outputs = type_outputs
    .into_iter()
    .map(|(name, ty, location)| {
      let name = format!("{name:>width$}", width = longest_name);

      println!("Exporting type: {} {}", name.bright_green(), location.bright_black());

      let output = export(&config, &types, &ty)?;

      Ok((output, location))
    })
    .collect::<Result<Vec<_>>>()?;

  let mut output = String::new();

  if !config.header.is_empty() {
    writeln!(output, "{}", config.header)?;
  }

  if !config.framework_header.is_empty() {
    writeln!(output, "{}", config.framework_header)?;
  }

  for (type_output, location) in type_outputs {
    writeln!(output)?;

    if debug_location {
      writeln!(output, "/* {location} */")?;
    }

    writeln!(output, "{type_output}")?;
  }

  let paths = routes
    .into_iter()
    .map(|route| {
      let path = route.path.trim_start_matches('/').trim_end_matches('/').to_string();
      (route, path)
    })
    .collect::<Vec<_>>();

  let mut paths_map = HashMap::new();
  for (route, path) in paths {
    let methods = paths_map.entry(path.clone()).or_insert_with(HashSet::new);
    methods.insert(route.method);
  }

  let mut paths = paths_map
    .into_iter()
    .map(|(path, methods)| {
      let methods = methods.into_iter().map(|m| m.as_str().to_string()).collect::<Vec<_>>();
      (methods, path)
    })
    .collect::<Vec<_>>();

  // sort by path primarily, then by method
  paths.sort_unstable();

  // generate function for each unique route path

  writeln!(output)?;

  let mut functions = paths
    .into_iter()
    .filter_map(|(methods, path)| {
      let last_part_param: bool = path_parts(&path).last().map(is_param).unwrap_or(false);

      // if the last part is a param, unpluralize all parts
      // otherwise, unpluralize all parts except the last one
      let mut parts = path_not_params(&path)
        .iter()
        .map(|s| s.chars().filter(|c| c.is_alphanumeric()).collect())
        .collect::<Vec<String>>();

      // / route
      if parts.is_empty() {
        return None;
      }

      let last = parts.pop();
      let last = if last_part_param { last.map(unpluralize) } else { last };

      let parts = parts.into_iter().map(unpluralize).chain(last).collect::<Vec<_>>();

      let name = camel_case(&parts);

      if name.is_empty() {
        return None;
      }

      let params = path_params(&path)
        .iter()
        .map(|m| camel_case(&m.split('_').collect::<Vec<_>>()))
        .map(|param| format!("{param}: string"))
        .collect::<Vec<_>>()
        .join(", ");

      let string = path_parts(&path)
        .map(|s| {
          if is_param(s) {
            let param = s.trim_start_matches('{').trim_end_matches('}');
            let param = camel_case(&param.split('_').collect::<Vec<_>>());
            format!("${{{param}}}")
          } else {
            s.to_string()
          }
        })
        .collect::<Vec<_>>()
        .join("/");

      if params.is_empty() {
        Some((name, methods, path, format!("() => '{string}' as const")))
      } else {
        Some((name, methods, path, format!("({params}) => `{string}` as const")))
      }
    })
    .collect::<Vec<_>>();

  functions.sort_by(|(a, ..), (b, ..)| a.cmp(b));

  writeln!(output, "export const Routes = {{")?;
  for (name, methods, path, function) in functions {
    writeln!(output)?;
    writeln!(output, "  /**")?;
    writeln!(output, "   * Route for:")?;

    let longest_method = methods.iter().map(|m| m.len()).max().unwrap_or_default();

    for method in methods {
      let path = path.replace("_id", ".id");
      writeln!(output, "   * - {method:>width$} `{path}`", width = longest_method)?;
    }

    writeln!(output, "   */")?;
    writeln!(output, "  {name}: {function},")?;
  }
  writeln!(output, "}};")?;
  writeln!(output)?;
  writeln!(output, "Object.freeze(Routes);")?;
  writeln!(output)?;

  // attempt to format by piping output into prettier

  // find yarn path
  let yarn = find_yarn_path()?;

  // set cwd to the directory of the types_path

  let cwd = types_path
    .parent()
    .context(anyhow::anyhow!(
      "failed to get parent directory of types_path: {}",
      display_path(&types_path)
    ))?
    .canonicalize()?;

  let cwd = cwd.to_string_lossy();

  #[cfg(target_os = "windows")]
  let cwd = display_path(&*cwd);

  println!("output > yarn prettier");

  let mut child = Command::new(yarn)
    .current_dir(cwd)
    .stdout(Stdio::piped())
    .stderr(Stdio::inherit())
    .stdin(Stdio::piped())
    .arg("prettier")
    .arg("--parser")
    .arg("typescript")
    .spawn()?;

  let stdout = child.stdout.take().context("failed to open stdout")?;
  let mut reader = BufReader::new(stdout);

  let stdin = child.stdin.take().context("failed to open stdin")?;
  let mut writer = BufWriter::new(stdin);
  writer
    .write_all(output.as_bytes())
    .context("failed to write to stdin")?;
  writer.flush().context("failed to flush stdin")?;
  drop(writer);

  let status = child.wait()?;

  if !status.success() {
    bail!("`output > yarn prettier` failed");
  }

  let mut formatted_output = String::new();
  reader
    .read_to_string(&mut formatted_output)
    .context("failed to read from stdout")?;

  // write to file
  let mut file = File::options()
    .write(true)
    .create(true)
    .truncate(true)
    .open(&types_path)?;
  file
    .write_all(formatted_output.as_bytes())
    .context("failed to write to file")?;
  file.flush().context("failed to flush file")?;
  drop(file);

  Ok(())
}

fn display_path<P: AsRef<Path>>(path: P) -> String {
  display_path_str(&path.as_ref().display().to_string())
}

fn display_path_str(path: &str) -> String {
  path.replace("\\\\?\\", "").replace("\\", "/")
}

#[cfg(windows)]
fn find_yarn_path() -> Result<String> {
  let output = Command::new("where").arg("yarn").output()?;

  if !output.status.success() {
    bail!("yarn not found");
  }

  let paths = String::from_utf8(output.stdout)?;
  let paths = paths.lines().collect::<Vec<_>>();

  if paths.is_empty() {
    bail!("yarn not found");
  }

  // if a path has .cmd, use that
  // otherwise use first

  let path = paths
    .iter()
    .find(|path| path.to_lowercase().ends_with(".cmd"))
    .unwrap_or(&paths[0]);

  let path = path.trim();
  let path = path.replace("\\\\?\\", "").replace("\\", "/");
  let path = path.to_string();
  println!("yarn path: {}", path);
  Ok(path)
}

#[cfg(not(windows))]
fn yarn_path() -> Result<String> {
  let output = Command::new("which").arg("yarn").output()?;

  if !output.status.success() {
    bail!("yarn not found");
  }

  let path = String::from_utf8(output.stdout)?;
  let path = path.trim();
  let path = path.to_string();
  println!("yarn path: {}", path);
  Ok(path)
}

fn camel_case<S: AsRef<str>>(s: &[S]) -> String {
  let mut result = String::new();

  for (i, s) in s.iter().enumerate() {
    let s = s.as_ref();
    if i == 0 {
      result.push_str(&s.to_lowercase());
    } else {
      let mut chars = s.chars();
      if let Some(first) = chars.next() {
        result.push(first.to_uppercase().next().unwrap());
      }
      result.push_str(&chars.as_str().to_lowercase());
    }
  }

  result
}

fn path_parts(path: &str) -> impl Iterator<Item = &str> {
  path.split('/').filter(|s| !s.is_empty())
}

fn is_param(s: &str) -> bool {
  s.starts_with('{') && s.ends_with('}')
}

fn path_params(path: &str) -> Vec<&str> {
  path_parts(path)
    .filter_map(|s| if is_param(s) { Some(&s[1..s.len() - 1]) } else { None })
    .collect()
}

fn path_not_params(path: &str) -> Vec<&str> {
  path_parts(path)
    .filter(|s| !s.starts_with('{') || !s.ends_with('}'))
    .collect()
}

fn unpluralize(s: String) -> String {
  if let Some(stripped) = s.strip_suffix('s') {
    stripped.to_string()
  } else {
    s
  }
}

fn strip_location_position(location: &str) -> &str {
  let mut parts = location.split(':');
  parts.next().unwrap_or(location)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_path_params() {
    let path = "/api/v1/users/{user_id}/posts/{post_id}";
    let params = path_params(path);
    assert_eq!(params, vec!["user_id", "post_id"]);
  }

  #[test]
  fn test_path_not_params() {
    let path = "/api/v1/users/{user_id}/posts/{post_id}";
    let not_params = path_not_params(path);
    assert_eq!(not_params, vec!["api", "v1", "users", "posts"]);
  }

  #[test]
  fn test_camel_case() {
    let input = vec!["hello", "world"];
    let output = camel_case(&input);
    assert_eq!(output, "helloWorld");
  }
}
