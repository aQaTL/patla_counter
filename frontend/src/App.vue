<template>
  <div id="app">
    <div v-if="authenticated" id="main-screen">
      <h1 class="counter">{{ entries.length }}</h1>
      <form v-on:submit.prevent="addOne()" class="add-form">
        <textarea
          cols="40"
          rows="3"
          name="reason"
          id="reason"
          placeholder="Opis wpisu"
          v-model="reason"
        ></textarea>
        <input type="submit" value="+1" />
      </form>
      <div class="entries">
        <div v-for="(entry, idx) in entries" v-bind:key="entry.id" class="entry">
          <div class="idx">{{ entries.length - idx }}.</div>
          <div class="date">
            <small>{{ entry.created }}</small>
          </div>
          <div class="reason">{{ entry.reason }}</div>
        </div>
      </div>
    </div>
    <auth v-else v-on:authenticated="authenticated = load_entries()"></auth>
  </div>
</template>

<script>
import Auth from "./components/Auth.vue";

export default {
  name: "app",
  components: {
    auth: Auth
  },

  data: () => ({
    entries: [],
    authenticated: false,
    reason: ""
  }),

  created: async function() {
    this.authenticated = await this.load_entries();
  },

  methods: {
    load_entries: async function() {
      let rows = await fetch("/api/all", {
        method: "GET",
        credentials: "include",
        headers: {
          "Content-Type": "application/json"
        }
      });

      if (rows.status != 200) {
        return false;
      }

      this.entries = (await rows.json()).map(v => {
        v.created = new Date(v.created + "Z").toLocaleString();
        return v;
      });
      return true;
    },

    addOne: async function() {
      let resp = await fetch("/api/add", {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ reason: this.reason })
      });
      if (resp.status == 201) {
        let new_entry = await resp.json();
        new_entry.created = new Date(new_entry.created + "Z").toLocaleString();
        this.entries.splice(0, 0, new_entry);
        this.reason = "";
      }
    }
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

#main-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.add-form {
  display: flex;
  flex-direction: column;
}

.add-form > #reason {
  border: none;
  border-radius: 4px;
  background-color: #073642;
  color: #eee8d5;
  padding: 1em;
}

.add-form > input[type="submit"] {
  border: none;
  border-radius: 4px;
  background-color: #073642;
  color: #b58900;
  font-size: 14pt;
}

.entry {
  display: grid;
  grid-template-rows: 1fr;
  grid-template-columns: minmax(40px, auto) 1fr;
  margin: 1em;
  justify-items: self-start;
}

.entry > .idx {
  grid-area: 1 / 1 / 3 / 2;
  align-self: center;
  justify-self: end;

  font-weight: bold;
  font-size: 18pt;
  margin-right: 10px;
  padding-right: 3px;
  color: #586e75;
  border-right: 3px solid #586e75;
}

.entry > .date {
  grid-area: 1 / 2 / 2 / 3;
}

.entry > .reason {
  grid-area: 2 / 2 / 3 / 3;
  color: #b58900;
  font-size: 16pt;
  white-space: pre-line;
  text-align: start;
}

.entry:hover {
  background-color: #073642;
}

.counter {
  color: #d33682;
  font-size: 40pt;
}

body {
  margin: 0;
  background-color: #002b36;
}
</style>
