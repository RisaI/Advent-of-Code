#[derive(Debug, Default)]
pub struct MinHeap<T> {
    data: Vec<T>,
}

fn left_child(index: usize) -> usize {
    2 * index + 1
}

fn right_child(index: usize) -> usize {
    2 * index + 2
}

fn parent(index: usize) -> Option<usize> {
    if index == 0 {
        return None;
    }

    Some(match index % 2 {
        0 => index / 2 - 1,
        _ => index / 2,
    })
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let data = self.data.swap_remove(0);
        self.sift_down(0);

        Some(data)
    }

    fn sift_up(&mut self, idx: usize) {
        let Some(parent_idx) = parent(idx) else {
            return;
        };

        if self.data[idx] < self.data[parent_idx] {
            self.data.swap(idx, parent_idx);
        }

        self.sift_up(parent_idx);
    }

    fn sift_down(&mut self, idx: usize) {
        if idx >= self.data.len() {
            return;
        }

        let left_idx = left_child(idx);
        let right_idx = right_child(idx);

        if left_idx >= self.data.len() {
            return;
        }

        let lower_idx =
            if right_idx >= self.data.len() || self.data[left_idx] < self.data[right_idx] {
                left_idx
            } else {
                right_idx
            };

        if self.data[idx] > self.data[lower_idx] {
            self.data.swap(idx, lower_idx);
            self.sift_down(lower_idx);
        }
    }
}
