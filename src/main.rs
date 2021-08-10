mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //crate::day_one::solve().await;
    //crate::day_two::solve().await;
    //crate::day_three::solve().await;
    //crate::day_four::solve().await?;
    crate::day_five::solve().await?;
    Ok(())
}
