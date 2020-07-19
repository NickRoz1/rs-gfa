#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header {
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OptionalFieldValue {
    PrintableChar(char),
    SignedInt(i64),
    Float(f32),
    PrintableString(String),
    JSON(String),
    ByteArray(Vec<u32>),
    IntArray(Vec<i64>),
    FloatArray(Vec<f32>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OptionalField {
    pub tag: String,
    pub content: OptionalFieldValue,
}

impl OptionalField {
    pub fn new(tag: &str, content: OptionalFieldValue) -> Self {
        OptionalField {
            tag: tag.to_string(),
            content,
        }
    }
}

impl std::fmt::Display for OptionalField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OptionalFieldValue::*;
        write!(f, "{}:", self.tag)?;
        match &self.content {
            PrintableChar(c) => write!(f, "A:{}", c),
            SignedInt(i) => write!(f, "i:{}", i),
            Float(d) => write!(f, "f:{}", d),
            PrintableString(s) => write!(f, "Z:{}", s),
            JSON(s) => write!(f, "J:{}", s),
            ByteArray(a) => {
                let mut array_str = String::new();
                for x in a {
                    array_str.push(std::char::from_digit(*x, 16).unwrap())
                }
                write!(f, "H:{}", array_str)
            }
            IntArray(a) => {
                let mut array_str = String::new();
                for (i, x) in a.into_iter().enumerate() {
                    if i > 0 {
                        array_str.push_str(",");
                    }
                    array_str.push_str(&x.to_string());
                }
                write!(f, "B:I{}", array_str)
            }
            FloatArray(a) => {
                let mut array_str = String::new();
                for (i, x) in a.into_iter().enumerate() {
                    if i > 0 {
                        array_str.push_str(",");
                    }
                    array_str.push_str(&x.to_string());
                }
                write!(f, "B:f{}", array_str)
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Segment {
    pub name: String,
    pub sequence: String,
    pub segment_length: Option<i64>,
    pub read_count: Option<i64>,
    pub fragment_count: Option<i64>,
    pub kmer_count: Option<i64>,
    pub sha256: Option<Vec<u32>>,
    pub uri: Option<String>,
    pub optional_fields: Vec<OptionalField>,
}

impl Segment {
    pub fn new(name: &str, sequence: &str) -> Self {
        Segment {
            name: name.to_string(),
            sequence: sequence.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Orientation {
    Forward,
    Backward,
}

// It makes sense for forward to be the default
impl std::default::Default for Orientation {
    fn default() -> Orientation {
        Orientation::Forward
    }
}

impl std::str::FromStr for Orientation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Forward),
            "-" => Ok(Self::Backward),
            _ => Err("Could not parse orientation (was not + or -)"),
        }
    }
}

impl Orientation {
    pub fn is_reverse(&self) -> bool {
        match self {
            Self::Forward => false,
            Self::Backward => true,
        }
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = match self {
            Self::Forward => '+',
            Self::Backward => '-',
        };
        write!(f, "{}", sym)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Link {
    pub from_segment: String,
    pub from_orient: Orientation,
    pub to_segment: String,
    pub to_orient: Orientation,
    pub overlap: String,
    pub map_quality: Option<i64>,
    pub num_mismatches: Option<i64>,
    pub read_count: Option<i64>,
    pub fragment_count: Option<i64>,
    pub kmer_count: Option<i64>,
    pub edge_id: Option<String>,
    pub optional_fields: Vec<OptionalField>,
}

impl Link {
    pub fn new(
        from_segment: &str,
        from_orient: Orientation,
        to_segment: &str,
        to_orient: Orientation,
        overlap: &str,
    ) -> Link {
        Link {
            from_segment: from_segment.to_string(),
            from_orient,
            to_segment: to_segment.to_string(),
            to_orient,
            overlap: overlap.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Containment {
    pub container_name: String,
    pub container_orient: Orientation,
    pub contained_name: String,
    pub contained_orient: Orientation,
    pub pos: usize,
    pub overlap: String,
    pub read_coverage: Option<i64>,
    pub num_mismatches: Option<i64>,
    pub edge_id: Option<String>,
    pub optional_fields: Vec<OptionalField>,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Path {
    pub path_name: String,
    pub segment_names: Vec<(String, Orientation)>,
    pub overlaps: Vec<String>,
    pub optional_fields: Vec<OptionalField>,
}

impl Path {
    pub fn new(
        path_name: &str,
        seg_names: Vec<&str>,
        overlaps: Vec<String>,
    ) -> Path {
        let segment_names = seg_names
            .iter()
            .map(|s| {
                let s: &str = s;
                let (n, o) = s.split_at(s.len() - 1);
                let name = n.to_string();
                let orientation = o.parse().unwrap();
                (name, orientation)
            })
            .collect();

        Path {
            path_name: path_name.to_string(),
            segment_names,
            overlaps,
            optional_fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line {
    Header(Header),
    Segment(Segment),
    Link(Link),
    Containment(Containment),
    Path(Path),
    Comment,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GFAParsingConfig {
    pub segments: bool,
    pub links: bool,
    pub containments: bool,
    pub paths: bool,
}

impl GFAParsingConfig {
    pub fn none() -> Self {
        GFAParsingConfig {
            segments: false,
            links: false,
            containments: false,
            paths: false,
        }
    }

    pub fn all() -> Self {
        GFAParsingConfig {
            segments: true,
            links: true,
            containments: true,
            paths: true,
        }
    }
}

// struct to hold the results of parsing a file; not actually a graph
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA {
    pub version: Option<String>,
    pub segments: Vec<Segment>,
    pub links: Vec<Link>,
    pub containments: Vec<Containment>,
    pub paths: Vec<Path>,
}

impl GFA {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_path() {
        let name = "path1";
        let seg_names = vec!["1+", "2-", "13-", "60+"];
        let overlaps: Vec<_> = vec!["8M", "10M", "0M", "2M"]
            .into_iter()
            .map(String::from)
            .collect();

        let path_expected = Path {
            path_name: name.to_string(),
            segment_names: vec![
                ("1".to_string(), Orientation::Forward),
                ("2".to_string(), Orientation::Backward),
                ("13".to_string(), Orientation::Backward),
                ("60".to_string(), Orientation::Forward),
            ],
            overlaps: overlaps.clone(),
            optional_fields: Vec::new(),
        };

        let path = Path::new(name, seg_names, overlaps);

        assert_eq!(path, path_expected);
    }
}
