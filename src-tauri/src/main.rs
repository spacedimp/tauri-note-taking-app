#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use bson::{Document};

use serde::{Serialize, Deserialize};
use std::io::{Cursor};

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    bson_uuid: String,
    date_time: bson::DateTime,
    title: String,
    body: String,
}

// builds a new Note object for the froteend
// we then convert it to a bson document
// lastly we convert it into a vec of bytes to store on disk (frontend handles appending then saving this to disk)
#[tauri::command]
fn saveNote(title: &str, body: &str) -> Vec<u8> {
    let note = Note { bson_uuid: bson::Uuid::new().to_string(), date_time: bson::DateTime::now(), title: title.to_string(), body: body.to_string() };
    let note_doc = bson::to_document(&note).unwrap();

    return bson::to_vec(&note_doc).unwrap();
}

// after the frontend edits or deletes a note
// it must be saved back to db.bson
#[tauri::command]
fn editNote(data: &str) -> Vec<u8> {
    let vecNotes: Vec<Note> = serde_json::from_str(data).unwrap();
    let vecDocs: Vec<Document> = vecNotes.iter().map(|e| bson::to_document(&e).unwrap() ).collect();
    let docsArray: Vec<u8> = vecDocs.clone().into_iter().flat_map(|e| bson::to_vec(&e.clone()).unwrap()).collect();

    return docsArray;
}

// loading the raw data from db.bson requires us to convert it to JSON
// for the frontend to interact with
#[tauri::command]
fn loadNotes(data: &str) -> String{

   // check if database is empty.
   // Return if it is otherwise the program will crash
   if data.chars().count() == 0 {
        return String::from("no data");
   }

   // frontend passes the database as a string array of bytes
   // parse it into bytes
   let mybytes: Vec<u8> = data
       .trim_matches(|c| c == '[' || c== ']')
       .split(',')
       .map(|s| s.parse().unwrap())
       .collect();

   // now we iterate through the bytes and convert it
   // to a Vec of bson Document
   let mut curs = Cursor::new(mybytes);
   curs.set_position(0);

   let array_len = curs.get_ref().len() as u64;

   let mut docs = Vec::new();

   for _ in 0..array_len {
        match Document::from_reader(&mut curs) {
            Ok(doc) => {println!("{} \n\n", doc); docs.push(doc);},
            Err(e) => {
                println!("Error {:?}", e);
                break;
            }
        }
   }

   // return to the frontend an array of bson documents as JSON
   return serde_json::to_string(&docs).unwrap();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![saveNote, editNote, loadNotes])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
