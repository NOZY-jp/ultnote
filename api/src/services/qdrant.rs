use crate::error::{AppError, Result};
use crate::models::{Memo, MemoType, SearchFilters, SearchResult};
use chrono::{NaiveDate, Utc};
use qdrant_client::qdrant::{
    point_id::PointIdOptions, Condition, CreateCollectionBuilder, DeletePointsBuilder, Distance,
    Filter, GetPointsBuilder, PointId, PointStruct, PointsIdsList, SearchPointsBuilder,
    UpsertPointsBuilder, Value, VectorParamsBuilder,
};
use qdrant_client::Qdrant;
use std::collections::HashMap;
use uuid::Uuid;

const VECTOR_SIZE: u64 = 768;
const COLLECTION_MEMOS: &str = "memos";
const COLLECTION_MEMOS_DEMO: &str = "memos_demo";
const COLLECTION_MEMOS_ARCHIVE: &str = "memos_archive";

#[derive(Clone)]
pub struct QdrantService {
    client: Qdrant,
}

impl QdrantService {
    pub async fn new(url: &str) -> Result<Self> {
        let client = Qdrant::from_url(url)
            .build()
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        Ok(Self { client })
    }

    pub async fn ensure_collections(&self) -> Result<()> {
        for collection in [COLLECTION_MEMOS, COLLECTION_MEMOS_DEMO, COLLECTION_MEMOS_ARCHIVE] {
            self.ensure_collection(collection).await?;
        }
        Ok(())
    }

    async fn ensure_collection(&self, name: &str) -> Result<()> {
        let exists = self
            .client
            .collection_exists(name)
            .await
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        if !exists {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(name)
                        .vectors_config(VectorParamsBuilder::new(VECTOR_SIZE, Distance::Cosine)),
                )
                .await
                .map_err(|e| AppError::Qdrant(e.to_string()))?;
            tracing::info!("Created collection: {}", name);
        }

