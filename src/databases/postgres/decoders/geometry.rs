use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, FromRow, Postgres, postgres::{PgRow, PgValueRef}};

use geozero::wkb;
use geo_types::Geometry as GeoGeometry;

// // TODO: Must pass type of coord
// use geo::CoordNum;
// pub struct GeometryDecoder<T: CoordNum> {
//     pub(crate) value: GeoGeometry<T>
// }

pub type JsonMap = HashMap<String, Value>;

pub struct Geometry {
    pub(crate) value: GeoGeometry
}

impl <'r>sqlx::Decode<'r, Postgres> for Geometry {
    fn decode(value: PgValueRef<'r>) -> std::prelude::v1::Result<Self, sqlx::error::BoxDynError> {
        return Ok(Self {
            value: <wkb::Decode<GeoGeometry<f64>> as Decode<Postgres>>::decode(value)
                .unwrap()
                .geometry
                .unwrap()
        });
    }
}

impl <'r>FromRow<'r, PgRow> for Geometry {
    fn from_row(_row: &'r PgRow) -> std::prelude::v1::Result<Self, sqlx::Error> {
        todo!()
    }
}

impl <'de>Deserialize<'de> for Geometry {
    fn deserialize<D>(_deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }
}

impl Serialize for Geometry {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        match &self.value {
            GeoGeometry::Point(point) => {
                return serializer.collect_map({
                    let mut map = JsonMap::new();

                    map.insert(String::from("lat"), point.y().into());
                    map.insert(String::from("lng"), point.x().into());

                    map
                });
            },
            GeoGeometry::Line(_line) => {
                todo!()
            },
            GeoGeometry::LineString(line_string) => {
                return serializer.collect_seq( line_string.points().map(|coord| {
                    let mut map = JsonMap::new();

                    map.insert(String::from("lat"), coord.y().into());
                    map.insert(String::from("lng"), coord.x().into());

                    map
                }));
            },
            GeoGeometry::Polygon(_polygon) => {
                todo!()
            },
            GeoGeometry::MultiPoint(_multi_point) => {
                todo!()
            },
            GeoGeometry::MultiLineString(_multi_line_string) => {
                todo!()
            },
            GeoGeometry::MultiPolygon(_multi_polygon) => {
                todo!()
            },
            GeoGeometry::GeometryCollection(_geometry_collection) => {
                todo!()
            },
            GeoGeometry::Rect(_rect) => {
                todo!()
            },
            GeoGeometry::Triangle(_triangle) =>{
                todo!()
            },
        };
    }
}

impl <DB: sqlx::Database>sqlx::Type<DB> for Geometry {
    fn compatible(_ty: &<DB as sqlx::Database>::TypeInfo) -> bool {
        return true;
    }
    
    fn type_info() -> <DB as sqlx::Database>::TypeInfo {
        todo!()
    }
}