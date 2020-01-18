<template>
	<div id="counter">
		<h1 class="counter">{{ entries.length }}</h1>
		<form v-on:submit.prevent="addOne()" class="add-form">
        <textarea
						cols="40"
						rows="3"
						name="reason"
						id="reason"
						placeholder="Opis wpisu"
						v-model="reason"
				/>
			<input type="submit" value="+1"/>
		</form>
		<div class="entries">
			<div
					v-for="(entry, idx) in entries"
					v-bind:key="entry.id"
					@dblclick="openEntryMenu(entry.id)"
					v-touch:swipe.left="() => openEntryMenu(entry.id)"
			>
				<div v-if="!entry.editMode" class="entry">
					<div class="idx">{{ entries.length - idx }}.</div>
					<div class="date">
						<small>{{ entry.created }}</small>
					</div>
					<div class="reason">{{ entry.reason }}</div>
				</div>
				<div v-else class="entry-edit-mode">
					<div class="idx">{{ entries.length - idx }}.</div>
					<textarea
							rows="6"
							placeholder="Opis wpisu"
							v-model="entry.reason"
					/>
					<button type="button" @click="updateEntry(entry.id)" class="ok-button">Ok</button>
					<button type="button" @click="entry.editMode = false" class="cancel-button">Anuluj
					</button>
					<button type="button" @click="deleteEntry(entry.id)" class="delete-button">Usuń</button>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: "Counter",

		data: function () {
			return {
				entries: [],
				reason: "",
				id: this.$route.params.id,
			};
		},

		computed: {},

		created: async function () {
			this.loadEntries();
		},

		methods: {
			loadEntries: async function () {
				let rows = await fetch("/api/counter/" + this.id, {
					method: "GET",
					credentials: "include",
					headers: {
						"Content-Type": "application/json"
					}
				});

				if (rows.status === 403) {
					await this.$router.push("/auth");
					return;
				}

				if (rows.status === 200) {
					this.$emit("loadSideBar");
					this.entries = (await rows.json()).map(v => {
						v.created = new Date(v.created + "Z").toLocaleString();
						v.editMode = false;
						return v;
					});
				}
			},

			addOne: async function () {
				let resp = await fetch("/api/add", {
					method: "POST",
					credentials: "include",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify({
						counter_id: parseInt(this.id),
						reason: this.reason,
					})
				});
				if (resp.status === 201) {
					let new_entry = await resp.json();
					new_entry.created = new Date(new_entry.created + "Z").toLocaleString();
					new_entry.editMode = false;
					this.entries.splice(0, 0, new_entry);
					this.reason = "";
				}
			},

			updateEntry: async function (id) {
				let entry = this.entries.find(e => e.id === id);
				let resp = await fetch("/api/edit", {
					method: "POST",
					credentials: "include",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify({
						id: parseInt(entry.id),
						reason: entry.reason,
					})
				});
				if (resp.status === 200) {
					entry.editMode = false;
				} else {
					prompt("Failed to update");
				}
			},

			deleteEntry: async function (id) {
				let entryIdx = this.entries.findIndex(e => e.id === id);
				let resp = await fetch("/api/delete", {
					method: "DELETE",
					credentials: "include",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify({
						id: parseInt(id),
					})
				});
				if (resp.status === 200) {
					this.entries.splice(entryIdx, 1);
				} else {
					prompt("Failed to delete");
				}
			},

			openEntryMenu: async function (id) {
				this.entries.find(entry => entry.id === id).editMode = true;
			},
		}
	}
</script>

<style scoped>
	#counter {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-width: 300px;
		max-width: 700px;
	}

	.add-form {
		display: flex;
		flex-direction: column;
	}

	textarea {
		border: none;
		border-radius: 4px;
		background-color: #073642;
		color: #EEE8D5;
		padding: 1em;
	}

	.add-form > input[type="submit"] {
		font-size: 14pt;
	}

	.entry {
		display: grid;
		grid-template-rows: 1fr;
		grid-template-columns: minmax(40px, auto) 1fr;
		margin-top: 1em;
		margin-bottom: 1em;
		justify-items: self-start;
		grid-column-gap: 5px;
		word-break: break-word;
	}

	.entry > .idx, .entry-edit-mode > .idx {
		grid-area: 1 / 1 / 3 / 2;
		align-self: center;
		justify-self: end;

		font-weight: bold;
		font-size: 18pt;
		margin-right: 5px;
		padding-right: 3px;
		color: #586E75;
		border-right: 3px solid #586E75;
	}

	.entry > .date {
		grid-area: 1 / 2 / 2 / 3;
	}

	.entry > .reason {
		grid-area: 2 / 2 / 3 / 3;
		color: #B58900;
		font-size: 16pt;
		white-space: pre-line;
		text-align: start;
	}

	.entry:hover {
		background-color: #073642;
	}

	.entry-edit-mode {
		display: grid;
		grid-template-rows: 1fr 30px;
		grid-template-columns: minmax(40px, auto) 1fr 1fr 1fr;
		margin-top: 1em;
		margin-bottom: 1em;
		justify-items: self-start;
		grid-row-gap: 5px;
		grid-column-gap: 5px;

		border: 1px #073642 solid;
		border-radius: 4px;
	}

	.entry-edit-mode > textarea {
		grid-area: 1 / 2 / 2 / 5;
		justify-self: stretch;
	}

	.entry-edit-mode > button {
		justify-self: stretch;
	}

	.entry-edit-mode > .ok-button {
		grid-area: 2 / 2 / 3 / 3;
	}

	.entry-edit-mode > .ok-button:hover {
		background-color: #859900;
		color: #002B36;
	}

	.entry-edit-mode > .cancel-button {
		grid-area: 2 / 3 / 3 / 4;
	}

	.entry-edit-mode > .cancel-button:hover {
		background-color: #2AA198;
		color: #002B36;
	}

	.entry-edit-mode > .delete-button {
		grid-area: 2 / 4 / 3 / 5;
	}

	.entry-edit-mode > .delete-button:hover {
		background-color: #DC322F;
		color: #002B36;
	}

	.counter {
		color: #D33682;
		font-size: 40pt;
	}
</style>
