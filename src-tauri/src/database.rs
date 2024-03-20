extern crate rusqlite;
extern crate chrono;

use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Local, TimeZone};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task{
    id: u32,
    name: String,
    explain: Option<String>,
    status: u8, // 0: todo, 1:doing, 2: suspend, 3: cancel, 4: done
    created_date: u64,
    updated_date: u64,
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

pub fn get_tasks(con: &Connection) -> Vec<Task> {
  let mut state = con.prepare("select * from tasks").unwrap();
  let result = state.query_map(params![], |row| {
    Ok(Task {
      id: row.get(0).unwrap(),
      name: row.get(1).unwrap(),
      explain: Some(row.get(2).unwrap()),
      status: row.get(3).unwrap(),
      created_date: row.get(4).unwrap(),
      updated_date: row.get(5).unwrap(),
      estimate_sec: row.get(6).unwrap(),
      actual_sec: row.get(7).unwrap()
    })
  }).unwrap();

  let mut tasks: Vec<Task> = vec![];
  for task in result {
    tasks.push(task.unwrap());
  }

  tasks
}