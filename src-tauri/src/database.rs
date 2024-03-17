extern crate rusqlite;
extern crate chrono;

use std::{collections::HashMap, thread::current};
use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Local, TimeZone};

#[derive(Debug)]
struct Task{
    id: u32,
    name: String,
    explain: Option<String>,
    status: u8, // 0: todo, 1:doing, 2: suspend, 3: cancel, 4: done
    created_date: DateTime<Local>,
    updated_date: DateTime<Local>,
    estimate_sec: u32,
    actual_sec: u32
}

pub fn open_db() -> Result<Connection, rusqlite::Error> {
    let path = "./test.db";
    let con = Connection::open(&path)?;
    println!("{}", con.is_autocommit());
    Ok(con)
}

// pub fn insert_task(con: &Connection, task: &Task) -> Result<usize, rusqlite::Error> {
//     let current_date: DateTime<Local> = Local::now();
//     let unix_time: i64 = current_date.timestamp();

//     con.execute(
//       "insert into tasks 
//         (name, explain, status, created_date, updated_date, estimate_sec, actual_sec) 
//       values 
//         (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//       params![task.name, task.explain.as_ref(), 0, unix_time, unix_time, task.estimate_sec, 0])
// }

pub fn get_all_tasks(con: &Connection) {
  let mut state = con.prepare("select * from tasks").unwrap();
  let tasks = state.query_map(params![], |row| {
    // println!("{:?}", row);
    Ok(Task {
      id: row.get(0).unwrap(),
      name: row.get(1).unwrap(),
      explain: Some(row.get(2).unwrap()),
      status: row.get(3).unwrap(),
      created_date: Local.timestamp_opt(row.get(4).unwrap(), 0).unwrap(),
      updated_date: Local.timestamp_opt(row.get(5).unwrap(), 0).unwrap(),
      estimate_sec: row.get(6).unwrap(),
      actual_sec: row.get(7).unwrap()
    })
  }).unwrap();

  for task in tasks {
    println!("{:?}", task);
  }
}