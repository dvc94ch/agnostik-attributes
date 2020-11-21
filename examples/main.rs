#[async_attributes::main]
async fn main() {
    agnostik::spawn(async {
        println!("Hello, world!");
    })
    .await;
}
