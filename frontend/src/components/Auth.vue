<template>
	<div id="auth">
		<form v-on:submit.prevent="submit()" class="auth-form">
			<label for="password">Hasło</label>
			<input type="password" id="password" v-model="password"/>
			<input type="submit" value="Zaloguj"/>
		</form>
	</div>
</template>

<script>
	export default {
		name: "Auth",

		data: () => ({
			password: ""
		}),

		methods: {
			submit: async function () {
				let resp = await fetch("/api/auth", {
					method: "POST",
					credentials: "include",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify({password: this.password})
				});
				let body = await resp.text();
				if (resp.status === 200 && body === "Ok") {
					this.$emit("authenticated");
				}
			}
		}
	};
</script>

<style scoped>
	#auth {
	}

	.auth-form {
		display: grid;
		grid-template-rows: 30px 50px;
		grid-template-columns: 70px 1fr;
		width: 300px;
		height: 100%;
		grid-gap: 5px;
		align-content: center;
	}

	.auth-form > label {
		grid-area: 1 / 1 / 2 / 2;
		align-self: center;
	}

	.auth-form > input[type="password"] {
		grid-area: 1 / 2 / 2 / 3;

		border: none;
		border-radius: 4px;
		background-color: #073642;
		color: #2AA198;
		padding: 1em;
	}

	.auth-form > input[type="submit"] {
		grid-area: 2 / 1 / 3 / 3;

		border: none;
		border-radius: 4px;
		background-color: #073642;
		color: #B58900;
		font-size: 14pt;
	}
	.auth-form > input[type="submit"]:hover {
		background-color: #586E75;
	}
</style>