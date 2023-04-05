<script>
	import { myStore, loadStore } from '$lib/store.js';

  	import { homeDir, join } from '@tauri-apps/api/path';
  	import { exists, BaseDirectory, createDir, writeBinaryFile, readBinaryFile } from '@tauri-apps/api/fs';

  	import {invoke} from '@tauri-apps/api/tauri';
  	import { onMount } from 'svelte';

	import Notes from '$lib/Notes.svelte'
	import CreateNote from '$lib/CreateNote.svelte'

	var db;

	// gets called whenever the page/component gets mounted
	onMount(async () => {
 		let home = await homeDir();	
		db = await join(home, 'notes-db');

		// check if notes-db directory exists. If not then create it
		let checkDB = await exists('notes-db', {dir: BaseDirectory.Home});
		if (!checkDB) {
			await createDir('notes-db', {dir: BaseDirectory.Home, recursive: true });
		}

		// check if db.bson exists. If not then create it. 
		let checkFile = await exists('./notes-db/db.bson', {dir: BaseDirectory.Home});
		if (!checkFile) {
			await writeBinaryFile('./notes-db/db.bson', new Uint8Array([]), {dir: BaseDirectory.Home});		
		}

		loadStore();
  });
</script>

<div id="container">
	<CreateNote/>
	<Notes allNotes={$myStore} />
</div>

