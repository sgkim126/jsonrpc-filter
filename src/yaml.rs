// Copyright 2019 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use yaml_rust::Yaml;

pub fn parse_yaml(yamls: Vec<Yaml>) -> Vec<Vec<String>> {
    parse_yaml_impl(Yaml::Array(yamls), &[])
}

fn parse_yaml_impl(yaml: Yaml, current: &[String]) -> Vec<Vec<String>> {
    match yaml {
        Yaml::Hash(yamls) => {
            let mut result = vec![];
            for (key, yaml) in yamls {
                let key = key.as_str().unwrap().to_string();
                let mut next = current.to_vec();
                next.push(key);
                result.append(&mut parse_yaml_impl(yaml, &next));
            }
            result
        }
        Yaml::Array(yamls) => {
            let mut result = vec![];
            for yaml in yamls {
                result.append(&mut parse_yaml_impl(yaml, current));
            }
            result
        }
        Yaml::String(value) => {
            let mut next = current.to_vec();
            next.push(value.clone());
            vec![next]
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use yaml_rust::YamlLoader;

    use super::*;

    #[test]
    fn single_ping() {
        let result = parse_yaml(YamlLoader::load_from_str(r#"ping"#).unwrap());
        assert_eq!(vec![vec!["ping".to_string()]], result);
    }

    #[test]
    fn ping_or_version() {
        let result = parse_yaml(
            YamlLoader::load_from_str(
                r#"
- ping
- version
"#,
            )
            .unwrap(),
        );
        assert_eq!(
            vec![vec!["ping".to_string()], vec!["version".to_string()]],
            result
        );
    }

    #[test]
    fn mempool_and_get() {
        let result = parse_yaml(
            YamlLoader::load_from_str(
                r#"
- mempool_: _get
"#,
            )
            .unwrap(),
        );
        assert_eq!(
            vec![vec!["mempool_".to_string(), "_get".to_string()]],
            result
        );
    }

    #[test]
    fn quoted_mempool_and_get() {
        let result = parse_yaml(
            YamlLoader::load_from_str(
                r#"
- "mempool_": "_get"
"#,
            )
            .unwrap(),
        );
        assert_eq!(
            vec![vec!["mempool_".to_string(), "_get".to_string()]],
            result
        );
    }

    #[test]
    fn quoted_mempool_and_get_or_ping() {
        let result = parse_yaml(
            YamlLoader::load_from_str(
                r#"
- "mempool_": "_get"
- ping
"#,
            )
            .unwrap(),
        );
        assert_eq!(
            vec![
                vec!["mempool_".to_string(), "_get".to_string()],
                vec!["ping".to_string()]
            ],
            result
        );
    }

    #[test]
    fn ping_or_quoted_mempool_and_get() {
        let result = parse_yaml(
            YamlLoader::load_from_str(
                r#"
- ping
- "mempool_": "_get"
"#,
            )
            .unwrap(),
        );
        assert_eq!(
            vec![
                vec!["ping".to_string()],
                vec!["mempool_".to_string(), "_get".to_string()],
            ],
            result
        );
    }

    #[test]
    fn ping_or_mempool_and_pending_or_error_hint() {
        let result = parse_yaml(
            YamlLoader::load_from_str(
                r#"
- ping
- "mempool_":
    - Pending
    - "ErrorHint"
"#,
            )
            .unwrap(),
        );
        assert_eq!(
            vec![
                vec!["ping".to_string()],
                vec!["mempool_".to_string(), "Pending".to_string()],
                vec!["mempool_".to_string(), "ErrorHint".to_string()],
            ],
            result
        );
    }
}
