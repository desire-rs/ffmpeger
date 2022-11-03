<template lang="pug">
.container
  table
    thead
      tr
        //- th id
        th title
        th url
        //- th status
        th createdAt
    tbody
      tr(v-for="task in tasks")
        //- td(v-text="task.id")
        td(v-text="task.title")
        td(v-text="task.url")
        //- td(v-text="task.status")
        td(v-text="task.createdAt")

</template>

<script lang="ts">
import axios from "axios";
axios.defaults.baseURL = window.location.origin;
export default {
  data() {
    return {
      tasks: [],
    };
  },
  mounted() {
    this.init();
  },
  methods: {
    init() {
      setInterval(async () => {
        const result = await axios.get("/task");
        this.tasks = result.data.data.list;
      }, 5000);
    },
  },
};
</script>

<style lang="styl" scoped>
html,body
  margin: 0;
  padding: 0;
.container table
  width: 100%;
  border-collapse: collapse;
.container caption
  height: 30px;
  line-height: 30px;
  font-weight: 700;
.container thead th,
.container tbody td
  padding:8px;
  height: 19px;
  line-height: 19px;
  font-weight: 400;
.container thead th
  text-align: left;
  color:#fff;
  background-color: #353535;
.container tbody tr
  background-color: #fefefe;
  color: #686868;
.container tbody tr:nth-child(even)
  background-color: #f2f2f2;
.row-selected
  width: 15px;
</style>
