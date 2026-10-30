//! CSL citation-cluster processing backed by the pinned citeproc-rs engine.

use std::str::FromStr;

use citeproc::csl::LocatorType;
use citeproc::io::{CiteMode, Locator, Locators, NumberLike};
use citeproc::prelude::{Cite, ClusterPosition, InitOptions, Processor, SupportedFormat};

use super::bibtex::Bibliography;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationMode {
    AuthorOnly,
    SuppressAuthor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationItem {
    pub key: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub locator: Option<String>,
    pub locator_label: Option<String>,
    pub mode: Option<CitationMode>,
}

impl CitationItem {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            prefix: None,
            suffix: None,
            locator: None,
            locator_label: None,
            mode: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationCluster {
    pub id: String,
    pub citations: Vec<CitationItem>,
    pub note_number: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedCluster {
    pub id: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedBibliographyEntry {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CslRender {
    pub clusters: Vec<RenderedCluster>,
    pub bibliography: Vec<RenderedBibliographyEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CslError {
    #[error("invalid CSL style: {0}")]
    Style(String),
    #[error("citation cluster {cluster_id} refers to missing key {key}")]
    MissingReference { cluster_id: String, key: String },
    #[error("citation cluster id is empty or duplicated: {0}")]
    InvalidClusterId(String),
    #[error("unsupported CSL locator label: {0}")]
    InvalidLocator(String),
    #[error("citeproc returned no output for cluster {0}")]
    MissingClusterOutput(String),
    #[error("failed to order citation clusters: {0}")]
    ClusterOrder(String),
}

pub fn render(
    bibliography: &Bibliography,
    style: &str,
    clusters: &[CitationCluster],
    format: SupportedFormat,
) -> Result<CslRender, CslError> {
    use std::collections::HashSet;

    let mut cluster_ids = HashSet::with_capacity(clusters.len());
    for cluster in clusters {
        if cluster.id.trim().is_empty() || !cluster_ids.insert(cluster.id.as_str()) {
            return Err(CslError::InvalidClusterId(cluster.id.clone()));
        }
        for citation in &cluster.citations {
            if bibliography.get(&citation.key).is_none() {
                return Err(CslError::MissingReference {
                    cluster_id: cluster.id.clone(),
                    key: citation.key.clone(),
                });
            }
            if citation.locator.is_some()
                && let Some(label) = citation.locator_label.as_deref()
            {
                LocatorType::from_str(label)
                    .map_err(|_| CslError::InvalidLocator(label.to_string()))?;
            }
        }
    }

    let mut processor = Processor::new(InitOptions {
        style,
        format,
        ..Default::default()
    })
    .map_err(|error| CslError::Style(error.to_string()))?;
    processor.reset_references(
        bibliography
            .records()
            .iter()
            .map(|record| record.to_csl_reference())
            .collect(),
    );

    let mut positions = Vec::with_capacity(clusters.len());
    for cluster in clusters {
        let cluster_id = processor.cluster_id(&cluster.id);
        let mut cites = Vec::with_capacity(cluster.citations.len());
        for citation in &cluster.citations {
            let mut cite = Cite::basic(citation.key.as_str());
            cite.prefix = citation.prefix.clone().map(Into::into);
            cite.suffix = citation.suffix.clone().map(Into::into);
            cite.mode = citation.mode.map(|mode| match mode {
                CitationMode::AuthorOnly => CiteMode::AuthorOnly,
                CitationMode::SuppressAuthor => CiteMode::SuppressAuthor,
            });
            if let Some(locator) = citation.locator.as_ref() {
                let loc_type = citation
                    .locator_label
                    .as_deref()
                    .map(LocatorType::from_str)
                    .transpose()
                    .map_err(|_| {
                        CslError::InvalidLocator(citation.locator_label.clone().unwrap_or_default())
                    })?
                    .unwrap_or_default();
                cite.locators = Some(Locators::Single(Locator {
                    locator: NumberLike::Str(locator.clone().into()),
                    loc_type,
                }));
            }
            cites.push(cite);
        }
        processor.insert_cites(cluster_id, &cites);
        positions.push(match cluster.note_number {
            Some(note_number) => ClusterPosition::note(cluster_id, note_number),
            None => ClusterPosition::in_text(cluster_id),
        });
    }

    if !positions.is_empty() {
        processor
            .set_cluster_order(&positions)
            .map_err(|error| CslError::ClusterOrder(error.to_string()))?;
    }

    let clusters = clusters
        .iter()
        .map(|cluster| -> Result<_, CslError> {
            let output = processor
                .get_cluster_str(&cluster.id)
                .map(|value| value.as_str().to_string())
                .ok_or_else(|| CslError::MissingClusterOutput(cluster.id.clone()))?;
            Ok(RenderedCluster {
                id: cluster.id.clone(),
                value: output,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let bibliography = processor
        .get_bibliography()
        .into_iter()
        .map(|entry| RenderedBibliographyEntry {
            key: entry.id.as_ref().to_string(),
            value: entry.value.as_str().to_string(),
        })
        .collect();

    Ok(CslRender {
        clusters,
        bibliography,
    })
}

#[cfg(test)]
mod tests {
    use citeproc::prelude::SupportedFormat;

    use super::{CitationCluster, CitationItem, CitationMode, CslError, render};
    use crate::extensions::bibtex::Bibliography;

    const STYLE: &str = r#"
        <style version="1.0" class="in-text">
          <info>
            <title>Bibliography adapter test</title>
            <id>https://example.test/styles/bibliography-adapter</id>
            <link href="https://example.test/styles/bibliography-adapter" rel="self"/>
                        <updated>2024-01-01T00:00:00+00:00</updated>
          </info>
          <citation>
            <layout prefix="(" suffix=")" delimiter="; ">
              <group delimiter=", ">
                <text variable="title"/>
                <text variable="locator"/>
              </group>
            </layout>
          </citation>
          <bibliography>
            <layout><text variable="title"/></layout>
          </bibliography>
        </style>
    "#;

    const AUTHOR_YEAR_STYLE: &str = r#"
                <style version="1.0" class="in-text">
                    <info>
                        <title>Author year disambiguation test</title>
                        <id>https://example.test/styles/author-year-test</id>
                        <link href="https://example.test/styles/author-year-test" rel="self"/>
                        <updated>2024-01-01T00:00:00+00:00</updated>
                    </info>
                    <citation disambiguate-add-year-suffix="true">
                        <layout delimiter="; ">
                            <group delimiter=" ">
                                <names variable="author"><name form="short"/></names>
                                <group delimiter="">
                                    <date variable="issued"><date-part name="year"/></date>
                                    <text variable="year-suffix"/>
                                </group>
                            </group>
                        </layout>
                    </citation>
                    <bibliography>
                        <sort><key variable="title"/></sort>
                        <layout><text variable="title"/></layout>
                    </bibliography>
                </style>
        "#;

    fn bibliography() -> Bibliography {
        Bibliography::parse(
            r#"
                @book{alpha, title={Alpha}, author={Ada Lovelace}, year={1843}}
                @book{beta, title={Beta}, author={Grace Hopper}, year={1952}}
            "#,
        )
        .unwrap()
    }

    #[test]
    fn renders_ordered_multi_citation_clusters_and_bibliography() {
        let mut first = CitationItem::new("alpha");
        first.prefix = Some("see ".into());
        first.locator = Some("7".into());
        first.locator_label = Some("page".into());
        first.mode = Some(CitationMode::SuppressAuthor);

        let result = render(
            &bibliography(),
            STYLE,
            &[
                CitationCluster {
                    id: "chapter:0".into(),
                    citations: vec![first, CitationItem::new("beta")],
                    note_number: None,
                },
                CitationCluster {
                    id: "chapter:1".into(),
                    citations: vec![CitationItem::new("beta")],
                    note_number: None,
                },
            ],
            SupportedFormat::Plain,
        )
        .unwrap();

        assert_eq!(result.clusters.len(), 2);
        assert_eq!(result.clusters[0].id, "chapter:0");
        assert!(result.clusters[0].value.contains("see Alpha, 7; Beta"));
        assert_eq!(result.clusters[1].value, "(Beta)");
        assert_eq!(
            result
                .bibliography
                .iter()
                .map(|entry| entry.key.as_str())
                .collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
    }

    #[test]
    fn reports_missing_keys_and_invalid_styles() {
        let missing = render(
            &bibliography(),
            STYLE,
            &[CitationCluster {
                id: "chapter:0".into(),
                citations: vec![CitationItem::new("missing")],
                note_number: None,
            }],
            SupportedFormat::Plain,
        )
        .unwrap_err();
        assert!(matches!(missing, CslError::MissingReference { .. }));

        let invalid_style =
            render(&bibliography(), "<style", &[], SupportedFormat::Plain).unwrap_err();
        assert!(matches!(invalid_style, CslError::Style(_)));

        let invalid_locator = render(
            &bibliography(),
            STYLE,
            &[CitationCluster {
                id: "chapter:0".into(),
                citations: vec![CitationItem {
                    key: "alpha".into(),
                    locator: Some("7".into()),
                    locator_label: Some("not-a-locator".into()),
                    ..CitationItem::new("alpha")
                }],
                note_number: None,
            }],
            SupportedFormat::Plain,
        )
        .unwrap_err();
        assert!(matches!(invalid_locator, CslError::InvalidLocator(_)));
    }

    #[test]
    fn disambiguates_same_author_and_year_across_clusters() {
        let bibliography = Bibliography::parse(
            r#"
                @article{first, author={Ada Lovelace}, title={First}, year={1843}}
                @article{second, author={Ada Lovelace}, title={Second}, year={1843}}
            "#,
        )
        .unwrap();
        let result = render(
            &bibliography,
            AUTHOR_YEAR_STYLE,
            &[
                CitationCluster {
                    id: "intro:0".into(),
                    citations: vec![CitationItem::new("first")],
                    note_number: None,
                },
                CitationCluster {
                    id: "intro:1".into(),
                    citations: vec![CitationItem::new("second")],
                    note_number: None,
                },
            ],
            SupportedFormat::Plain,
        )
        .unwrap();

        assert!(
            result.clusters[0].value.contains("1843a"),
            "rendered clusters: {:?}",
            result.clusters
        );
        assert!(
            result.clusters[1].value.contains("1843b"),
            "rendered clusters: {:?}",
            result.clusters
        );
    }
}
