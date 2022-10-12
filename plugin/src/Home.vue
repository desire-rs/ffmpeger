<template lang="pug">
.container
  .control
    button.btn(@click="submit") submit
    button.btn(@click="clear") clear
  .content
    .item(v-for="(url, index) in urls")
      input(type="checkbox"  v-model="targets" :value="url")
      label {{url}}
    textarea(cols='30' rows='5' v-model="info")
    textarea(cols='30' rows='4' style='color:red;' v-model="result")
</template>

<script lang="ts">
export default {
  data() {
    return {
      title: "title", // title
      urls: [], // 捕获的urls
      targets: [], // 选中的urls
      info: "{}", // 日志信息
      result: "{}", // 结果信息
    };
  },
  mounted() {
    this.init();
  },
  methods: {
    async init() {
      const { m3u8 } = await chrome.storage.local.get("m3u8");
      console.log(m3u8);
      if (!!m3u8) {
        this.urls = JSON.parse(m3u8);
      }
    },
    async clear() {
      const result = confirm("确定清空内容?");
      if (result) {
        await chrome.storage.local.clear();
      }
    },
    async sendToContent(params) {
      let [tab] = await chrome.tabs.query({
        active: true,
        currentWindow: true,
      });
      params.url = tab.url;
      params.title = tab.title;
      if (tab && tab.id) {
        const port = chrome.tabs.connect(tab.id, {
          name: "ffmpeger",
        });
        port.postMessage(params);
        port.onMessage.addListener((msg) => {
          console.log("response:", msg);
          this.info = JSON.stringify(msg, null, 2);
        });
      }
    },
    async submit() {
      for (const m3u8 of this.targets) {
        const data = { url: m3u8, title: this.title };
        await this.sendToContent(data);
      }
    },
  },
};
</script>

<style lang="styl" scoped>
html,
body
  margin: 0;
  padding: 0;
.container
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
.content
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
body
  margin: 0;
  padding: 10px;
textarea
  width: 700px;
  margin-top: 5px;
  border-radius: 4px;
  padding: 5px;
  outline: none;
label
  color: #000;
  background-color: #ccc;
  margin: 2px 0;
  width: 710px;
button
  cursor: pointer;
  padding: 8px 18px;
  border: none;
  border-radius: 4px;
  outline: none;
  color: white;
  margin: 0 5px;;
  background-color: cornflowerblue;
</style>
