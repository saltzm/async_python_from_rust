use std::time::Duration;

pub struct MyLibrary {
    // Not needed in simple example, but could be used to store other data.
}

impl MyLibrary {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn sleep_and_add(&self, left: u64, right: u64) -> u64 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        left + right
    }
}

// Declare and re-export the C bindings
mod c_bindings;
pub use c_bindings::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let result = MyLibrary::new().sleep_and_add(2, 2).await;
        assert_eq!(result, 4);
    }
}
