<template>
	<div id="app">
		<router-view v-on:authenticated="$router.push('/counter/1')">
		</router-view>
	</div>
</template>

<script>

	export default {
		name: "app",
		components: {},

		data: () => ({
		}),

		created: async function () {
			if (this.$route.fullPath === "/")
				await this.$router.push("/counter/1");
		},
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
	}
</style>
