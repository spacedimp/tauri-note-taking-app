import { writable } from 'svelte/store';

import { homeDir, join } from '@tauri-apps/api/path';
import { exists, BaseDirectory, createDir, writeBinaryFile, readBinaryFile } from '@tauri-apps/api/fs';

import {invoke} from '@tauri-apps/api/tauri';

export const myStore = writable({});

// initialize myStore with the contents in the database
// let Rust convert the binary to an array of JSON
export async function loadStore() {
		let binData = await readBinaryFile('./notes-db/db.bson', {dir: BaseDirectory.Home});
		invoke('loadNotes', {data: binData.toString()}).then((dat) => {
			myStore.set(JSON.parse(dat));
		});
}

// send the updated JSON to the backend as a string
// the backend converts it to an array of bson documents as bytes and we store it to db.bson 
export async function editStore(newVal) {
	let jsonToString = JSON.stringify(newVal);
	invoke('editNote', {data: jsonToString}).then((dat) => {
		let data = new Uint8Array(dat);
		writeBinaryFile('./notes-db/db.bson', data, {dir: BaseDirectory.Home});
		loadStore();
	});
}
