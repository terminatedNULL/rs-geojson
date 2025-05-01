mod geojson {
    use std::cmp::PartialEq;
    use std::collections::HashMap;
    use std::fmt;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use uuid::Uuid;

    #[derive(Debug)]
    pub struct GeoJSON {
        features: Feature
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    pub enum FeatureType {
        Point,
        MultiPoint,
        LineString,
        MultiLineString,
        Polygon,
        MultiPolygon,
        GeometryCollection,
        Feature,
    }

    impl fmt::Display for FeatureType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                FeatureType::Feature => write!(f, "Feature"),
                FeatureType::Point => write!(f, "Point"),
                FeatureType::MultiPoint => write!(f, "MultiPoint"),
                FeatureType::LineString => write!(f, "LineString"),
                FeatureType::MultiLineString => write!(f, "MultiLineString"),
                FeatureType::Polygon => write!(f, "Polygon"),
                FeatureType::MultiPolygon => write!(f, "MultiPolygon"),
                FeatureType::GeometryCollection => write!(f, "GeometryCollection"),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Coordinates {
        Point(Vec<f64>),
        MultiPoint(Vec<Vec<f64>>),
        LineString(Vec<Vec<f64>>),
        MultiLineString(Vec<Vec<Vec<f64>>>),
        Polygon(Vec<Vec<Vec<f64>>>),
        MultiPolygon(Vec<Vec<Vec<Vec<f64>>>>),
    }

    impl Coordinates {
        pub fn empty_for(geometry_type: FeatureType) -> Self {
            match geometry_type {
                FeatureType::Point => Coordinates::Point(Vec::new()),
                FeatureType::MultiPoint => Coordinates::MultiPoint(Vec::new()),
                FeatureType::LineString => Coordinates::LineString(Vec::new()),
                FeatureType::MultiLineString => Coordinates::MultiLineString(Vec::new()),
                FeatureType::Polygon => Coordinates::Polygon(Vec::new()),
                FeatureType::MultiPolygon => Coordinates::MultiPolygon(Vec::new()),
                _ => panic!("No valid coordinates for type {:?}", geometry_type),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Geometry {
        #[serde(rename = "type")]
        pub geometry_type: FeatureType,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub coordinates: Option<Coordinates>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub geometries: Option<Vec<Geometry>>,
    }

    impl Geometry {
        pub fn new(geometry_type: FeatureType) -> Self {
            match geometry_type {
                FeatureType::GeometryCollection => Geometry {
                    geometry_type,
                    coordinates: None,
                    geometries: Some(Vec::new()),
                },
                FeatureType::Feature => panic!("Cannot use FeatureType::Feature as geometry type."),
                _ => Geometry {
                    geometry_type: geometry_type.clone(),
                    coordinates: Some(Coordinates::empty_for(geometry_type)),
                    geometries: None,
                }
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Feature {
        #[serde(rename = "type")]
        pub feature_type: FeatureType,

        #[serde(skip)]
        pub id: Uuid,

        pub properties: HashMap<String, Value>,
        pub geometry: Geometry,

        #[serde(skip_serializing_if="Option::is_none")]
        pub marker: Option<String>,
    }

    impl Feature {
        pub fn new(geometry_type: FeatureType) -> Self {
            let coordinates = match geometry_type {
                FeatureType::Point => Coordinates::Point(Vec::new()),
                FeatureType::MultiPoint => Coordinates::MultiPoint(Vec::new()),
                FeatureType::LineString => Coordinates::LineString(Vec::new()),
                FeatureType::MultiLineString => Coordinates::MultiLineString(Vec::new()),
                FeatureType::Polygon => Coordinates::Polygon(Vec::new()),
                FeatureType::MultiPolygon => Coordinates::MultiPolygon(Vec::new()),
                FeatureType::GeometryCollection => todo!(),
                FeatureType::Feature => unreachable!(),
            };

            Feature {
                feature_type: FeatureType::Feature,
                id: Uuid::new_v4(),
                properties: HashMap::new(),
                geometry: Geometry {
                    geometry_type: geometry_type.clone(),
                    coordinates: Some(coordinates),
                    geometries: None,
                },
                marker: None,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn blank_feature_display() {
            let feature = Feature::new(FeatureType::Point);
            println!("{:?}", feature);
        }
    }
}