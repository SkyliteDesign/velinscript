pub struct StreamStdlib;

impl StreamStdlib {
    pub fn generate_create_code() -> String {
        "futures::stream::unfold((), |_| async { Some(((), ())) }).boxed()".to_string()
    }

    pub fn generate_map_code(stream: &str, mapper: &str) -> String {
        format!("{}.map(|item| {}).boxed()", stream, mapper)
    }

    pub fn generate_filter_code(stream: &str, predicate: &str) -> String {
        format!(
            "{}.filter(|item| async move {{ {} }}).boxed()",
            stream, predicate
        )
    }

    pub fn generate_reduce_code(stream: &str, reducer: &str, initial: &str) -> String {
        format!(
            "{}.fold({}, |acc, item| async move {{ {} }}).await",
            stream, initial, reducer
        )
    }

    pub fn generate_batch_code(stream: &str, size: &str) -> String {
        format!("{}.chunks({} as usize).boxed()", stream, size)
    }

    pub fn generate_buffer_code(stream: &str, size: &str) -> String {
        format!("{}.buffered({} as usize).boxed()", stream, size)
    }

    pub fn generate_merge_code(stream1: &str, stream2: &str) -> String {
        format!("futures::stream::select({}, {}).boxed()", stream1, stream2)
    }

    pub fn generate_zip_code(stream1: &str, stream2: &str) -> String {
        format!("{}.zip({}).boxed()", stream1, stream2)
    }
}
