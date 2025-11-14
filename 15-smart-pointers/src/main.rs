use crate::resource::Resource;
use anyhow::{Result, anyhow};

mod list;
mod mut_list;
mod resource;

fn main() -> Result<()> {
    {
        use crate::list::CanBeConned;
        println!(
            "list = {}",
            1.cons(&2.cons(&3.cons(&4.cons(&5.nil()))))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" :: ")
        );
    }

    {
        use crate::mut_list::CanBeConned;
        println!(
            "mut_list = {}",
            1.cons(&2.cons(&3.cons(&4.cons(&5.nil()))))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" :: ")
        );
    }

    let resource = Resource::new(|| -> Result<i32, String> { Ok(5) }, |_| Ok(()));

    dbg!(resource.use_value(|x| Ok(*x))).map_err(|x| anyhow!(x))?;
    dbg!(resource.use_value(|x| Ok(x + 1))).map_err(|x| anyhow!(x))?;
    dbg!(resource.use_value(|x| Ok(x + 2))).map_err(|x| anyhow!(x))?;
    Ok(())
}