        Ok(())
    }

    pub async fn insert_memo(&self, memo: &Memo, vector: Vec<f32>, demo: bool) -> Result<()> {
        let collection = if demo {
            COLLECTION_MEMOS_DEMO
        } else {
            COLLECTION_MEMOS
        };

        let payload = self.memo_to_payload(memo);

        let point = PointStruct::new(memo.id.to_string(), vector, payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(collection, vec![point]))
            .await
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        Ok(())
    }

    pub async fn get_memo(&self, id: Uuid, demo: bool) -> Result<Option<Memo>> {
        let collection = if demo {
            COLLECTION_MEMOS_DEMO
        } else {
            COLLECTION_MEMOS
        };

        let point_id: PointId = id.to_string().into();

        let points = self
            .client
            .get_points(
                GetPointsBuilder::new(collection, vec![point_id]).with_payload(true),
            )
            .await
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        let Some(point) = points.result.into_iter().next() else {
            return Ok(None);
        };

        self.payload_to_memo(id, &point.payload).map(Some)
    }

    pub async fn update_memo(
        &self,
        memo: &Memo,
        vector: Option<Vec<f32>>,
        demo: bool,
    ) -> Result<()> {
        let existing_point = self.get_memo_point(memo.id, demo).await?;
        let Some(existing) = existing_point else {
            return Err(AppError::NotFound(format!("Memo {} not found", memo.id)));
        };

        let vector = vector.unwrap_or_else(|| self.extract_vector_from_point(&existing));

        self.insert_memo(memo, vector, demo).await
    }

    #[allow(deprecated)]
    fn extract_vector_from_point(&self, point: &qdrant_client::qdrant::RetrievedPoint) -> Vec<f32> {
        point
            .vectors
            .as_ref()
            .and_then(|v| v.vectors_options.as_ref())
            .and_then(|v| match v {
                qdrant_client::qdrant::vectors_output::VectorsOptions::Vector(vec) => {
                    Some(vec.data.clone())
                }
                _ => None,
            })
            .unwrap_or_default()
    }

    async fn get_memo_point(
        &self,
        id: Uuid,
        demo: bool,
    ) -> Result<Option<qdrant_client::qdrant::RetrievedPoint>> {
        let collection = if demo {
            COLLECTION_MEMOS_DEMO
        } else {
            COLLECTION_MEMOS
        };

        let point_id: PointId = id.to_string().into();

        let points = self
            .client
            .get_points(
                GetPointsBuilder::new(collection, vec![point_id])
                    .with_payload(true)
                    .with_vectors(true),
            )
            .await
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        Ok(points.result.into_iter().next())
    }

    pub async fn delete_memo(&self, id: Uuid, demo: bool) -> Result<()> {
        let collection = if demo {
            COLLECTION_MEMOS_DEMO
        } else {
            COLLECTION_MEMOS
        };

        let point_id: PointId = id.to_string().into();

        self.client
            .delete_points(
                DeletePointsBuilder::new(collection)
                    .points(PointsIdsList { ids: vec![point_id] }),
            )
            .await
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        Ok(())
    }

    pub async fn search(
        &self,
        vector: Vec<f32>,
        filters: &SearchFilters,
        limit: u32,
        demo: bool,
    ) -> Result<Vec<SearchResult>> {
        let collection = if demo {
            COLLECTION_MEMOS_DEMO
        } else {
            COLLECTION_MEMOS
        };

        let filter = self.build_filter(filters);

        let mut search_builder =
            SearchPointsBuilder::new(collection, vector, limit as u64).with_payload(true);

        if let Some(f) = filter {
            search_builder = search_builder.filter(f);
        }

        let results = self
            .client
            .search_points(search_builder)
            .await
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        results
            .result
            .into_iter()
            .map(|point| {
                let id = self.extract_uuid_from_point_id(&point.id)?;

                let content = self.get_string_field(&point.payload, "content")?;
                let tags = self.get_string_array_field(&point.payload, "tags")?;
                let from = self.get_optional_date_field(&point.payload, "from")?;
                let date_added = self.get_datetime_field(&point.payload, "date_added")?;

                // スコア変換: ベースライン0.77を基準に0-1にスケール、指数変換で高スコアを強調
                let baseline = 0.77_f32;
                let linear = ((point.score - baseline) / (1.0 - baseline)).clamp(0.0, 1.0);
                let normalized_score = linear.powf(0.5);

                Ok(SearchResult {
                    id,
                    content,
                    score: normalized_score,
                    tags,
                    from,
                    date_added,
                })
            })
            .collect()
    }

    pub async fn get_all_tags(&self, demo: bool) -> Result<Vec<String>> {
        use qdrant_client::qdrant::ScrollPointsBuilder;
        use std::collections::HashSet;

        let collection = if demo {
            COLLECTION_MEMOS_DEMO
        } else {
            COLLECTION_MEMOS
        };

        let mut all_tags: HashSet<String> = HashSet::new();
        let mut offset: Option<PointId> = None;
        let limit = 100u32;

        loop {
            let mut scroll_builder = ScrollPointsBuilder::new(collection)
                .with_payload(true)
                .limit(limit);

            if let Some(ref off) = offset {
                scroll_builder = scroll_builder.offset(off.clone());
            }

            let result = self
                .client
                .scroll(scroll_builder)
                .await
                .map_err(|e| AppError::Qdrant(e.to_string()))?;

            for point in &result.result {
                if let Ok(tags) = self.get_string_array_field(&point.payload, "tags") {
                    for tag in tags {
                        all_tags.insert(tag);
                    }
                }
            }

            match result.next_page_offset {
                Some(next_offset) => offset = Some(next_offset),
                None => break,
            }

            if result.result.len() < limit as usize {
                break;
            }
        }

        let mut tags: Vec<String> = all_tags.into_iter().collect();
        tags.sort();
        Ok(tags)
    }

    fn extract_uuid_from_point_id(&self, point_id: &Option<PointId>) -> Result<Uuid> {
        let point_id = point_id
            .as_ref()
            .ok_or_else(|| AppError::Qdrant("Missing point ID".into()))?;

        match &point_id.point_id_options {
            Some(PointIdOptions::Uuid(uuid_str)) => Uuid::parse_str(uuid_str)
                .map_err(|e| AppError::Qdrant(format!("Invalid UUID: {}", e))),
            Some(PointIdOptions::Num(num)) => {
                Err(AppError::Qdrant(format!("Unexpected numeric ID: {}", num)))
            }
            None => Err(AppError::Qdrant("Missing point ID options".into())),
        }
    }

    fn build_filter(&self, filters: &SearchFilters) -> Option<Filter> {
        let mut conditions = Vec::new();

        if let Some(ref memo_type) = filters.memo_type {
            let type_str = match memo_type {
                MemoType::Flash => "flash",
                MemoType::Permanent => "permanent",
            };
            conditions.push(Condition::matches("type", type_str.to_string()));
        }

        if let Some(from_gte) = filters.from_gte {
            conditions.push(Condition::range(
                "from_ts",
                qdrant_client::qdrant::Range {
                    gte: Some(
                        from_gte
                            .and_hms_opt(0, 0, 0)
                            .unwrap()
                            .and_utc()
                            .timestamp() as f64,
                    ),
                    ..Default::default()
                },
            ));
        }

        if let Some(until_lte) = filters.until_lte {
            conditions.push(Condition::range(
                "until_ts",
                qdrant_client::qdrant::Range {
                    lte: Some(
                        until_lte
                            .and_hms_opt(23, 59, 59)
                            .unwrap()
                            .and_utc()
                            .timestamp() as f64,
                    ),
                    ..Default::default()
                },
            ));
        }

        for tag in &filters.tags {
            conditions.push(Condition::matches("tags", tag.clone()));
        }

        if conditions.is_empty() {
            None
        } else {
            Some(Filter::must(conditions))
        }
    }

    fn memo_to_payload(&self, memo: &Memo) -> HashMap<String, Value> {
        let type_str = match memo.memo_type {
            MemoType::Flash => "flash",
            MemoType::Permanent => "permanent",
        };

        let mut payload: HashMap<String, Value> = HashMap::new();
        payload.insert("content".into(), memo.content.clone().into());
        payload.insert("type".into(), type_str.into());
        payload.insert("tags".into(), memo.tags.clone().into());
        payload.insert("date_added".into(), memo.date_added.to_rfc3339().into());
        payload.insert("access_count".into(), (memo.access_count as i64).into());
        payload.insert(
            "last_accessed".into(),
            memo.last_accessed.to_rfc3339().into(),
        );
        payload.insert("completed".into(), memo.completed.into());

        if let Some(from) = memo.from {
            payload.insert("from".into(), from.to_string().into());
            payload.insert(
                "from_ts".into(),
                (from.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as i64).into(),
            );
        }

        if let Some(until) = memo.until {
            payload.insert("until".into(), until.to_string().into());
            payload.insert(
                "until_ts".into(),
                (until
                    .and_hms_opt(23, 59, 59)
                    .unwrap()
                    .and_utc()
                    .timestamp() as i64)
                    .into(),
            );
        }

        payload
    }

    fn payload_to_memo(&self, id: Uuid, payload: &HashMap<String, Value>) -> Result<Memo> {
        let content = self.get_string_field(payload, "content")?;
        let type_str = self.get_string_field(payload, "type")?;
        let memo_type = match type_str.as_str() {
            "flash" => MemoType::Flash,
            "permanent" => MemoType::Permanent,
            other => {
                return Err(AppError::Qdrant(format!("Unknown memo type: {}", other)));
            }
        };
        let tags = self.get_string_array_field(payload, "tags")?;
        let date_added = self.get_datetime_field(payload, "date_added")?;
        let access_count = self.get_int_field(payload, "access_count")? as u32;
        let last_accessed = self.get_datetime_field(payload, "last_accessed")?;
        let completed = self.get_bool_field(payload, "completed")?;
        let from = self.get_optional_date_field(payload, "from")?;
        let until = self.get_optional_date_field(payload, "until")?;

        Ok(Memo {
            id,
            content,
            memo_type,
            from,
            until,
            tags,
            date_added,
            access_count,
            last_accessed,
            completed,
        })
    }

    fn get_string_field(&self, payload: &HashMap<String, Value>, key: &str) -> Result<String> {
        payload
            .get(key)
            .and_then(|v| v.kind.as_ref())
            .and_then(|k| match k {
                qdrant_client::qdrant::value::Kind::StringValue(s) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| AppError::Qdrant(format!("Missing or invalid field: {}", key)))
    }

    fn get_string_array_field(
        &self,
        payload: &HashMap<String, Value>,
        key: &str,
    ) -> Result<Vec<String>> {
        payload
            .get(key)
            .and_then(|v| v.kind.as_ref())
            .and_then(|k| match k {
                qdrant_client::qdrant::value::Kind::ListValue(list) => Some(
                    list.values
                        .iter()
                        .filter_map(|v| v.kind.as_ref())
                        .filter_map(|k| match k {
                            qdrant_client::qdrant::value::Kind::StringValue(s) => Some(s.clone()),
                            _ => None,
                        })
                        .collect(),
                ),
                _ => None,
            })
            .ok_or_else(|| AppError::Qdrant(format!("Missing or invalid field: {}", key)))
    }

    fn get_int_field(&self, payload: &HashMap<String, Value>, key: &str) -> Result<i64> {
        payload
            .get(key)
            .and_then(|v| v.kind.as_ref())
            .and_then(|k| match k {
                qdrant_client::qdrant::value::Kind::IntegerValue(i) => Some(*i),
                _ => None,
            })
            .ok_or_else(|| AppError::Qdrant(format!("Missing or invalid field: {}", key)))
    }

    fn get_bool_field(&self, payload: &HashMap<String, Value>, key: &str) -> Result<bool> {
        payload
            .get(key)
            .and_then(|v| v.kind.as_ref())
            .and_then(|k| match k {
                qdrant_client::qdrant::value::Kind::BoolValue(b) => Some(*b),
                _ => None,
            })
            .ok_or_else(|| AppError::Qdrant(format!("Missing or invalid field: {}", key)))
    }

    fn get_datetime_field(
        &self,
        payload: &HashMap<String, Value>,
        key: &str,
    ) -> Result<chrono::DateTime<Utc>> {
        let s = self.get_string_field(payload, key)?;
        chrono::DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| AppError::Qdrant(format!("Invalid datetime for {}: {}", key, e)))
    }

    fn get_optional_date_field(
        &self,
        payload: &HashMap<String, Value>,
        key: &str,
    ) -> Result<Option<NaiveDate>> {
        match payload.get(key) {
            None => Ok(None),
            Some(v) => match v.kind.as_ref() {
                Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                    NaiveDate::parse_from_str(s, "%Y-%m-%d")
                        .map(Some)
                        .map_err(|e| AppError::Qdrant(format!("Invalid date for {}: {}", key, e)))
                }
                Some(qdrant_client::qdrant::value::Kind::NullValue(_)) => Ok(None),
                _ => Ok(None),
            },
        }
    }
}
