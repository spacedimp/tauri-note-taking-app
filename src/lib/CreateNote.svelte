<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { loadStore } from '$lib/store.js'
	import {BaseDirectory, writeBinaryFile, readBinaryFile} from '@tauri-apps/api/fs'

	let newNote;
	let newTitle;

	let allNotes;

	async function save(){
			await load();

			invoke('saveNote', {title: newTitle, body: newNote} ).then((response) => {
				let loaded = new Uint8Array(allNotes);
				response = new Uint8Array(response);
				let mergeArray = new Uint8Array(loaded.length + response.length);
				mergeArray.set(loaded);
				mergeArray.set(response, loaded.length);
				writeBinaryFile('./notes-db/db.bson', mergeArray, {dir: BaseDirectory.Home});	

				// after saving, reload writable myStore with saved data on disk
				loadStore();

				// empty textarea contents after save	
				newNote = "";
				newTitle = "";
			});
	}

	async function load() {
		allNotes = await readBinaryFile('./notes-db/db.bson', { dir: BaseDirectory.Home});	
	}
</script>

<div id = "new-note">
	<h1> Create a note </h1>
	<button on:click={save}>Save</button>
	<textarea bind:value={newTitle} id="new-note-title" placeholder="Note title"></textarea>
	<textarea bind:value={newNote} id="new-note-box" placeholder="Note body"></textarea>
</div>

