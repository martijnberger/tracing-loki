// This file is @generated by prost-build.
/// Result contains LogQL query statistics.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Result {
    #[prost(message, optional, tag = "1")]
    pub summary: ::core::option::Option<Summary>,
    #[prost(message, optional, tag = "2")]
    pub querier: ::core::option::Option<Querier>,
    #[prost(message, optional, tag = "3")]
    pub ingester: ::core::option::Option<Ingester>,
    #[prost(message, optional, tag = "4")]
    pub caches: ::core::option::Option<Caches>,
    #[prost(message, optional, tag = "5")]
    pub index: ::core::option::Option<Index>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Caches {
    #[prost(message, optional, tag = "1")]
    pub chunk: ::core::option::Option<Cache>,
    #[prost(message, optional, tag = "2")]
    pub index: ::core::option::Option<Cache>,
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<Cache>,
    #[prost(message, optional, tag = "4")]
    pub stats_result: ::core::option::Option<Cache>,
    #[prost(message, optional, tag = "5")]
    pub volume_result: ::core::option::Option<Cache>,
    #[prost(message, optional, tag = "6")]
    pub series_result: ::core::option::Option<Cache>,
    #[prost(message, optional, tag = "7")]
    pub label_result: ::core::option::Option<Cache>,
    #[prost(message, optional, tag = "8")]
    pub instant_metric_result: ::core::option::Option<Cache>,
}
/// Summary is the summary of a query statistics.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Summary {
    /// Total bytes processed per second.
    #[prost(int64, tag = "1")]
    pub bytes_processed_per_second: i64,
    /// Total lines processed per second.
    #[prost(int64, tag = "2")]
    pub lines_processed_per_second: i64,
    /// Total bytes processed. Includes structured metadata bytes.
    #[prost(int64, tag = "3")]
    pub total_bytes_processed: i64,
    /// Total lines processed.
    #[prost(int64, tag = "4")]
    pub total_lines_processed: i64,
    /// Execution time in seconds.
    /// In addition to internal calculations this is also returned by the HTTP API.
    /// Grafana expects time values to be returned in seconds as float.
    #[prost(double, tag = "5")]
    pub exec_time: f64,
    /// Queue time in seconds.
    /// In addition to internal calculations this is also returned by the HTTP API.
    /// Grafana expects time values to be returned in seconds as float.
    #[prost(double, tag = "6")]
    pub queue_time: f64,
    /// Subqueries exists for backwards compatibility reasons and is deprecated. Do not use.
    /// Instead use splits and shards
    #[prost(int64, tag = "7")]
    pub subqueries: i64,
    /// Total number of result entries returned
    #[prost(int64, tag = "8")]
    pub total_entries_returned: i64,
    /// Total number of splits by time
    #[prost(int64, tag = "9")]
    pub splits: i64,
    /// Total number of shards
    #[prost(int64, tag = "10")]
    pub shards: i64,
    /// Total lines post query filtering
    #[prost(int64, tag = "11")]
    pub total_post_filter_lines: i64,
    /// Total bytes processed of metadata.
    #[prost(int64, tag = "12")]
    pub total_structured_metadata_bytes_processed: i64,
}
/// Statistics from Index queries
/// TODO(owen-d): include bytes.
/// Needs some index methods added to return _sized_ chunk refs to know
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Index {
    /// Total chunks
    #[prost(int64, tag = "1")]
    pub total_chunks: i64,
    /// Post-filtered chunks
    #[prost(int64, tag = "2")]
    pub post_filter_chunks: i64,
    /// Nanosecond duration spent fetching shards
    #[prost(int64, tag = "3")]
    pub shards_duration: i64,
    /// Indicates whether the query used blooms to filter chunks
    #[prost(bool, tag = "4")]
    pub used_bloom_filters: bool,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Querier {
    #[prost(message, optional, tag = "1")]
    pub store: ::core::option::Option<Store>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Ingester {
    /// Total ingester reached for this query.
    #[prost(int32, tag = "1")]
    pub total_reached: i32,
    /// Total of chunks matched by the query from ingesters
    #[prost(int64, tag = "2")]
    pub total_chunks_matched: i64,
    /// Total of batches sent from ingesters.
    #[prost(int64, tag = "3")]
    pub total_batches: i64,
    /// Total lines sent by ingesters.
    #[prost(int64, tag = "4")]
    pub total_lines_sent: i64,
    #[prost(message, optional, tag = "5")]
    pub store: ::core::option::Option<Store>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Store {
    /// The total of chunk reference fetched from index.
    #[prost(int64, tag = "1")]
    pub total_chunks_ref: i64,
    /// Total number of chunks fetched.
    #[prost(int64, tag = "2")]
    pub total_chunks_downloaded: i64,
    /// Time spent fetching chunks in nanoseconds.
    #[prost(int64, tag = "3")]
    pub chunks_download_time: i64,
    /// Whether the query referenced structured metadata
    #[prost(bool, tag = "13")]
    pub query_referenced_structured: bool,
    #[prost(message, optional, tag = "4")]
    pub chunk: ::core::option::Option<Chunk>,
    /// Time spent fetching chunk refs from index.
    #[prost(int64, tag = "5")]
    pub chunk_refs_fetch_time: i64,
    /// Time spent being blocked on congestion control.
    #[prost(int64, tag = "6")]
    pub congestion_control_latency: i64,
    /// Total number of lines filtered by pipeline wrapper.
    #[prost(int64, tag = "7")]
    pub pipeline_wrapper_filtered_lines: i64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Chunk {
    /// Total bytes processed but was already in memory (found in the headchunk). Includes structured metadata bytes.
    #[prost(int64, tag = "4")]
    pub head_chunk_bytes: i64,
    /// Total lines processed but was already in memory. (found in the headchunk)
    #[prost(int64, tag = "5")]
    pub head_chunk_lines: i64,
    /// Total bytes decompressed and processed from chunks. Includes structured metadata bytes.
    #[prost(int64, tag = "6")]
    pub decompressed_bytes: i64,
    /// Total lines decompressed and processed from chunks.
    #[prost(int64, tag = "7")]
    pub decompressed_lines: i64,
    /// Total bytes of compressed chunks (blocks) processed.
    #[prost(int64, tag = "8")]
    pub compressed_bytes: i64,
    /// Total duplicates found while processing.
    #[prost(int64, tag = "9")]
    pub total_duplicates: i64,
    /// Total lines post filtering
    #[prost(int64, tag = "10")]
    pub post_filter_lines: i64,
    /// Total bytes processed for metadata but was already in memory. (found in the headchunk)
    #[prost(int64, tag = "11")]
    pub head_chunk_structured_metadata_bytes: i64,
    /// Total bytes of entries metadata decompressed and processed from chunks.
    #[prost(int64, tag = "12")]
    pub decompressed_structured_metadata_bytes: i64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cache {
    #[prost(int32, tag = "1")]
    pub entries_found: i32,
    #[prost(int32, tag = "2")]
    pub entries_requested: i32,
    #[prost(int32, tag = "3")]
    pub entries_stored: i32,
    #[prost(int64, tag = "4")]
    pub bytes_received: i64,
    #[prost(int64, tag = "5")]
    pub bytes_sent: i64,
    #[prost(int32, tag = "6")]
    pub requests: i32,
    #[prost(int64, tag = "7")]
    pub download_time: i64,
    #[prost(int64, tag = "8")]
    pub query_length_served: i64,
}
