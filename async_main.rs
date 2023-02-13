use async_io::runtime::*;
use std::io::Result;

fn main() -> Result<()> {
   let mut executor = Executor::new();
    executor(spawn {
       let x = test_runtime().await;
       println!(" --> {}", x);
    });
  
    executor.run();
   Ok(())
}
  
async fn test_runtime() -> usize {
    7
}
