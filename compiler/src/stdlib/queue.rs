
pub struct QueueStdlib;

impl QueueStdlib {
    pub fn generate_create_code(capacity: Option<&str>) -> String {
        if let Some(cap) = capacity {
            format!(
                "std::collections::VecDeque::with_capacity({} as usize)",
                cap
            )
        } else {
            "std::collections::VecDeque::new()".to_string()
        }
    }

    pub fn generate_enqueue_code(queue: &str, item: &str) -> String {
        format!("{}.push_back({})", queue, item)
    }

    pub fn generate_dequeue_code(queue: &str) -> String {
        format!("{}.pop_front()", queue)
    }

    pub fn generate_peek_code(queue: &str) -> String {
        format!("{}.front().cloned()", queue)
    }

    pub fn generate_size_code(queue: &str) -> String {
        format!("{}.len() as i64", queue)
    }

    pub fn generate_is_empty_code(queue: &str) -> String {
        format!("{}.is_empty()", queue)
    }

    pub fn generate_is_full_code(queue: &str) -> String {
        format!("false") // VecDeque has no capacity limit by default
    }

    pub fn generate_priority_create_code(compare: &str) -> String {
        format!(
            "{{
                use std::cmp::Reverse;
                use std::collections::BinaryHeap;
                BinaryHeap::new()
            }}"
        )
    }

    pub fn generate_priority_enqueue_code(queue: &str, item: &str, priority: &str) -> String {
        format!(
            "{{
                use std::cmp::Reverse;
                {}.push(Reverse(({} as i64, {})))
            }}",
            queue, priority, item
        )
    }

    pub fn generate_bounded_create_code(capacity: &str) -> String {
        format!(
            "std::collections::VecDeque::with_capacity({} as usize)",
            capacity
        )
    }
}
