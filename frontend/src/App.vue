<template>
	<div id="app">
		<side-bar v-if="showSideBar" v-bind:counters="counters" />
		<router-view v-on:authenticated="auth()" v-on:loadSideBar="loadSideBar()">
		</router-view>
	</div>
</template>

<script>
	import SideBar from "./components/SideBar.vue";

	export default {
		name: "app",
		components: {
			SideBar,
		},

		data: () => ({
			showSideBar: true, //todo fixme
			counters: [],
		}),

		created: async function () {
			if (this.$route.fullPath === "/")
				await this.$router.push("/counter/1");
		},

		methods: {
			auth: async function () {
				this.$router.push('/counter/1');
				this.loadSideBar();
			},

			loadSideBar: async function () {
				let counters_resp = await fetch("/api/counters", {
					method: "GET",
					credentials: "include",
					headers: {
						"Content-Type": "application/json"
					},
				});
				this.counters = await counters_resp.json();
			},
		}
	};
</script>

<style>
	@import url("https://fonts.googleapis.com/css?family=Lato&display=swap");

	#app {
		font-family: "Lato", sans-serif;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		text-align: center;
		color: #839496;
		display: flex;
		justify-content: center;
		height: 100vh;
		overflow: auto;
	}

	body {
		margin: 0;
		background-color: #002B36;
		font-family: "Lato", sans-serif;

	}

	input[type="submit"], button {
		border: none;
		border-radius: 4px;
		background-color: #073642;
		color: #B58900;
	}
</style>
