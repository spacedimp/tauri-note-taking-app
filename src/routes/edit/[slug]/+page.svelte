<script>
	import { myStore, editStore } from '$lib/store.js';
	import { page } from '$app/stores';
	import { onMount} from 'svelte';

	let oneNote;
	let oneNoteBody;
	let oneNoteTitle;

	let slug;

	function save(){
		let currentStore = $myStore;
		let index = currentStore.findIndex(item => item.bson_uuid === slug); 
		let updatedObject = {...currentStore[index]};
		updatedObject.title = oneNoteTitle;
		updatedObject.body = oneNoteBody;

		myStore.update(store => {
			let updatedStore = [...store];
			updatedStore[index] = updatedObject;

			return updatedStore;
		});

		editStore($myStore);
	}

	function del() {
		myStore.update(objects => objects.filter(obj => obj.bson_uuid !== slug));
		editStore($myStore);
		window.location.href="/";
	}

	onMount(async () => {
		slug = $page.params.slug.toString();
		$myStore.forEach(element => {
			if (element.bson_uuid === slug) {
				oneNote = element;
				oneNoteTitle = element.title;
				oneNoteBody = element.body;
				console.log(oneNote);		
				return;
			} else {
				console.log("wrong page");	
			}
		});
	});

</script>

<div id = "new-note">
	<h1> Edit note </h1>
	<a href="/" id="home-button">BACK</a>
	<button on:click={save}>Save</button>
	<button on:click={del}>Delete</button>
	<textarea bind:value={oneNoteTitle} id="edit-note-title"></textarea>
	<textarea bind:value={oneNoteBody} id="edit-note-box"></textarea>
</div>

