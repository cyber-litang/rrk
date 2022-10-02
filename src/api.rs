use crate::accounts;
use anyhow::{Context, Result};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

const PREFIX: &str = "/home/students";

pub fn batch_add_users(path: &Path) -> Result<()> {
    let mut users: HashMap<String, Vec<_>> = HashMap::new();
    for line in std::fs::read_to_string(path)?
        .lines()
        .filter(|l| !l.is_empty())
    {
        let (id, class) = line.split_once(':').context("split file failed")?;
        let id = id.trim().to_string();
        let class = class.to_string();
        users.entry(class).or_default().push(id);
    }
    crate::db::insert_users(users)?;
    Ok(())
}

pub fn batch_get_users(class: Option<&str>) -> Result<()> {
    let users = if let Some(class) = class {
        crate::db::get_user_ids_by_class(class)?
    } else {
        crate::db::get_all_user_ids()?
    };
    for user in users {
        println!("{}", user);
    }
    Ok(())
}

pub fn clear_users() -> Result<()> {
    crate::accounts::delete_all_accounts()?;
    println!("all users cleared");
    Ok(())
}

pub fn sync_users() -> Result<()> {
    let users_in_db = crate::db::get_all_user_ids()?
        .into_iter()
        .collect::<HashSet<_>>();
    let users_in_system = accounts::get_all_accounts()?
        .into_iter()
        .collect::<HashSet<_>>();
    let new_users = users_in_db
        .iter()
        .filter(|id| !users_in_system.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    let removed_users = users_in_system
        .iter()
        .filter(|id| !users_in_db.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    println!(
        "users_in_db = {:?}, users_in_system = {:?}",
        users_in_db, users_in_system
    );
    println!(
        "new_users = {:?}, removed_users = {:?}",
        new_users, removed_users
    );
    accounts::add_accounts(PREFIX, &new_users)?;
    accounts::delete_accounts(&removed_users)?;
    Ok(())
}
