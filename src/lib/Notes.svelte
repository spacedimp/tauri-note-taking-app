<script lang="ts">
	export let allNotes;

	function numToDate(num) {
		let toInt = parseInt(num, 10);
		let date = new Date(toInt);
		let options = {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: 'numeric',
			minute: 'numeric',
		};
		return date.toLocaleString(undefined, options);
	}
</script>

<div id="notes">
	{#if allNotes.length > 0}
	{#each allNotes as note }
		<div id="note">
			<a href="/edit/{note.bson_uuid}">Edit</a>
			<p> {note.title} </p>
			<p> { numToDate(note.date_time.$date.$numberLong) } </p>
		</div>
	{/each}
	{:else}
		<p>Try saving a note</p>
	{/if}
</div>

