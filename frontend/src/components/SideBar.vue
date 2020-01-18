<template>
	<div id="side-bar" v-bind:class="[ show ? 'expanded' : 'collapsed' ]">
		<a id="show-button" @click="toggleSideBar()">
			<svg
					id="menu"
					viewBox="0 0 10 10"
					height="10mm"
					width="10mm">
				<g
						transform="translate(0.04487227,2.5984439)"
						id="layer1">
					<g
							style="fill:#657B83;fill-opacity:1"
							transform="matrix(0.11467355,0,0,0.18833804,-0.04487227,-38.56862)"
							id="layer1-3">
						<rect y="191.16667" x="0" height="11.112499" width="87.3125"/>
						<rect y="212.33333" x="0" height="11.112499" width="87.3125"/>
						<rect y="232.97083" x="0" height="11.112499" width="87.3125"/>
					</g>
				</g>
			</svg>
		</a>
		<div v-if="show" id="side-bar-main">
			<hr>
			<div
					v-for="counter in counters"
					v-bind:key="counter.id"
					class="counter"
					v-bind:class="{ 'counter-current': $route.params.id == counter.id }"
			>
				<router-link :to="'/counter/' + counter.id">
					{{ counter.name }}
				</router-link>
			</div>
			<button type="button" class="add-button" value="">Dodaj licznik</button>
		</div>
		<div v-else></div>
	</div>
</template>

<script>
	export default {
		name: "SideBar",

		data: () => ({
			show: false,
		}),

		props: {
			counters: Array,
		},

		methods: {
			toggleSideBar: async function () {
				this.show = !this.show;
			},
		},
	}
</script>

<style scoped>
	#side-bar {
		position: fixed;

		height: 100vh;

		display: flex;
		flex-direction: column;
	}

	.expanded {
		top: 0;
		left: 0;

		width: 300px;
		min-width: 300px;
		max-width: 300px;
		background-color: #073642;

		transition: 0.10s;
	}

	.collapsed {
		top: 5px;
		left: 5px;

		width: unset;
		min-width: 0;
		max-width: 50px;
		background: none;

		transition: 0.10s;
	}

	#show-button {
		align-self: center;

		text-shadow: none;

		border: none;
		border-radius: 10px;
		background-color: #073642;
		color: #657B83;

		width: 30px;
		height: 30px;
		padding: 10px;
	}

	#menu {
		width: 30px;
		height: 30px;
	}

	#menu rect {
		fill: #657B83;
		fill-opacity: 1;
		stroke-width: 0.15145162;
	}

	#show-button:focus {
		outline: none;
	}

	#show-button:hover rect {
		fill: #00AFAF;
	}

	.add-button {
		margin-top: 20px;
		background-color: #002B36;
		padding: 10px;
	}

	#side-bar-main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	#side-bar-main > hr {
		align-self: stretch;
		margin-left: 0;
		margin-right: 0;
		margin-bottom: 10px;
	}

	.counter {

	}

	.counter-current > a {
		color: #D33682;
	}
</style>