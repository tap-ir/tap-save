use tap::tree::Tree;
use tap::task_scheduler::Task;
use tap::session::Session;

use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use log::warn;

pub struct SaveTree
{
}

impl SaveTree
{
  pub fn to_file(_file_name : String, _tree : &Tree)
  {
  }

  pub fn from_file(_file_name : String, _tree : &Tree)
  {

  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInfo
{
  id : u32,
  plugin : String,
  argument : String,
}


pub struct TaskReplay
{
  
}


impl TaskReplay 
{
  ///save all task to a file
  pub fn to_file(file_name : String, session : &Session) //choose file format .bin .json ? 
  {
    let mut tasks : Vec<Task> = session.task_scheduler.tasks_finished().into_iter().map(|(task, _)| task).collect();
    tasks.sort_by(|a, b| a.id.cmp(&b.id));

    let json = json!(&tasks);
    let file = File::create(file_name.clone()).unwrap();
    serde_json::to_writer(file, &json).unwrap();
    warn!("Task save to file {}", file_name);
  }

  ///reload all task from file
  pub fn from_file(file_name : String, session : &Session)
  {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut tasks : Vec<Task> = serde_json::from_reader(reader).unwrap();
    tasks.sort_by(|a, b| a.id.cmp(&b.id)); //we sort tasks by id 

    warn!("Loading tasks replay file");
    for task in tasks
    {
      let _res = session.run(&task.plugin_name, task.argument, false);
    }
    warn!("Tasks replay file loaded")
  }
}

pub enum Save
{
  Replay,
  Tree,
}

impl Save
{
  pub fn to_file(&self, file_name : String, session : &Session)
  {
    match self
    {
      Save::Tree => SaveTree::to_file(file_name, &session.tree),
      Save::Replay => TaskReplay::to_file(file_name, session),
    }

  }
 
  pub fn from_file(&self, file_name : String, session : &Session)
  {
    match self
    {
      Save::Tree => SaveTree::from_file(file_name, &session.tree),
      Save::Replay => TaskReplay::from_file(file_name, session),
    }
  }
}
