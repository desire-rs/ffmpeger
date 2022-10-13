<template lang="pug">
.container
  .control
    button.btn(@click="submit") submit
    button.btn(@click="clear") clear
  .content
    .item(v-for="(url, index) in urls")
      input.checkbox(type="checkbox"  v-model="targets" :value="url")
      input.m3u8(type="text" :value="url")
    textarea.info(v-if="info" v-model="info" style='color:red;')
</template>

<script lang="ts">
interface IMsg {
  m3u8s: string[];
}
export default {
  data() {
    return {
      urls: [], // 捕获的urls
      targets: [], // 选中的urls
      info: null, // 日志信息
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
        this.targets = this.urls;
      }
    },
    async clear() {
      const result = confirm("确定清空内容?");
      if (result) {
        await chrome.storage.local.remove(["m3u8"]);
        this.urls = [];
        this.targets = [];
      }
    },
    async sendToContent(params: IMsg) {
      let [tab] = await chrome.tabs.query({
        active: true,
        currentWindow: true,
      });
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
      await this.sendToContent({ m3u8s: this.targets });
      await chrome.storage.local.remove(["m3u8"]);
      this.urls = [];
      this.targets = [];
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
  width: 710px;
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
  padding 5px;
.control
  height: 60px;
  border-bottom: 1px solid #c6c6c6;
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  algin-items: center;
  margin-bottom: 20px;
.content
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
body
  margin: 0;
  padding: 0;
textarea.info
  width: 700px;
  margin-top: 5px;
  border-radius: 4px;
  padding: 5px;
  outline: none;
  min-height: 300px;
  border: 1px solid #c6c6c6;
button
  cursor: pointer;
  padding: 8px 18px;
  height: 36px;
  border: none;
  border-radius: 4px;
  outline: none;
  color: white;
  margin: 0 5px;;
  background-color: cornflowerblue;
.item
  display: flex;
  flex-direction: row;
  justify-content:flex-start;
  algin-items: center;
input.checkbox
  flex-basis: 16px;
input.m3u8
  margin: 5px 0;
  flex-grow: 1;
  height: 32px;
  border-radius: 5px;
  outline: none;
  padding: 2px 5px;
  border: 1px solid #c6c6c6;
</style>
